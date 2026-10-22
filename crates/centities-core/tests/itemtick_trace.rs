//! L6 trace tests: hand-computed bit-exact sequences from the vanilla
//! ItemEntity.tick + Entity.move semantics (decompiled Paper 1.21.8).
//!
//! The resting-item trace is the load-bearing one: a resting item's delta
//! oscillates (gravity on every tick, the move gate every 4th tick zeroes
//! it via collide + updateEntityMovementAfterFallOn) while the POSITION
//! freezes exactly — reproduced here to the last bit.

use centities_core::itemtick::Aabb6;
use centities_core::sections::{PaletteEntry, SectionData, SectionGrid, AIR_IDX};
use centities_core::tick::{tick_item, tick_item_inactive, ItemState, Scratch, TickEnv, TickOutcome};

const MIN_SEC: i32 = 0;
const MAX_SEC: i32 = 23;

fn stone_entry() -> PaletteEntry {
    PaletteEntry {
        collider: Some(Aabb6::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0)),
        friction: 0.6,
        speed: 1.0,
        is_water: false,
        has_fluid: false,
        cobweb: false,
        entity_inside_special: false,
        step_on_special: false,
        fall_on_special: false,
    }
}

fn cobweb_entry() -> PaletteEntry {
    // WebBlock: EMPTY collision shape (entities sit inside it); the
    // slow-down comes from entityInside -> makeStuckInBlock.
    PaletteEntry {
        collider: None,
        friction: 0.6,
        speed: 1.0,
        is_water: false,
        has_fluid: false,
        cobweb: true,
        entity_inside_special: false,
        step_on_special: false,
        fall_on_special: false,
    }
}

fn air_entry() -> PaletteEntry {
    PaletteEntry::air()
}

/// Flat world: chunk (0,0) only; ground = one full-cube layer at y=63
/// (section 3), air everywhere above; chunks beyond (0,0) are unloaded.
fn flat_world(cobweb_at: Option<(i32, i32, i32)>) -> impl FnMut(i32, i32, i32, bool) -> Result<SectionData, ()> {
    move |cx, cy, cz, _load| {
        if cx != 0 || cz != 0 {
            return Err(()); // unloaded
        }
        if cy < MIN_SEC || cy > MAX_SEC {
            return Err(());
        }
        if cy == 3 {
            // ground section: all stone
            let mut data = SectionData::all_air();
            data.entries = vec![air_entry(), stone_entry()];
            data.blocks = vec![1u16; 4096];
            return Ok(data);
        }
        let mut data = SectionData::all_air();
        if let Some((wx, wy, wz)) = cobweb_at {
            if wx >> 4 == cx && wz >> 4 == cz && (wy >> 4) == cy {
                data.entries = vec![air_entry(), cobweb_entry()];
                let local = ((wx & 15) | ((wz & 15) << 4) | ((wy & 15) << 8)) as usize;
                data.blocks[local] = 1;
            }
        }
        Ok(data)
    }
}

fn flat_env() -> TickEnv {
    TickEnv {
        min_section_y: MIN_SEC,
        max_section_y: MAX_SEC,
        void_damage: false,
        void_threshold: -64.0,
        border_min_x: -29_999_984.0,
        border_max_x: 29_999_984.0,
        border_min_z: -29_999_984.0,
        border_max_z: 29_999_984.0,
        border_margin: 2.0,
    }
}

fn resting_item() -> ItemState {
    // On flat ground (ground top y=64.0): x/z centered in the block, y=64.
    ItemState {
        x: 0.5,
        y: 64.0,
        z: 0.5,
        dx: 0.0,
        dy: 0.0,
        dz: 0.0,
        on_ground: true,
        despawn_rate: 6000,
        ..ItemState::new()
    }
}

