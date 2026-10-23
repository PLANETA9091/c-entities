//! L6 — the native ItemEntity tick body.
//!
//! A line-by-line port of Paper 1.21.8:
//!   tickNonPassenger (dispatcher prelude: setOldPosAndRot, tickCount++,
//!                     totalEntityAge++ — mirrored by the module)
//!   ItemEntity.tick  (gravity, noPhysics probe, move gate, move/collide,
//!                     updateEntityMovementAfterFallOn, blockSpeedFactor,
//!                     friction, merge cadence, age++, despawn check)
//!   ItemEntity.inactiveTick (pickupDelay--, age++, despawn check)
//!   Entity.move + Entity.collide + moonrise performCollisions
//!   Entity.checkFallDamage / checkSupportingBlock / findSupportingBlock
//!   Level.findSupportingBlock (Paper min-distance selection)
//!   Entity.getOnPos(0.999999F) + Entity.getBlockSpeedFactor
//!   Entity.applyEffectsFromBlocks -> checkInsideBlocks (branch 1: movement
//!                     < 1 block — always true for native items) with the
//!                     cobweb makeStuckInBlock effect applied natively.
//!
//! ABORT SEMANTICS: every gather / routing check runs BEFORE any state
//! mutation. If a tick cannot be computed exactly (complex shapes, unloaded
//! chunks, fluid, special block overrides, stuck-in-block) the function
//! returns ABORTED with ZERO mutations — the item freezes for one tick and
//! the vanilla body computes it next tick (documented transition; reachable
//! only through rare block changes / exotic terrain).
//!
//! Anything the port cannot compute bit-exactly (fluids, complex shapes,
//! world border, nether void, portals, fire, vehicles) routes to the vanilla
//! Java path: the collector reads fresh Java flags before the tick is
//! suppressed, and the core re-checks physical routing reasons on every
//! flush (TickOutcome::VANILLA_NEXT).

use crate::itemtick::{
    mth_equal, mth_floor, perform_collisions, voxel_shape_intersect, Aabb6, EPS7,
    FRICTION_F32_AS_F64, GATE_HORZ_SQR, ITEM_GRAVITY, MOVE_EPS, Y_DRAG,
};
use crate::sections::SectionCache;

/// Vanilla defaults for air / unloaded blocks (Block friction 0.6F,
/// speedFactor 1.0F).
pub const AIR_FRICTION: f32 = 0.6;
pub const AIR_SPEED: f32 = 1.0;
/// CobwebBlock.entityInside: makeStuckInBlock(state, new Vec3(0.25, 0.05, 0.25)).
pub const COBWEB_STUCK: (f64, f64, f64) = (0.25, 0.05, 0.25);

/// Per-item state owned by Rust between flushes. Synced from the Java
/// entity at registration, at K-tick resyncs, on probe wakes, and whenever
/// the Java body ran vanilla for that item.
#[derive(Clone, Copy, Debug)]
pub struct ItemState {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
    /// Position before this tick's movement (vanilla xo/yo/zo, set by the
    /// dispatcher's setOldPosAndRot).
    pub xo: f64,
    pub yo: f64,
    pub zo: f64,
    pub on_ground: bool,
    pub fall_dist: f32,
    /// Mirror of Java tickCount (dispatcher ++s every tick, active or not).
    pub tick_count: i32,
    pub age: i32,
    pub pickup_delay: i32,
    pub despawn_rate: i32,
    pub has_impulse: bool,
    /// stuckSpeedMultiplier (cobweb), consumed by the next move.
    pub stuck: (f64, f64, f64),
    /// mainSupportingBlockPos (exact, Rust-only; drives getOnPos).
    pub sup_x: i32,
    pub sup_y: i32,
    pub sup_z: i32,
    pub sup_present: bool,
    pub on_ground_no_blocks: bool,
    /// Friction/speed memo: the friction block position it was resolved
    /// for. Invalidated by resyncs (vanilla ticks, probes) and position
    /// changes — NOT by a version check (the block-change probe routes
    /// overlapping items vanilla, whose resync invalidates the memo).
    pub fr_pos: (i32, i32, i32),
    pub friction: f32,
    pub speed: f32,
    /// stepOn-override flag of the memoized friction block.
    pub fr_step_special: bool,
    /// fallDistance BEFORE resetFallDistance on LAND ticks (the Java
    /// fallOn snippet needs it; cleared after the writeback).
    pub land_fall: f32,
    /// frictionState == TriState.FALSE (f = 1.0 exact; else block friction).
    pub friction_disabled: bool,
}