#[test]
fn resting_item_position_freezes_and_delta_oscillates() {
    let env = flat_env();
    let mut sg = SectionGrid::new();
    let mut prov = flat_world(None);
    let mut scratch = Scratch::new();
    let mut st = resting_item();

    // Java float friction for the stone ground: (0.6f32 * 0.98f32) as f64.
    let ground_friction = (0.6f32 * 0.98f32) as f64;

    for tick in 1..=12i32 {
        st.tick_count = tick; // the dispatcher ++ed before the guard fires (post-increment)
        let before = (st.x, st.y, st.z);
        let out = tick_item(&mut st, 0, &env, &mut sg, &mut scratch, &mut prov);
        // position NEVER changes for a resting item
        assert_eq!((st.x, st.y, st.z), before, "tick {tick}: resting position moved");
        assert!(!out.has(TickOutcome::MOVED), "tick {tick}");
        assert!(out.has(TickOutcome::ON_GROUND), "tick {tick}");
        assert_eq!(st.age, tick);
        if tick % 4 == 0 {
            // gate tick: gravity accumulated 3 ticks then the move zeroes dy
            assert_eq!(st.dy, 0.0, "tick {tick}");
            assert!(out.has(TickOutcome::V_COLL), "tick {tick}");
            assert!(out.has(TickOutcome::HAS_IMPULSE), "tick {tick}: |ddelta|^2 > 0.01");
        } else {
            let k = tick % 4; // gravity ticks since the last zeroing
            assert_eq!(st.dy, -(k as f64) * 0.04, "tick {tick}");
            assert!(!out.has(TickOutcome::HAS_IMPULSE), "tick {tick}");
        }
        // delta x/z stay exactly zero (friction multiplies by ground friction
        // on gate ticks; 0 * f == 0)
        assert_eq!(st.dx, 0.0);
        assert_eq!(st.dz, 0.0);
        let _ = ground_friction;
        // merge cadence 40 (floor never changes): fires at tickCount % 40 == 0
        assert_eq!(out.has(TickOutcome::MERGE), tick % 40 == 0, "tick {tick}");
    }
}

#[test]
fn sliding_item_decays_and_merges_on_floor_change() {
    let env = flat_env();
    let mut sg = SectionGrid::new();
    let mut prov = flat_world(None);
    let mut scratch = Scratch::new();
    let mut st = ItemState {
        x: 0.4,
        y: 64.0,
        z: 0.5,
        dx: 0.5,
        on_ground: true,
        despawn_rate: 6000,
        ..ItemState::new()
    };

    // tick 1: x 0.4 -> 0.9 (floor 0 -> 0, no change; no merge: 1 % 40 != 0)
    st.tick_count = 1;
    let out = tick_item(&mut st, 0, &env, &mut sg, &mut scratch, &mut prov);
    assert!(out.has(TickOutcome::MOVED));
    assert_eq!(st.x, 0.9);
    assert_eq!(st.dx, 0.5 * (0.6f32 * 0.98f32) as f64, "friction 0.588f32");
    assert_eq!(st.dy, 0.0, "collide clamps dy to 0, fallOn zeroes it");
    assert!(!out.has(TickOutcome::MERGE));
    // tick 2: x 0.9 -> 1.194: floor 0 -> 1 CHANGED -> cadence 2, tickCount 2
    st.tick_count = 2;
    let out = tick_item(&mut st, 0, &env, &mut sg, &mut scratch, &mut prov);
    assert!(out.has(TickOutcome::MOVED));
    assert!(out.has(TickOutcome::MERGE), "floor changed -> cadence 2, tick 2 fires");
    assert!(out.has(TickOutcome::MERGE2));
    let f = (0.6f32 * 0.98f32) as f64;
    assert_eq!(st.x, 0.9 + 0.5 * f, "tick 2 moves by tick 1's post-friction delta");
    assert_eq!(st.dx, 0.5 * f * f, "friction applies after the move");
}

#[test]
fn falling_item_lands_exactly_on_ground_with_land_event() {
    let env = flat_env();
    let mut sg = SectionGrid::new();
    let mut prov = flat_world(None);
    let mut scratch = Scratch::new();
    // Falling item: on_ground=false, dy = -0.6 from a previous vanilla tick.
    let mut st = ItemState {
        x: 0.5,
        y: 65.0,
        z: 0.5,
        dy: -0.6,
        on_ground: false,
        despawn_rate: 6000,
        ..ItemState::new()
    };

    // tick 1: gate open (!onGround); dy_g = -0.64; no hit yet (64.36 > 64).
    st.tick_count = 1;
    let out = tick_item(&mut st, 0, &env, &mut sg, &mut scratch, &mut prov);
    assert!(out.has(TickOutcome::MOVED));
    assert_eq!(st.y, 64.36);
    assert_eq!(st.dy, -0.6272, "air Y drag is the double literal 0.98 (not f32)");
    assert!(!out.has(TickOutcome::ON_GROUND));
    assert!((st.fall_dist - 0.64f32).abs() < 1e-9, "fallDistance += 0.64 (float)");

    // tick 2: dy_g = -0.6672 - eps; collide clamps to the ground: y = 64.0.
    st.tick_count = 2;
    let out = tick_item(&mut st, 0, &env, &mut sg, &mut scratch, &mut prov);
    assert!(out.has(TickOutcome::MOVED));
    assert_eq!(st.y, 64.0, "64.36 - 0.36 == 64.0 exactly");
    assert!(out.has(TickOutcome::ON_GROUND));
    assert!(out.has(TickOutcome::LAND), "landed with fallDistance > 0");
    assert_eq!(st.fall_dist, 0.0, "resetFallDistance");
    assert!(st.land_fall > 0.0, "pre-reset fallDistance preserved for the Java fallOn");
    assert_eq!(st.dy, 0.0);
}

#[test]
fn gate_phase_follows_tick_count_plus_id() {
    let env = flat_env();
    let mut sg = SectionGrid::new();
    let mut prov = flat_world(None);
    let mut scratch = Scratch::new();
    // id = 1: gate fires when (tickCount + 1) % 4 == 0 -> ticks 3, 7, ...
    let mut st = resting_item();
    st.tick_count = 0;
    for tick in 1..=8i32 {
        st.tick_count = tick;
        let out = tick_item(&mut st, 1, &env, &mut sg, &mut scratch, &mut prov);
        let gate_tick = (tick + 1) % 4 == 0;
        assert_eq!(out.has(TickOutcome::V_COLL), gate_tick, "tick {tick}");
        let k = if tick < 3 { tick } else { (tick + 1) % 4 }; // gravity ticks since the last zeroing
        assert_eq!(st.dy, if gate_tick { 0.0 } else { -(k as f64) * 0.04 }, "tick {tick}");
    }
}

#[test]
fn inactive_tick_mirrors_vanilla() {
    let mut st = ItemState {
        pickup_delay: 10,
        age: 5990,
        despawn_rate: 6000,
        ..ItemState::new()
    };
    let out = tick_item_inactive(&mut st);
    assert_eq!(st.pickup_delay, 9);
    assert_eq!(st.age, 5991);
    assert!(!out.has(TickOutcome::DESPAWN));
    // position and delta untouched
    assert_eq!((st.x, st.dy), (0.0, 0.0));
    for k in 0..8 {
        st.tick_count = 1 + k;
        tick_item_inactive(&mut st);
    }
    // age now 5999; the next tick hits despawnRate
    st.tick_count = 9;
    let out = tick_item_inactive(&mut st);
    assert_eq!(st.age, 6000);
    assert!(out.has(TickOutcome::DESPAWN));
    // 32767 pickup delay never decays
    let mut inf = ItemState { pickup_delay: 32767, ..ItemState::new() };
    tick_item_inactive(&mut inf);
    assert_eq!(inf.pickup_delay, 32767);
}