impl ItemState {
    pub fn new() -> ItemState {
        ItemState {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            xo: 0.0,
            yo: 0.0,
            zo: 0.0,
            on_ground: false,
            fall_dist: 0.0,
            tick_count: 0,
            age: 0,
            pickup_delay: 0,
            despawn_rate: 6000,
            has_impulse: false,
            stuck: (0.0, 0.0, 0.0),
            sup_x: 0,
            sup_y: 0,
            sup_z: 0,
            sup_present: false,
            on_ground_no_blocks: false,
            fr_pos: (i32::MIN, i32::MIN, i32::MIN),
            friction: AIR_FRICTION,
            speed: AIR_SPEED,
            fr_step_special: false,
            land_fall: 0.0,
            friction_disabled: false,
        }
    }
}

impl Default for ItemState {
    fn default() -> Self {
        ItemState::new()
    }
}

/// Per-world environment, read once per flush by the module.
#[derive(Clone, Copy, Debug)]
pub struct TickEnv {
    /// WorldUtil.getMinSection(level) / getMaxSection(level) (section idx).
    pub min_section_y: i32,
    pub max_section_y: i32,
    /// world.getWorld().isVoidDamageEnabled().
    pub void_damage: bool,
    /// level.getMinY() + world.getVoidDamageMinBuildHeightOffset().
    pub void_threshold: f64,
    /// World border extents; items within `border_margin` route vanilla.
    pub border_min_x: f64,
    pub border_max_x: f64,
    pub border_min_z: f64,
    pub border_max_z: f64,
    pub border_margin: f64,
}

impl TickEnv {
    pub fn border_close(&self, x: f64, z: f64) -> bool {
        x < self.border_min_x + self.border_margin
            || x > self.border_max_x - self.border_margin
            || z < self.border_min_z + self.border_margin
            || z > self.border_max_z - self.border_margin
    }
}