#[test]
fn cobweb_sets_stuck_multiplier_natively() {
    let env = flat_env();
    let mut sg = SectionGrid::new();
    let mut prov = flat_world(Some((0, 64, 0))); // cobweb at the item's block
    let mut scratch = Scratch::new();
    // Item inside the cobweb block (box overlaps block (0,64,0)).
    let mut st = ItemState {
        x: 0.5,
        y: 64.2,
        z: 0.5,
        dy: -0.1,
        on_ground: false,
        despawn_rate: 6000,
        ..ItemState::new()
    };
    st.tick_count = 1;
    let out = tick_item(&mut st, 0, &env, &mut sg, &mut scratch, &mut prov);
    assert!(!out.has(TickOutcome::ABORTED));
    assert_eq!(st.stuck, (0.25, 0.05, 0.25), "makeStuckInBlock(0.25, 0.05, 0.25)");
    assert_eq!(st.fall_dist, 0.0, "resetFallDistance");
    assert_eq!(st.y, 64.06, "tick 1 moved by dy_g = -0.14 (web has no collision)");
    // next tick: the move scales by the stuck multiplier, delta zeroes
    st.tick_count = 2;
    let out = tick_item(&mut st, 0, &env, &mut sg, &mut scratch, &mut prov);
    // vanilla: move() consumed the multiplier (movement was stuck-scaled,
    // delta zeroed), then entityInside RE-APPLIED makeStuckInBlock — the
    // field is (0.25, 0.05, 0.25) after every tick inside the web.
    assert_eq!(st.stuck, (0.25, 0.05, 0.25), "re-applied every tick inside the web");
    assert_eq!(st.dy, 0.0, "setDeltaMovement(Vec3.ZERO) inside move");
    // dy after tick 1: -0.14 * 0.98 (air drag double); tick 2: dy_g = that - 0.04,
    // movement scaled by stuck.y = 0.05
    let dy1 = -0.14 * 0.98;
    let expected = 64.06 + (dy1 - 0.04) * 0.05;
    assert_eq!(st.y, expected, "stuck-scaled movement applied: {} vs {expected}", st.y);
}