/// Abort-reason diagnostics (folded into the module's flush log). Statics
/// because the abort decisions live deep inside tick_item; diagnostics only,
/// never read for control flow.
pub static AB_STUCK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static AB_UNLOADED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static AB_FLUID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static AB_COMPLEX: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static AB_FALL: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Outcome flags for one computed tick (bitset; packed straight into the
/// writeback).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TickOutcome(pub u32);

impl TickOutcome {
    pub const MOVED: u32 = 1;
    pub const ON_GROUND: u32 = 1 << 1;
    pub const HAS_IMPULSE: u32 = 1 << 2;
    pub const H_COLL: u32 = 1 << 3;
    pub const V_COLL: u32 = 1 << 4;
    pub const MERGE: u32 = 1 << 5;
    pub const MERGE2: u32 = 1 << 6;
    pub const DESPAWN: u32 = 1 << 7;
    pub const VOID: u32 = 1 << 8;
    pub const VANILLA_NEXT: u32 = 1 << 9;
    pub const ABORTED: u32 = 1 << 10;
    /// Landed with fallDistance > 0 this tick -> Java runs the exact
    /// fallOn + gameEvent + (writeback carries the pre-reset fallDistance).
    pub const LAND: u32 = 1 << 11;

    #[inline]
    pub fn has(self, bit: u32) -> bool {
        self.0 & bit != 0
    }

    #[inline]
    pub fn set(&mut self, bit: u32) {
        self.0 |= bit;
    }
}

/// Scratch buffers reused across items (no per-item allocation).
pub struct Scratch {
    pub boxes: Vec<Aabb6>,
    pub visited: Vec<(i32, i32, i32)>,
}

impl Scratch {
    pub fn new() -> Scratch {
        Scratch {
            boxes: Vec::with_capacity(64),
            visited: Vec::with_capacity(32),
        }
    }
}

impl Default for Scratch {
    fn default() -> Self {
        Scratch::new()
    }
}

/// Pre-flight routing decision on the freshest state (used by the flush to
/// set the mode for the next tick). The Java-side collector flags (fluids,
/// fire, vehicles, portals, empty stack, Paper despawn timers) are checked
/// before the tick is ever suppressed; this covers the geometry reasons.
pub fn want_native(st: &ItemState, env: &TickEnv) -> bool {
    !env.border_close(st.x, st.z)
}

/// The full native tick for one item (active body). All checks run before
/// any mutation: on ABORTED the state is untouched.
#[allow(clippy::too_many_arguments)]
pub fn tick_item(
    st: &mut ItemState,
    id: i64,
    env: &TickEnv,
    sc: &mut SectionCache,
    scratch: &mut Scratch,
) -> TickOutcome {
    let mut out = TickOutcome(0);

    // NOTE: the dispatcher (tickNonPassenger) increments tickCount BEFORE the
    // tick; the module injects the incremented value — no mirror ++ here.

    // ===== read-only phase =====
    let box_now = Aabb6::item_box(st.x, st.y, st.z);

    // Entity.baseTick -> checkBelowWorld (void damage)
    if env.void_damage && st.y < env.void_threshold {
        out.set(TickOutcome::VOID);
    }

    // noPhysics probe: level.noCollision(this, bb.deflate(1e-7)).
    // The probe happens after gravity in vanilla but reads only the box, so
    // hoisting it before the mutation phase is exact.
    let deflated = box_now.deflate(1.0e-7);
    let flags_np = sc.gather(
        &deflated,
        env.min_section_y,
        env.max_section_y,
        &mut scratch.boxes,
    );
    if flags_np.unloaded || flags_np.complex || flags_np.any_fluid {
        out.set(TickOutcome::VANILLA_NEXT);
    }
    let stuck_in_block = scratch
        .boxes
        .iter()
        .any(|b| voxel_shape_intersect(&deflated, b));
    if stuck_in_block {
        AB_STUCK.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        // vanilla would run moveTowardsClosestSpace + a noPhysics move;
        // freeze one tick, the vanilla body unsticks next tick (only
        // reachable right after a block change, which routes vanilla at
        // the probe anyway).
        out.set(TickOutcome::ABORTED | TickOutcome::VANILLA_NEXT);
        return out;
    }

    // move gate (pre-move delta = post-gravity delta in vanilla)
    let dy_g = st.dy - ITEM_GRAVITY;
    let delta_pre_gravity = (st.dx, st.dy, st.dz);
    let mv0 = (st.dx, dy_g, st.dz);
    let horiz_sqr = st.dx * st.dx + st.dz * st.dz;
    let gate = !st.on_ground
        || horiz_sqr > GATE_HORZ_SQR
        || (st.tick_count.wrapping_add(id as i32) % 4) == 0;

    // collide inputs (read-only)
    let mut mv = mv0;
    let mut stuck_applied = false;
    if st.stuck.0 * st.stuck.0 + st.stuck.1 * st.stuck.1 + st.stuck.2 * st.stuck.2 > 1.0e-7 {
        mv = (st.dx * st.stuck.0, dy_g * st.stuck.1, st.dz * st.stuck.2);
        stuck_applied = true;
    }

    let mut collided = mv;
    let mut below_friction: f32 = AIR_FRICTION;
    let mut below_speed: f32 = 1.0;
    let mut below_step_special = false;
    let mut at_speed: f32 = AIR_SPEED;
    let mut at_is_water = false;
    let mut new_pos = (st.x, st.y, st.z);
    let mut inside_special = false;
    let mut cobweb_hit = false;

    if gate {
        let swept = if mv.0 == 0.0 && mv.2 == 0.0 {
            if mv.1 < 0.0 {
                // cutDownwards
                Aabb6::new(
                    box_now.x0,
                    box_now.y0 + mv.1,
                    box_now.z0,
                    box_now.x1,
                    box_now.y0,
                    box_now.z1,
                )
            } else {
                // cutUpwards
                Aabb6::new(
                    box_now.x0,
                    box_now.y1,
                    box_now.z0,
                    box_now.x1,
                    box_now.y1 + mv.1,
                    box_now.z1,
                )
            }
        } else {
            Aabb6::expand_towards(&box_now, mv.0, mv.1, mv.2)
        };
        let swept_flags = sc.gather(
            &swept,
            env.min_section_y,
            env.max_section_y,
            &mut scratch.boxes,
        );
        if swept_flags.unloaded
            || swept_flags.complex
            || swept_flags.any_fluid
            || swept_flags.fall_special
        {
            if swept_flags.unloaded {
                AB_UNLOADED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            if swept_flags.any_fluid {
                AB_FLUID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            if swept_flags.complex {
                AB_COMPLEX.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            if swept_flags.fall_special {
                AB_FALL.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            // exact collide impossible (or a fallOn bounce override sits in
            // the swept region): freeze + vanilla next tick
            out.set(TickOutcome::ABORTED | TickOutcome::VANILLA_NEXT);
            return out;
        }
        collided = perform_collisions(mv, &box_now, &scratch.boxes);
        let d = collided.0 * collided.0 + collided.1 * collided.1 + collided.2 * collided.2;
        let mv_sqr = mv.0 * mv.0 + mv.1 * mv.1 + mv.2 * mv.2;
        if d > MOVE_EPS || mv_sqr - d < MOVE_EPS {
            new_pos = (st.x + collided.0, st.y + collided.1, st.z + collided.2);
            out.set(TickOutcome::MOVED);
        }

        // friction/speed resolution at the NEW position (read-only; memo
        // first — the memo is invalidated by resyncs and position changes)
        let probe = ItemState {
            x: new_pos.0,
            y: new_pos.1,
            z: new_pos.2,
            sup_x: st.sup_x,
            sup_y: st.sup_y,
            sup_z: st.sup_z,
            sup_present: st.sup_present,
            ..*st
        };
        let (bx, by, bz) = get_on_pos_friction(&probe);
        if st.fr_pos == (bx, by, bz) {
            below_friction = st.friction;
            below_speed = st.speed;
            below_step_special = st.fr_step_special;
        } else {
            let (entry, _unloaded) = sc.block_at(bx, by, bz, env.min_section_y, env.max_section_y);
            let below = entry.unwrap_or((
                AIR_FRICTION,
                AIR_SPEED,
                false,
                false,
                false,
                false,
                false,
                false,
            ));
            below_friction = below.0;
            below_speed = below.1;
            below_step_special = below.6;
        }

        let at = (
            mth_floor(new_pos.0),
            mth_floor(new_pos.1),
            mth_floor(new_pos.2),
        );
        let (entry_at, _u) = sc.block_at(at.0, at.1, at.2, env.min_section_y, env.max_section_y);
        let at_e = entry_at.unwrap_or((
            AIR_FRICTION,
            AIR_SPEED,
            false,
            false,
            false,
            false,
            false,
            false,
        ));
        at_speed = if !at_e.2 && at_e.1 == 1.0 {
            below_speed
        } else {
            at_e.1
        };
        at_is_water = at_e.2;

        // checkInsideBlocks path scan (movement < 1 block for native items;
        // the DDA branch is a safety net)
        let mv_len_sqr = mv.0 * mv.0 + mv.1 * mv.1 + mv.2 * mv.2;
        if mv_len_sqr >= (0.99999f32 * 0.99999f32) as f64 {
            out.set(TickOutcome::VANILLA_NEXT);
        } else {
            let (seg_special, seg_cobweb) =
                check_inside_blocks(sc, env, (st.xo, st.yo, st.zo), new_pos, scratch);
            inside_special = seg_special;
            cobweb_hit = seg_cobweb;
        }

        if (st.on_ground || new_on_ground(mv, collided)) && below_step_special {
            // effectful stepOn (magma hurt, honey, ...) -> vanilla computes
            // it; freeze this tick (state untouched)
            out.set(TickOutcome::ABORTED | TickOutcome::VANILLA_NEXT);
            return out;
        }
        if inside_special {
            // effectful entityInside other than cobweb (bush, fire, ...) ->
            // vanilla computes it; freeze this tick
            out.set(TickOutcome::ABORTED | TickOutcome::VANILLA_NEXT);
            return out;
        }
    }

    // ===== commit phase (all checks passed) =====
    st.xo = st.x;
    st.yo = st.y;
    st.zo = st.z;
    // the delta post-tick carries gravity even when the gate stayed closed
    st.dx = mv0.0;
    st.dy = dy_g;
    st.dz = mv0.2;

    if stuck_applied {
        st.stuck = (0.0, 0.0, 0.0);
        st.dx = 0.0;
        st.dy = 0.0;
        st.dz = 0.0; // setDeltaMovement(Vec3.ZERO) — before the friction step
    }

    let flag_x = !mth_equal(mv.0, collided.0);
    let flag_z = !mth_equal(mv.2, collided.2);
    let vertical = mv.1 != collided.1;
    let vertical_below = vertical && mv.1 < 0.0;

    if gate {
        st.x = new_pos.0;
        st.y = new_pos.1;
        st.z = new_pos.2;
        if flag_x || flag_z {
            out.set(TickOutcome::H_COLL);
        }
        if vertical {
            out.set(TickOutcome::V_COLL);
        }
        st.on_ground = vertical_below;
        if vertical_below {
            out.set(TickOutcome::ON_GROUND);
        }
        if flag_x {
            st.dx = 0.0;
        }
        if flag_z {
            st.dz = 0.0;
        }
        if vertical {
            st.dy = 0.0; // updateEntityMovementAfterFallOn (default: zero Y)
        }
        // checkFallDamage(collided.y, onGround, ...): fallDistance tracking
        // (dry items only; the (float) cast is part of the vanilla code)
        if collided.1 < 0.0 {
            st.fall_dist -= collided.1 as f32;
        }
        if st.on_ground {
            if st.fall_dist > 0.0 {
                out.set(TickOutcome::LAND); // fallOn + gameEvent in Java
                st.land_fall = st.fall_dist; // pre-reset value for the writeback
            }
            st.fall_dist = 0.0; // resetFallDistance
        }
        // getBlockSpeedFactor at the new position
        if !at_is_water && at_speed != 1.0 {
            st.dx *= at_speed as f64;
            st.dz *= at_speed as f64;
        }
        // friction (f32 multiply in Java: blockFriction * 0.98F)
        let mut f = FRICTION_F32_AS_F64;
        if st.friction_disabled {
            f = 1.0;
        } else if st.on_ground {
            f = (below_friction * 0.98f32) as f64;
        }
        st.dx *= f;
        st.dy *= Y_DRAG;
        st.dz *= f;
        if st.on_ground && st.dy < 0.0 {
            st.dy *= -0.5; // vanilla's -0.5 branch (dead after the Y zeroing;
                           // kept faithful)
        }
        // memo update
        let (bx, by, bz) = get_on_pos_friction(&ItemState {
            x: st.x,
            y: st.y,
            z: st.z,
            sup_x: st.sup_x,
            sup_y: st.sup_y,
            sup_z: st.sup_z,
            sup_present: st.sup_present,
            ..*st
        });
        st.fr_pos = (bx, by, bz);
        st.friction = below_friction;
        st.speed = below_speed;
        st.fr_step_special = below_step_special;
        // supporting block refresh (post-move box)
        let new_box = Aabb6::item_box(st.x, st.y, st.z);
        check_supporting_block(sc, env, st, &new_box, collided, scratch);
        // cobweb effect (buffered in vanilla, applied at the end — same final
        // state for a single effect)
        if cobweb_hit {
            st.stuck = COBWEB_STUCK;
            st.fall_dist = 0.0; // makeStuckInBlock -> resetFallDistance
        }
    }

    // merge cadence
    let floor_changed = mth_floor(st.xo) != mth_floor(st.x)
        || mth_floor(st.yo) != mth_floor(st.y)
        || mth_floor(st.zo) != mth_floor(st.z);
    let cadence: i32 = if floor_changed { 2 } else { 40 };
    if st.tick_count % cadence == 0 {
        out.set(TickOutcome::MERGE);
        if floor_changed {
            out.set(TickOutcome::MERGE2);
        }
    }

    // age++
    if st.age != -32768 {
        st.age = st.age.wrapping_add(1);
    }

    // hasImpulse (fluid pushing never applies for dry native items)
    let ddx = st.dx - delta_pre_gravity.0;
    let ddy = st.dy - delta_pre_gravity.1;
    let ddz = st.dz - delta_pre_gravity.2;
    if ddx * ddx + ddy * ddy + ddz * ddz > 0.01 {
        out.set(TickOutcome::HAS_IMPULSE);
    }

    // despawn
    if st.age >= st.despawn_rate {
        out.set(TickOutcome::DESPAWN);
    }

    // ON_GROUND is an ABSOLUTE bit (the final onGround state) so the
    // writeback can apply it directly.
    if st.on_ground {
        out.set(TickOutcome::ON_GROUND);
    }

    out
}

/// The inactive tick body (vanilla inactiveTick: pickupDelay--, age++,
/// despawn check; tickCount mirrored from the dispatcher). No physics.
pub fn tick_item_inactive(st: &mut ItemState) -> TickOutcome {
    let mut out = TickOutcome(0);
    if st.pickup_delay > 0 && st.pickup_delay != 32767 {
        st.pickup_delay -= 1;
    }
    if st.age != -32768 {
        st.age = st.age.wrapping_add(1);
    }
    if st.age >= st.despawn_rate {
        out.set(TickOutcome::DESPAWN);
    }
    out
}

#[inline]
fn new_on_ground(mv: (f64, f64, f64), collided: (f64, f64, f64)) -> bool {
    mv.1 != collided.1 && mv.1 < 0.0
}

/// The per-axis walk of `checkInsideBlocks` (movement < 1 block): segments in
/// axisStepOrder, the box at each segment end deflated by 1e-5, blocks in
/// floor ranges, shared visited set, air skipped. Returns (special, cobweb).
fn check_inside_blocks(
    sc: &mut SectionCache,
    env: &TickEnv,
    from: (f64, f64, f64),
    to: (f64, f64, f64),
    scratch: &mut Scratch,
) -> (bool, bool) {
    let dm = (to.0 - from.0, to.1 - from.1, to.2 - from.2);
    if dm.0 * dm.0 + dm.1 * dm.1 + dm.2 * dm.2 <= 0.0 {
        return (false, false);
    }
    let segments: [(u8, f64); 3] = if dm.0.abs() < dm.2.abs() {
        [(1, dm.1), (2, dm.2), (0, dm.0)]
    } else {
        [(1, dm.1), (0, dm.0), (2, dm.2)]
    };
    let mut cursor = from;
    let mut special = false;
    let mut cobweb = false;
    scratch.visited.clear();
    for (axis, d) in segments {
        if d == 0.0 {
            continue;
        }
        match axis {
            0 => cursor.0 += d,
            1 => cursor.1 += d,
            _ => cursor.2 += d,
        }
        let seg_box = Aabb6::item_box(cursor.0, cursor.1, cursor.2).deflate(1.0e-5);
        let (min_x, max_x) = (mth_floor(seg_box.x0), mth_floor(seg_box.x1));
        let (min_y, max_y) = (mth_floor(seg_box.y0), mth_floor(seg_box.y1));
        let (min_z, max_z) = (mth_floor(seg_box.z0), mth_floor(seg_box.z1));
        for bx in min_x..=max_x {
            for by in min_y..=max_y {
                for bz in min_z..=max_z {
                    if scratch.visited.contains(&(bx, by, bz)) {
                        continue;
                    }
                    scratch.visited.push((bx, by, bz));
                    let (entry, unloaded) =
                        sc.block_at(bx, by, bz, env.min_section_y, env.max_section_y);
                    // (fr, speed, water, fluid, cobweb, inside, step, fall)
                    // air (or unloaded -> vanilla reads air) yields None
                    if let Some(e) = (match (entry, unloaded) {
                        (Some(e), false) => Some(e),
                        _ => None,
                    }) {
                        if e.4 {
                            cobweb = true;
                        } else if e.5 {
                            special = true;
                        }
                    }
                }
            }
        }
    }
    (special, cobweb)
}

/// `Level.findSupportingBlock` (Paper): min distToCenterSqr, ties by
/// BlockPos compareTo (Y, then Z, then X), over the section cache.
fn find_supporting_block(
    sc: &mut SectionCache,
    env: &TickEnv,
    box_: &Aabb6,
    ex: f64,
    ey: f64,
    ez: f64,
) -> Option<(i32, i32, i32)> {
    let min_block_x = mth_floor(box_.x0 - EPS7) - 1;
    let max_block_x = mth_floor(box_.x1 + EPS7) + 1;
    let min_block_y = ((env.min_section_y << 4) - 1).max(mth_floor(box_.y0 - EPS7) - 1);
    let max_block_y = ((env.max_section_y << 4) + 16).min(mth_floor(box_.y1 + EPS7) + 1);
    let min_block_z = mth_floor(box_.z0 - EPS7) - 1;
    let max_block_z = mth_floor(box_.z1 + EPS7) + 1;
    if min_block_y > max_block_y {
        return None;
    }
    let mut selected: Option<(i32, i32, i32)> = None;
    let mut selected_distance = f64::MAX;
    for cy in min_block_y..=max_block_y {
        for cz in min_block_z..=max_block_z {
            for cx in min_block_x..=max_block_x {
                let (rel, unloaded) =
                    sc.block_box_at(cx, cy, cz, env.min_section_y, env.max_section_y);
                let rel = match (rel, unloaded) {
                    (Some(r), false) => r,
                    _ => continue,
                };
                let shifted = Aabb6::new(
                    box_.x0 - cx as f64,
                    box_.y0 - cy as f64,
                    box_.z0 - cz as f64,
                    box_.x1 - cx as f64,
                    box_.y1 - cy as f64,
                    box_.z1 - cz as f64,
                );
                if !voxel_shape_intersect(&rel, &shifted) {
                    continue;
                }
                let ddx = cx as f64 + 0.5 - ex;
                let ddy = cy as f64 + 0.5 - ey;
                let ddz = cz as f64 + 0.5 - ez;
                let distance = ddx * ddx + ddy * ddy + ddz * ddz;
                let take = distance < selected_distance
                    || (distance == selected_distance
                        && match selected.map(|s| (s.1, s.2, s.0)) {
                            None => true,
                            Some(a) => a < (cy, cz, cx),
                        });
                if take {
                    selected = Some((cx, cy, cz));
                    selected_distance = distance;
                }
            }
        }
    }
    selected
}

/// `Entity.checkSupportingBlock(onGround, movement)`.
fn check_supporting_block(
    sc: &mut SectionCache,
    env: &TickEnv,
    st: &mut ItemState,
    box_: &Aabb6,
    movement: (f64, f64, f64),
    _scratch: &mut Scratch,
) {
    if st.on_ground {
        let slab = Aabb6::new(
            box_.x0,
            box_.y0 - 1.0e-6,
            box_.z0,
            box_.x1,
            box_.y0,
            box_.z1,
        );
        let mut final_found = find_supporting_block(sc, env, &slab, st.x, st.y, st.z);
        if final_found.is_none() && !st.on_ground_no_blocks {
            let slab2 = Aabb6::new(
                slab.x0 - movement.0,
                slab.y0,
                slab.z0 - movement.2,
                slab.x1 - movement.0,
                slab.y1,
                slab.z1 - movement.2,
            );
            final_found = find_supporting_block(sc, env, &slab2, st.x, st.y, st.z);
        }
        match final_found {
            Some((sx, sy, sz)) => {
                st.sup_x = sx;
                st.sup_y = sy;
                st.sup_z = sz;
                st.sup_present = true;
            }
            None => st.sup_present = false,
        }
        st.on_ground_no_blocks = final_found.is_none();
    } else {
        st.on_ground_no_blocks = false;
        st.sup_present = false;
    }
}

/// `Entity.getOnPos(0.999999F)` — the friction block position.
fn get_on_pos_friction(st: &ItemState) -> (i32, i32, i32) {
    let floor_y = mth_floor(st.y - 0.999_999f32 as f64);
    if st.sup_present {
        // FENCE/WALL/FENCEGATE supporting blocks are multi-cuboid (complex
        // sections) and route vanilla, so the plain branch is exact here.
        (st.sup_x, floor_y, st.sup_z)
    } else {
        (mth_floor(st.x), floor_y, mth_floor(st.z))
    }
}