#[test]
fn collide_matches_bruteforce_reference() {
    // Independent reference: sequential per-axis clamp over all boxes with
    // the same epsilon semantics, written differently from the core.
    fn ref_axis(t: &Aabb6, src: &Aabb6, mv: f64, axis: usize) -> f64 {
        // (moved-axis src lo/hi, moved-axis target lo/hi,
        //  perp1 src lo/hi, perp1 target lo/hi,
        //  perp2 src lo/hi, perp2 target lo/hi)
        let (s_lo, s_hi, m_lo, m_hi, o1_lo, o1_hi, o1t_lo, o1t_hi, o2_lo, o2_hi, o2t_lo, o2t_hi) = match axis {
            0 => (src.x0, src.x1, t.x0, t.x1, src.y0, src.y1, t.y0, t.y1, src.z0, src.z1, t.z0, t.z1),
            1 => (src.y0, src.y1, t.y0, t.y1, src.x0, src.x1, t.x0, t.x1, src.z0, src.z1, t.z0, t.z1),
            _ => (src.z0, src.z1, t.z0, t.z1, src.x0, src.x1, t.x0, t.x1, src.y0, src.y1, t.y0, t.y1),
        };
        let eps = 1.0e-7;
        let perp = (o1_lo - o1t_hi < -eps && o1_hi - o1t_lo > eps) && (o2_lo - o2t_hi < -eps && o2_hi - o2t_lo > eps);
        if !perp {
            return mv;
        }
        if mv >= 0.0 {
            let max_move = m_lo - s_hi;
            if max_move < -eps {
                mv
            } else {
                max_move.min(mv)
            }
        } else {
            let max_move = m_hi - s_lo;
            if max_move > eps {
                mv
            } else {
                max_move.max(mv)
            }
        }
    }

    fn ref_collide(mv: (f64, f64, f64), box_: &Aabb6, boxes: &[Aabb6]) -> (f64, f64, f64) {
        let mut x = mv.0;
        let mut y = mv.1;
        let mut z = mv.2;
        let mut b = *box_;
        if y != 0.0 {
            for t in boxes {
                y = ref_axis(t, &b, y, 1);
            }
            if y != 0.0 {
                b.y0 += y;
                b.y1 += y;
            }
        }
        let x_smaller = x.abs() < z.abs();
        if x_smaller && z != 0.0 {
            for t in boxes {
                z = ref_axis(t, &b, z, 2);
            }
            if z != 0.0 {
                b.z0 += z;
                b.z1 += z;
            }
        }
        if x != 0.0 {
            for t in boxes {
                x = ref_axis(t, &b, x, 0);
            }
            if !x_smaller && x != 0.0 {
                b.x0 += x;
                b.x1 += x;
            }
        }
        if !x_smaller && z != 0.0 {
            for t in boxes {
                z = ref_axis(t, &b, z, 2);
            }
        }
        (x, y, z)
    }

    // Deterministic pseudo-random boxes and movements.
    let mut seed: u64 = 0x9E3779B97F4A7C15;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    for case in 0..20000usize {
        let mut boxes = Vec::new();
        let n = (next() % 6) as usize + 1;
        for _ in 0..n {
            let bx = (next() % 8) as f64 - 4.0;
            let by = (next() % 8) as f64 - 4.0;
            let bz = (next() % 8) as f64 - 4.0;
            boxes.push(Aabb6::new(bx, by, bz, bx + 1.0, by + 1.0, bz + 1.0));
        }
        let px = (next() % 800) as f64 / 100.0 - 4.0;
        let py = (next() % 800) as f64 / 100.0 - 4.0;
        let pz = (next() % 800) as f64 / 100.0 - 4.0;
        let b = Aabb6::item_box(px, py, pz);
        let mv = (
            (next() % 200) as f64 / 100.0 - 1.0,
            (next() % 200) as f64 / 100.0 - 1.0,
            (next() % 200) as f64 / 100.0 - 1.0,
        );
        let got = centities_core::itemtick::perform_collisions(mv, &b, &boxes);
        let want = ref_collide(mv, &b, &boxes);
        assert_eq!(got, want, "case {case}: collide mismatch (mv={mv:?}, box={b:?}, boxes={boxes:?})");
    }
}

#[test]
fn gather_finds_ground_boxes_exactly() {
    let mut sg = SectionGrid::new();
    let mut prov = flat_world(None);
    let mut scratch = Scratch::new();
    let env = flat_env();
    // The resting item's swept box (pure-Y cut downwards).
    let b = Aabb6::item_box(0.5, 64.0, 0.5);
    let swept = Aabb6::new(b.x0, b.y0 - 0.16, b.z0, b.x1, b.y0, b.z1);
    let flags = sg.gather(&swept, MIN_SEC, MAX_SEC, &mut prov, &mut scratch.boxes);
    assert!(!flags.unloaded && !flags.complex && !flags.any_fluid);
    // exactly the ground blocks under the 0.5-wide box: the box spans
    // x 0.375..0.625, z 0.375..0.625 -> with the ±1e-7 slop and ±1 expansion
    // and sectionAdjust trim: only block (0,63,0) qualifies.
    assert_eq!(scratch.boxes.len(), 1, "ground boxes: {:?}", scratch.boxes);
    assert_eq!(scratch.boxes[0], Aabb6::new(0.0, 63.0, 0.0, 1.0, 64.0, 1.0));

    // A box over the void (chunk 1,0 unloaded) reports unloaded.
    let b2 = Aabb6::item_box(20.0, 64.0, 0.5);
    let flags2 = sg.gather(&b2, MIN_SEC, MAX_SEC, &mut prov, &mut scratch.boxes);
    assert!(flags2.unloaded, "chunk (1,0) absent");
    assert!(scratch.boxes.is_empty());

    // AIR_IDX consistency: the air palette entry is never referenced via
    // index 1 in the flat world (blocks all 1).
    let _ = AIR_IDX;
}
