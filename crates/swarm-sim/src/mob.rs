//! swarm-sim::mob — the NATIVE T0 tick executor.
//!
//! The hot ring's entities must run the FULL vanilla tick. The
//! parity-proven decision layer is `centities_core::goals::ai_floor` —
//! a coroutine that computes the goal-selector walk natively and parks
//! on requests the world must resolve (the production executor is the
//! Java side, CeHooks.aiRun). This module is the pure-Rust executor of
//! the SAME protocol: it owns the per-mob state the Java executor owns
//! (goal records with their live state words, the RNG mirror, nav
//! state), packs the exact aiPack wire format, drives the resume loop,
//! applies the emitted actions and advances the entity through the
//! parity-proven travel slice (mobtravel) and push kernel (pushcore).
//!
//! Honesty ledger (what is engine-exact vs scripted here):
//! - EXACT: the selector rounds, every classified canUse/canContinue
//!   gate and their RNG draws (bit-identical to Paper 1.21.8 — PARITY
//!   run #369), the seed handoff across rounds, the action semantics
//!   (locks, displaced stops, look ticks, ref writes), the travel
//!   slice math, the push kernel, the move/setPos condition.
//! - SCRIPTED (world queries Java resolves in production; they change
//!   WHERE a mob walks, never the per-tick cost structure):
//!   1. REQ_STROLL_POS target value (vanilla: LandRandomPos 10x7 +
//!      block-safety checks + its own draws) — the bench draws the
//!      target from a separate splitmix stream, so the mob RNG stream
//!      is NOT advanced by the body draws of a stroll start.
//!   2. The stroll path model: straight line to the target (the
//!      open-ground routing proof makes the swept hull clip-free; the
//!      Java path node list is not modeled), arrival at d < 0.5.
//!   3. The RandomLookAround start body: nextDouble + nextInt(20) are
//!      consumed EXACTLY (javap 26.3 draw pattern — the seed stays
//!      vanilla-aligned), lookTime = 20 + the drawn int, rel = 2pi*d.
//!   4. MeleeAttack (ZombieAttackGoal) continue/approach: while the
//!      target is alive and beyond attack range the executor walks
//!      toward it (the open-ground straight line is the proven path).
//!   5. noActionTime: +1 per tick, reset on any goal start.

use centities_core::goals as g;
use centities_core::mobtravel::{self, Mth};

// re-export for the executor consumers (centities-native)
pub use centities_core::mobtravel::Mth as TravelMth;

// the vanilla zombie table shape (the parity fixture order):
// target selector: HurtBy(1), Nearest(Player,2), Nearest(3), Nearest(5)
// goal selector: SpearUse(2), ZombieAttack(3), ZombieAttack(4),
//                MoveThroughVillage(6), Stroll(7), LookAt(8), LookAround(8)
pub const N_GOALS: usize = 11;
const FL_TARGET: i64 = g::FL_TARGET;
const FL_ML: i64 = g::FL_MOVE | g::FL_LOOK;
const MF_BASE: i32 = g::MF_NO_PASSENGERS | g::MF_MAINHAND_EMPTY;
const FOLLOW_RANGE: f32 = 35.0;
const LOOK_DIST: f32 = 8.0;
const EYE_Y: f64 = 1.62;

// -- WAVE6: the folded open-ground travel chain (bit-pinned constants) -----

/// Mth.clamp(1 - (1-f)*m, 0, 1) as a const fn (float arithmetic in const
/// context is stable and IEEE-exact — the same ops, the same bits).
const fn cmf_const(f: f32, m: f32) -> f32 {
    let v = 1.0f32 - (1.0f32 - f) * m;
    if v < 0.0f32 {
        0.0f32
    } else if v > 1.0f32 {
        1.0f32
    } else {
        v
    }
}

/// computeModifiedFriction(0.6, 1.0) — the block friction (fm = 1.0).
const OG_FRICTION: f32 = cmf_const(0.6, 1.0);
/// getFrictionInfluencedSpeed(true, friction, 0.23, 0.02) — friction is NOT
/// > 0.6 (0.6 − 3.6e-8 after the f32 chain), so the else arm: plain speed.
const OG_FIS: f32 = if OG_FRICTION > 0.6f32 {
    0.23f32 * (0.21600002f32 / (OG_FRICTION * OG_FRICTION * OG_FRICTION))
} else {
    0.23f32
};
/// computeModifiedFriction(0.91, 1.0) — the ground drag modifier.
const OG_GM: f32 = cmf_const(0.91, 1.0);
/// h = friction * gm — the horizontal drag factor (f32) ...
const OG_H: f32 = OG_FRICTION * OG_GM;
/// ... widened to f64 for the delta multiply.
const OG_HF: f64 = OG_H as f64;
/// computeModifiedFriction(0.98, 1.0) — the vertical drag (omnidirectional
/// false).
const OG_DRAG_I: f32 = cmf_const(0.98, 1.0);
/// dy = (dm.y − 0.08) * i with dm.y ground-clipped to 0.0 — the constant
/// gravity tail.
const OG_GRAV_TAIL: f64 = (0.0f64 - 0.08f64) * (OG_DRAG_I as f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GoalRec {
    pub kind: i32,
    pub prio: i32,
    pub flags: i64,
    pub cfg: i32,
    pub running: bool,
    pub state: i32,
    pub cfg2: i32,
    pub cfg3: i32,
}

impl GoalRec {
    #[inline]
    fn rec(kind: i32, prio: i32, flags: i64, cfg: i32) -> Self {
        GoalRec {
            kind,
            prio,
            flags,
            cfg,
            running: false,
            state: 0,
            cfg2: 0,
            cfg3: 0,
        }
    }
}

/// The vanilla zombie goal table, byte-equal to the parity fixture
/// (goals.rs::build_in — the table PARITY #369 proved bit-identical).
/// `player_scan` marks the two Player-typed goals (cfg3 bit0): the
/// NearestAttackable(Player) and the LookAtPlayer(Player).
pub fn zombie_table() -> [GoalRec; N_GOALS] {
    let mut t = [
        GoalRec::rec(g::GK_HURT_BY, 1, FL_TARGET, 0),
        GoalRec::rec(g::GK_NEAREST_TARGET, 2, FL_TARGET, 10), // Player (cfg3)
        GoalRec::rec(g::GK_NEAREST_TARGET, 3, FL_TARGET, 10),
        GoalRec::rec(g::GK_NEAREST_TARGET, 5, FL_TARGET, 10),
        GoalRec::rec(g::GK_SPEAR_USE, 2, FL_ML, 0),
        GoalRec::rec(g::GK_ZOMBIE_ATTACK, 3, FL_ML, 0),
        GoalRec::rec(g::GK_ZOMBIE_ATTACK, 4, FL_ML, 0),
        GoalRec::rec(g::GK_MOVE_THROUGH_VILLAGE, 6, g::FL_MOVE, 0),
        GoalRec::rec(g::GK_STROLL, 7, g::FL_MOVE, 60), // reducedTickDelay(120)
        GoalRec::rec(g::GK_LOOKAT_PLAYER, 8, g::FL_LOOK, 0.02f32.to_bits() as i32),
        GoalRec::rec(g::GK_LOOKAROUND, 8, FL_ML, 0),
    ];
    t[1].cfg3 = 1; // Player-typed nearest (global idx 1)
    t[9].cfg3 = 1; // Player-typed lookat  (global idx 9)
    t[9].cfg2 = LOOK_DIST.to_bits() as i32; // the context range
    t
}

/// One player snapshot record (the packer's PSTRIDE view).
#[derive(Debug, Clone, Copy)]
pub struct PlayerSnap {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub id: i32,
    /// PF_* bits from goals.rs (PF_ALIVE, PF_SPECTATOR, ...)
    pub flags: i32,
}

/// WAVE4: the RESIDENT-PACK state — ONE per EXECUTOR (it identifies the
/// SHARED scratch's static content, which spans mobs and ticks).
///
/// The aiPack wire buffer splits into three word classes:
/// - STATIC per mob TYPE (goal cfg words, counts, follow, lastHurt) —
///   written once per fingerprint change (`pack_state_static`), never
///   mutated afterward (the goal table is fixed at add_mob — the same
///   contract the wave3 delta-pack already relies on);
/// - TICK-static (game_time, player count, the player snapshot block) —
///   written ONCE per tick by the executor (`pack_tick_words`); identical
///   for every mob;
/// - PER-MOB round words (cursor/mf/counts/no_action/nav/seed/res/pos/
///   running/state) — written per mob per round-1 (`pack_into_resident`)
///   and delta-maintained on rounds 2+ (wave3).
///
/// The invariant "scratch == what a fresh pack_into would write" holds by
/// construction and is verified by (a) the debug probe (full pack word-
/// compare EVERY round), (b) the resident-vs-full twin gate in tests.
#[derive(Debug, Default)]
pub struct PackState {
    /// Some(fp) = the scratch's static half matches this mob-type fp
    pub fp: Option<u64>,
    /// Some((game_time, pcount, player-word hash)) = the scratch's tick
    /// half (game_time, player count, the player block) is current. The
    /// hash covers MOVING players even when game_time/pcount repeat (the
    /// vanilla game_time advances per tick, but the stamp does not rely
    /// on it).
    pub tick: Option<(i32, usize, u64)>,
    /// twin gate: force the FULL pack every round-1 (the wave3 semantics —
    /// used by the resident-vs-full bit test; production keeps false)
    pub full: bool,
}

/// Persistent per-mob state — everything the Java executor holds on the
/// live Mob object, in the same shape aiPack reads.
pub struct MobAI {
    pub id: i64,
    /// the shared-random mirror (SHARED_RANDOM.value)
    pub seed: u64,
    pub tick_count: i32,
    pub no_action: i32,
    pub nav_state: i32,
    pub nav_x: f64,
    pub nav_z: f64,
    pub goals: [GoalRec; N_GOALS],
    /// the lookAt/target player INDEX into this tick's snapshot (-1 none)
    pub lookat_player: i32,
    pub target_player: i32,
    pub rel_x: f64,
    pub rel_z: f64,
    // entity physics (f64 like the entity fields)
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
    pub yaw: f32,
    /// the pending stroll-position result (stashed by the resolver arm
    /// between REQ_STROLL_POS and its REQ_START_VANILLA)
    pub pend_x: f64,
    pub pend_z: f64,
    /// the melee approach point (the target's live position, refreshed
    /// per round from the snapshot)
    pub approach_x: f64,
    pub approach_z: f64,
    /// WAVE4: the static-pack fingerprint (fnv1a over the goal cfg words
    /// + counts/follow/lastHurt header trio). Computed ONCE at
    /// construction — the goal table is fixed at add_mob (the executor
    /// contract; the wave3 delta-pack already assumes the cfg words never
    /// mutate). A stale fp after a post-construction goals swap is caught
    /// by the debug probe (full-pack word compare every round).
    pub pack_fp: u64,
}

impl MobAI {
    /// WAVE4: fnv1a over the words the resident path leaves to the static
    /// writer (goal cfg {0,1,2,3,6,7} per record + counts + follow bits +
    /// the static-zero lastHurtByMobTimestamp). Two mobs share the static
    /// scratch iff this matches.
    fn static_fp(goals: &[GoalRec; N_GOALS]) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for grec in goals.iter() {
            for w in [grec.kind as u64, grec.prio as u64, grec.flags as u64, grec.cfg as u64, grec.cfg2 as u64, grec.cfg3 as u64] {
                h = (h ^ w).wrapping_mul(0x1000_0000_01b3);
            }
        }
        for w in [4u64, 7, FOLLOW_RANGE.to_bits() as u64, 0] {
            h = (h ^ w).wrapping_mul(0x1000_0000_01b3);
        }
        h
    }

    pub fn new(id: i64, seed: u64, x: f64, y: f64, z: f64) -> Self {
        MobAI {
            id,
            seed,
            tick_count: 0,
            no_action: 0,
            nav_state: g::NAV_NONE,
            nav_x: 0.0,
            nav_z: 0.0,
            goals: zombie_table(),
            lookat_player: -1,
            target_player: -1,
            rel_x: 0.0,
            rel_z: 0.0,
            x,
            y,
            z,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            yaw: 0.0,
            pend_x: 0.0,
            pend_z: 0.0,
            approach_x: 0.0,
            approach_z: 0.0,
            pack_fp: Self::static_fp(&zombie_table()),
        }
    }

    /// The moveControl input: forward 0.98 while a path is active
    /// (LivingEntity.aiStep damps xxa/zza by 0.98 — the exact constant).
    #[inline]
    pub fn walking(&self) -> bool {
        self.nav_state == g::NAV_PATH_ACTIVE
    }
}

/// The world-query resolver — the pure-Rust stand-in for CeHooks.aiRun's
/// resolution arm. See the module doc for the honesty ledger.
pub trait Resolver {
    /// REQ_STROLL_POS: the scripted getPosition() result.
    fn stroll_pos(&mut self, mob: &mut MobAI) -> Option<(f64, f64, f64)>;
    /// REQ_LOS_SCAN: which staged candidates pass (bitmask by index).
    fn los(&mut self, n_cands: usize) -> i32;
    /// REQ_CANUSE_VANILLA: the vanilla-parked canUse tail (ZA's
    /// alive+pathfinding half, SpearUse, HurtBy anger, MTV) — true = pass.
    fn canuse_vanilla(&mut self, mob: &MobAI, kind: i32) -> bool;
    /// REQ_CC_VANILLA: the vanilla-parked canContinueToUse — the
    /// classified zombie set parks it for NearestAttackableTargetGoal
    /// and the MeleeAttack goal (ZombieAttack).
    fn cc_vanilla(&mut self, mob: &MobAI, kind: i32) -> bool;
    /// REQ_START_VANILLA: the start body's side effects (the engine's
    /// lock transition already applied via the park round's actions).
    fn start_vanilla(&mut self, mob: &mut MobAI, gi: usize);
    /// REQ_LOOKAT_QUERY: the getNearestPlayer predicate — true = a
    /// candidate exists (the floor picks it from the packed players;
    /// bit-exact distance/tie logic stays in the floor, the resolver owns
    /// the conditions: alive, range, LoS). Default false — the swarm sim
    /// has no players.
    fn lookat_query(&mut self, _mob: &MobAI, _players: &[PlayerSnap]) -> bool {
        false
    }
    /// REQ_FIND_TARGET: the NearestAttackableTargetGoal predicate — true =
    /// at least one targetable candidate within follow range. Same
    /// contract as lookat_query. Default false.
    fn find_target(&mut self, _mob: &MobAI, _players: &[PlayerSnap]) -> bool {
        false
    }
    /// WAVE6: the native FOLD contract — Some(ctx) lets the floor resolve
    /// the pure-v0 requests IN PLACE (no park/resume round-trip: the exact
    /// resolver math runs inside ai_floor_ext over the pack + the carried
    /// streams). None (the default) keeps every request parked — the
    /// Java-mirror twin semantics.
    fn fold_ctx(&self, _mob: &MobAI) -> Option<centities_core::goals::FoldCtx> {
        None
    }
    /// WAVE6: the stroll fold advances the resolver's external target
    /// stream inside the floor; the advanced value comes back through the
    /// out buffer's ext tail. Default: no stream to update.
    fn set_target_rng(&mut self, _s: u64) {}
}

/// The flat open-ground world: nothing occludes, stroll targets are
/// splitmix-scripted, melee approaches while within 16 blocks.
pub struct FlatWorld {
    /// separate target stream (NOT the mob's RNG — see honesty note 1)
    pub target_rng: u64,
    pub stroll_radius: f64,
    /// WAVE6 twin hook: true = fold_ctx returns None — every request
    /// parks, the exact pre-fold semantics (the A/B gate drives both).
    pub no_fold: bool,
}

impl FlatWorld {
    pub fn new(target_seed: u64) -> Self {
        FlatWorld {
            target_rng: target_seed,
            stroll_radius: 10.0,
            no_fold: false,
        }
    }
}

#[inline]
fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ z >> 31
}

impl Resolver for FlatWorld {
    fn fold_ctx(&self, mob: &MobAI) -> Option<centities_core::goals::FoldCtx> {
        if self.no_fold {
            return None;
        }
        Some(centities_core::goals::FoldCtx {
            on: true,
            los_all_pass: true,
            stroll_seed: self.target_rng,
            stroll_lcg16: false,
            stroll_radius: self.stroll_radius,
            follow_range: 35.0,
            target_player: mob.target_player,
            approach_x: mob.approach_x,
            approach_z: mob.approach_z,
            stroll_drew: false,
            ext_cands: false,
            find_target_fold: false, // FlatWorld's find_target contract = false (no targets)
        })
    }

    fn set_target_rng(&mut self, s: u64) {
        self.target_rng = s;
    }

    fn stroll_pos(&mut self, mob: &mut MobAI) -> Option<(f64, f64, f64)> {
        let r = self.stroll_radius;
        let dx = (splitmix(&mut self.target_rng) & 0xFFFF) as f64 / 65535.0 * 2.0 * r - r;
        let dz = (splitmix(&mut self.target_rng) & 0xFFFF) as f64 / 65535.0 * 2.0 * r - r;
        Some((mob.x + dx, mob.y, mob.z + dz))
    }

    #[inline]
    fn los(&mut self, n_cands: usize) -> i32 {
        // flat open ground: every candidate has line of sight
        if n_cands >= 32 {
            -1
        } else {
            (1i32 << n_cands) - 1
        }
    }

    fn canuse_vanilla(&mut self, mob: &MobAI, kind: i32) -> bool {
        // the open ground always yields the path; the target lives
        matches!(kind, g::GK_ZOMBIE_ATTACK) && mob.target_player >= 0
    }

    fn cc_vanilla(&mut self, mob: &MobAI, kind: i32) -> bool {
        if mob.target_player < 0 {
            return false;
        }
        let (tx, tz) = (mob.approach_x, mob.approach_z);
        let (dx, dz) = (tx - mob.x, tz - mob.z);
        match kind {
            // TargetGoal.canContinueToUse keeps the target while the
            // interval recheck would re-pick it: the follow-range bound
            // (the open ground always passes the conditions)
            g::GK_NEAREST_TARGET => dx * dx + dz * dz <= 35.0 * 35.0,
            // MeleeAttackGoal chases without a range cutoff (the path
            // repath is Java-side; the open-ground straight line is the
            // proven path)
            _ => true,
        }
    }

    fn start_vanilla(&mut self, mob: &mut MobAI, gi: usize) {
        match mob.goals[gi].kind {
            g::GK_STROLL => {
                // WaterAvoidingRandomStrollGoal.start(): navigation.moveTo(
                //   wantedX, wantedY, wantedZ, speedModifier) — the open
                //   ground always yields a path
                mob.nav_state = g::NAV_PATH_ACTIVE;
                mob.nav_x = mob.pend_x;
                mob.nav_z = mob.pend_z;
                mob.goals[gi].running = true;
                mob.no_action = 0;
            }
            g::GK_LOOKAROUND => {
                // RandomLookAroundGoal.start() — the DRAW-EXACT body
                // (javap 26.3: nextDouble + nextInt(20)):
                let d0 = std::f64::consts::PI * 2.0 * lcg_next_f64(&mut mob.seed);
                mob.rel_x = d0.cos();
                mob.rel_z = d0.sin();
                mob.goals[gi].state = 20 + lcg_next_int(&mut mob.seed, 20);
                mob.goals[gi].running = true;
                mob.no_action = 0;
            }
            g::GK_ZOMBIE_ATTACK => {
                mob.goals[gi].running = true;
                mob.no_action = 0;
            }
            _ => {}
        }
    }
}

// -- the exact ThreadUnsafeRandom math (mirror of goals.rs::Lcg) ----------

#[inline]
pub fn lcg_next(seed: &mut u64, bits: u32) -> i32 {
    *seed = seed.wrapping_mul(25214903917).wrapping_add(11) & 0xFFFF_FFFF_FFFF;
    (*seed >> (48 - bits)) as i32
}

#[inline]
pub fn lcg_next_int(seed: &mut u64, bound: i32) -> i32 {
    if bound <= 0 {
        return 0;
    }
    if bound & (bound - 1) == 0 {
        return (((bound as i64) * (lcg_next(seed, 31) as i64)) >> 31) as i32;
    }
    loop {
        let bits = lcg_next(seed, 31);
        let val = bits % bound;
        if bits.wrapping_sub(val).wrapping_add(bound - 1) >= 0 {
            return val;
        }
    }
}

#[inline]
pub fn lcg_next_f64(seed: &mut u64) -> f64 {
    let hi = (lcg_next(seed, 26) as i64) << 27;
    let lo = lcg_next(seed, 27) as i64;
    ((hi + lo) as f64) * (1.0f64 / 9007199254740992.0f64)
}

// -- the executor ---------------------------------------------------------

#[inline]
fn put_f64(b: &mut [i32], idx: usize, v: f64) {
    let bits = v.to_bits();
    b[idx] = bits as i32;
    b[idx + 1] = (bits >> 32) as i32;
}

/// Per-tick accounting for the hot loop.
#[derive(Debug, Default, Clone, Copy)]
pub struct AiStat {
    pub rounds: u32,
    pub requests: u32,
    pub status: i32,
    pub fallback: i32,
    /// the last staged (request op, goal index) — diagnostics
    pub last_req: (i32, usize),
}

const OUT_LEN: usize = g::OUT_EXT_LEN;

/// The out-buffer length the executor must provide to `ai_tick` (the
/// wave2 shared-buffer contract — one allocation per EXECUTOR, not per mob).
pub const fn out_len() -> usize {
    OUT_LEN
}

/// Words the executor READS from the out buffer before the floor writes
/// them: the RNG handoff pair on the fallback path (fallback() writes
/// {0,5,7} only). Every other read word (0,1,2,5,6,7,8 + act slots) is
/// write-before-read by the floor's exit discipline (done/park/fallback
/// bodies + Out::push). Zeroing these nine words per mob reproduces the
/// old fresh-`[0i32; OUT_LEN]` semantics bit-for-bit at 1/340th the cost.
const OUT_RESET: usize = 9;

impl MobAI {
    /// The packed LOOKAT state word (wave3: shared by pack_into AND the
    /// apply-mirror — one formula, no drift): live state bits 0..30, the
    /// has-lookat bit 31, the within-look-range bit 30.
    #[inline]
    fn packed_lookat_state(&self, gi: usize, players: &[PlayerSnap]) -> i32 {
        let rec = &self.goals[gi];
        let mut st = (rec.state as u32 & 0x3FFF_FFFF) as i32;
        if self.lookat_player >= 0 {
            st |= 1 << 31;
            if let Some(p) = players.get(self.lookat_player as usize) {
                let dx = self.x - p.x;
                let dy = self.y - p.y;
                let dz = self.z - p.z;
                let d2 = dx * dx + dy * dy + dz * dz;
                let range = LOOK_DIST as f64;
                if d2 <= range * range {
                    st |= 1 << 30;
                }
            }
        }
        st
    }
    /// WAVE3: re-derive the packed words the REQUEST ARM can mutate
    /// (outside apply_actions): the goal's running/state pair, no_action,
    /// nav_state. Called after the resolver's start_vanilla and the
    /// melee canUse state write — resolver-agnostic (repacks from LIVE
    /// state, which is what a fresh pack would read).
    #[inline]
    fn repack_goal_words(&self, gi: usize, b: &mut [i32], players: &[PlayerSnap]) {
        let o = g::IN_HEADER + gi * g::GOAL_STRIDE;
        b[o + 4] = self.goals[gi].running as i32;
        b[o + 5] = match self.goals[gi].kind {
            g::GK_LOOKAROUND
            | g::GK_ZOMBIE_ATTACK
            | g::GK_HURT_BY
            | g::GK_MOVE_THROUGH_VILLAGE => self.goals[gi].state,
            g::GK_LOOKAT_PLAYER => self.packed_lookat_state(gi, players),
            _ => 0,
        };
        b[4] = self.no_action;
        b[11] = self.nav_state;
    }

    /// Pack the aiPack wire format into a CALLER buffer (wave2: zero
    /// alloc — the executor owns one scratch, reused across all mobs and
    /// rounds). Writes exactly `need` words; the caller may pass a longer
    /// scratch (the tail is never read by the floor — its len check is a
    /// minimum).
    #[allow(clippy::too_many_arguments)]
    pub fn pack_into(
        &self,
        b: &mut [i32],
        players: &[PlayerSnap],
        game_time: i32,
        cursor: i32,
        res: i32,
        req_op: i32,
        rx: f64,
        ry: f64,
        rz: f64,
    ) {
        let need = g::IN_HEADER + N_GOALS * g::GOAL_STRIDE + players.len() * g::PSTRIDE;
        debug_assert!(b.len() >= need, "pack scratch too small");
        let b = &mut b[..need];
        b[0] = cursor;
        let mut mf = MF_BASE;
        if self.target_player >= 0 {
            mf |= g::MF_HAS_TARGET;
        }
        // NOTE: MF_BRIGHT_OUTSIDE is deliberately NOT set — the classified
        // zombie table's MoveThroughVillage has onlyAtNight=0 (the parity
        // fixture cfg), so the bright-outside gate is a no-op and the pack
        // stays byte-equal to it.
        b[1] = mf;
        b[2] = self.tick_count;
        b[3] = self.id as i32;
        b[4] = self.no_action;
        b[5] = 4; // target selector count
        b[6] = 7; // goal selector count
        b[11] = self.nav_state;
        b[g::IN_SEED_LO] = self.seed as i32;
        b[g::IN_SEED_HI] = (self.seed >> 32) as i32;
        b[g::IN_RES] = res;
        b[g::IN_RES_OP] = req_op;
        put_f64(b, g::IN_RES_X, rx);
        put_f64(b, g::IN_RES_X + 2, ry);
        put_f64(b, g::IN_RES_X + 4, rz);
        b[22] = 0; // lastHurtByMobTimestamp (the bench mob is never hurt)
        b[23] = game_time;
        b[g::IN_PCOUNT] = players.len() as i32;
        b[g::IN_FOLLOW] = FOLLOW_RANGE.to_bits() as i32;
        put_f64(b, g::IN_MOBX, self.x);
        put_f64(b, g::IN_MOBY, self.y);
        put_f64(b, g::IN_MOBZ, self.z);
        put_f64(b, g::IN_EYEY, self.y + EYE_Y);
        // goal records (mirror of aiPackSel's live state words)
        let base = g::IN_HEADER;
        for (i, rec) in self.goals.iter().enumerate() {
            let o = base + i * g::GOAL_STRIDE;
            b[o] = rec.kind;
            b[o + 1] = rec.prio;
            b[o + 2] = rec.flags as i32;
            b[o + 3] = rec.cfg;
            b[o + 4] = rec.running as i32;
            b[o + 5] = match rec.kind {
                g::GK_LOOKAROUND
                | g::GK_ZOMBIE_ATTACK
                | g::GK_HURT_BY
                | g::GK_MOVE_THROUGH_VILLAGE => rec.state,
                g::GK_LOOKAT_PLAYER => self.packed_lookat_state(i, players),
                _ => 0,
            };
            b[o + 6] = rec.cfg2;
            b[o + 7] = rec.cfg3;
        }
        // player snapshot records (level.players() order)
        let pb = base + N_GOALS * g::GOAL_STRIDE;
        for (i, p) in players.iter().enumerate() {
            let o = pb + i * g::PSTRIDE;
            put_f64(b, o, p.x);
            put_f64(b, o + 2, p.y);
            put_f64(b, o + 4, p.z);
            put_f64(b, o + 6, p.y + EYE_Y);
            put_f64(b, o + 8, 1.0); // visibility 100%
            b[o + 10] = p.flags;
            b[o + 11] = p.id;
        }
    }

    /// WAVE4: the STATIC pack half — the words that never change while the
    /// mob lives (goal cfg words, selector counts, follow range, the
    /// static-zero lastHurtByMobTimestamp). Written into the shared scratch
    /// ONCE per mob-type fingerprint, never touched afterward. Words 7..10
    /// (the disabled-flags pair) stay the scratch's zeros — pack_into never
    /// writes them either; the floor reads zeros from both paths.
    fn pack_state_static(&self, b: &mut [i32]) {
        let end = g::IN_HEADER + N_GOALS * g::GOAL_STRIDE;
        debug_assert!(b.len() >= end, "pack scratch too small");
        b[5] = 4; // target selector count
        b[6] = 7; // goal selector count
        b[22] = 0; // lastHurtByMobTimestamp (the bench mob is never hurt)
        b[g::IN_FOLLOW] = FOLLOW_RANGE.to_bits() as i32;
        let base = g::IN_HEADER;
        for (i, rec) in self.goals.iter().enumerate() {
            let o = base + i * g::GOAL_STRIDE;
            b[o] = rec.kind;
            b[o + 1] = rec.prio;
            b[o + 2] = rec.flags as i32;
            b[o + 3] = rec.cfg;
            b[o + 6] = rec.cfg2;
            b[o + 7] = rec.cfg3;
        }
    }

    /// WAVE4: the RESIDENT round pack — writes ONLY the per-mob round
    /// words into a scratch whose static half (pack_state_static) and tick
    /// half (pack_tick_words) are already current: ~46 words instead of
    /// pack_into's 134, EVERY round (fresh OR resumed — the wave3 delta
    /// arm subsumed; the probe's first catch was a resumed round with the
    /// park token 0 = SEL_TARGET/PH_CLEANUP/gi0 whose request words MUST
    /// ride the pack, not zeros).
    #[allow(clippy::too_many_arguments)]
    fn pack_into_resident(
        &self,
        b: &mut [i32],
        players: &[PlayerSnap],
        cursor: i32,
        res: i32,
        req_op: i32,
        rx: f64,
        ry: f64,
        rz: f64,
    ) {
        let need = g::IN_HEADER + N_GOALS * g::GOAL_STRIDE + players.len() * g::PSTRIDE;
        debug_assert!(b.len() >= need, "pack scratch too small");
        b[0] = cursor;
        b[1] = MF_BASE
            | if self.target_player >= 0 {
                g::MF_HAS_TARGET
            } else {
                0
            };
        b[2] = self.tick_count;
        b[3] = self.id as i32;
        b[4] = self.no_action;
        b[11] = self.nav_state;
        b[g::IN_SEED_LO] = self.seed as i32;
        b[g::IN_SEED_HI] = (self.seed >> 32) as i32;
        b[g::IN_RES] = res;
        b[g::IN_RES_OP] = req_op;
        put_f64(b, g::IN_RES_X, rx);
        put_f64(b, g::IN_RES_X + 2, ry);
        put_f64(b, g::IN_RES_X + 4, rz);
        put_f64(b, g::IN_MOBX, self.x);
        put_f64(b, g::IN_MOBY, self.y);
        put_f64(b, g::IN_MOBZ, self.z);
        put_f64(b, g::IN_EYEY, self.y + EYE_Y);
        let base = g::IN_HEADER;
        for (i, rec) in self.goals.iter().enumerate() {
            let o = base + i * g::GOAL_STRIDE;
            b[o + 4] = rec.running as i32;
            b[o + 5] = match rec.kind {
                g::GK_LOOKAROUND
                | g::GK_ZOMBIE_ATTACK
                | g::GK_HURT_BY
                | g::GK_MOVE_THROUGH_VILLAGE => rec.state,
                g::GK_LOOKAT_PLAYER => self.packed_lookat_state(i, players),
                _ => 0,
            };
        }
    }

    /// Pack the aiPack wire format from the live state (exact-len Vec;
    /// the parity-fixture and hot-path-test entry — the executor's hot
    /// path uses `pack_into` with a shared scratch).
    #[allow(clippy::too_many_arguments)]
    pub fn pack(
        &self,
        players: &[PlayerSnap],
        game_time: i32,
        cursor: i32,
        res: i32,
        req_op: i32,
        rx: f64,
        ry: f64,
        rz: f64,
    ) -> Vec<i32> {
        let need = g::IN_HEADER + N_GOALS * g::GOAL_STRIDE + players.len() * g::PSTRIDE;
        let mut b = vec![0i32; need];
        self.pack_into(&mut b, players, game_time, cursor, res, req_op, rx, ry, rz);
        b
    }

    /// WAVE4: fnv1a over the player words the tick half owns (x/y/z/id/
    /// flags) — computed ONCE PER TICK by the caller (the executor / test
    /// driver) and passed to ai_tick as `tfp`: the tick-half stamp's third
    /// member, so MOVING players with a repeated (game_time, pcount) still
    /// invalidate. The per-mob loop pays ZERO hash cost.
    pub fn tick_fp(players: &[PlayerSnap]) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for p in players {
            for w in [
                p.x.to_bits(),
                p.y.to_bits(),
                p.z.to_bits(),
                p.id as u32 as u64,
                p.flags as u32 as u64,
            ] {
                h = (h ^ w).wrapping_mul(0x1000_0000_01b3);
            }
        }
        h
    }

    /// WAVE4: the TICK-static pack half — game_time, the player count and
    /// the player snapshot block. Identical for every mob in the tick;
    /// written ONCE per tick into the shared scratch when the stamp says
    /// stale (the old path rewrote it per mob — 12+ words × 25k mobs =
    /// 300k+ redundant writes per tick at one player).
    fn pack_tick_words(b: &mut [i32], players: &[PlayerSnap], game_time: i32) {
        let need = g::IN_HEADER + N_GOALS * g::GOAL_STRIDE + players.len() * g::PSTRIDE;
        debug_assert!(b.len() >= need, "pack scratch too small for players");
        b[23] = game_time;
        b[g::IN_PCOUNT] = players.len() as i32;
        let pb = g::IN_HEADER + N_GOALS * g::GOAL_STRIDE;
        for (i, p) in players.iter().enumerate() {
            let o = pb + i * g::PSTRIDE;
            put_f64(b, o, p.x);
            put_f64(b, o + 2, p.y);
            put_f64(b, o + 4, p.z);
            put_f64(b, o + 6, p.y + EYE_Y);
            put_f64(b, o + 8, 1.0); // visibility 100%
            b[o + 10] = p.flags;
            b[o + 11] = p.id;
        }
    }

    /// Apply the emitted actions to the live state (the aiApply mirror).
    /// WAVE3: `packed` — the shared aiPack scratch; EVERY mutation here
    /// mirrors into the packed word(s) it owns, so the next round's
    /// delta-pack can skip the full rebuild: the invariant
    /// "scratch == what pack_into would write" holds by construction
    /// (single write site per field). The twin gate
    /// (reused_out_buffer_matches_fresh_zero) pins this bit-for-bit.
    /// WAVE4: `packed` is a plain `&mut [i32]` (the Option dance cost a
    /// branch per mirrored action; the executor always has the scratch).
    fn apply_actions(
        &mut self,
        out: &[i32],
        players: &[PlayerSnap],
        packed: &mut [i32],
    ) {
        let n = out[5].max(0) as usize;
        if n == 0 {
            return;
        }
        for a in 0..n {
            let b = g::OUT_HEADER + a * g::OUT_ACT;
            let op = out[b];
            let p0 = out[b + 1] as usize;
            let p1 = out[b + 2];
            // the packed-word mirror (goal record base + the two live
            // words every mutation can touch)
            let gw = p0 * g::GOAL_STRIDE;
            match op {
                g::ACT_GOAL_START => {
                    if p0 < N_GOALS {
                        self.goals[p0].running = true;
                        self.no_action = 0;
                        packed[g::IN_HEADER + gw + 4] = 1;
                        packed[4] = self.no_action;
                        // the start BODIES the executor runs itself (the
                        // engine emitted the transition, vanilla runs
                        // start() inline — aiApply: lockWrite + wg.start()):
                        match self.goals[p0].kind {
                            // LookAtPlayerGoal.start(): lookTime =
                            // adjustedTickDelay(40), no RNG (javap 26.3)
                            g::GK_LOOKAT_PLAYER => {
                                self.goals[p0].state = 40;
                                packed[g::IN_HEADER + gw + 5] =
                                    self.packed_lookat_state(p0, players);
                            }
                            // RandomStrollGoal.start(): navigation.moveTo(
                            //   wantedPos, speedModifier) — the position the
                            //   REQ_STROLL_POS resolution stored on the goal
                            g::GK_STROLL => {
                                self.nav_state = g::NAV_PATH_ACTIVE;
                                self.nav_x = self.pend_x;
                                self.nav_z = self.pend_z;
                                packed[11] = self.nav_state;
                            }
                            _ => {}
                        }
                    }
                }
                g::ACT_GOAL_LOCK_MAP => { /* map-only; running flips at start */ }
                // WAVE6 stroll fold: the floor drew the v0 target from the
                // external stream — stash it (pend); the paired GOAL_START
                // (later in this action list, lock_only's push) sets nav from
                // pend — the same resolution→start order as the parked flow.
                g::ACT_STROLL_SET => {
                    self.pend_x = f64::from_bits(
                        ((out[b + 3] as u64) << 32) | (out[b + 2] as u64 & 0xFFFF_FFFF),
                    );
                    self.pend_z = f64::from_bits(
                        ((out[b + 5] as u64) << 32) | (out[b + 4] as u64 & 0xFFFF_FFFF),
                    );
                }
                // WAVE6 RLA start fold: the floor ran the DRAW-EXACT start
                // body on its own LCG — apply the rel/state/running writes
                // (the parked flow's start_vanilla body, action-shaped).
                g::ACT_RLA_START => {
                    if p0 < N_GOALS {
                        self.rel_x = f64::from_bits(
                            ((out[b + 3] as u64) << 32) | (out[b + 2] as u64 & 0xFFFF_FFFF),
                        );
                        self.rel_z = f64::from_bits(
                            ((out[b + 5] as u64) << 32) | (out[b + 4] as u64 & 0xFFFF_FFFF),
                        );
                        self.goals[p0].state = out[b + 6];
                        self.goals[p0].running = true;
                        self.no_action = 0;
                        packed[g::IN_HEADER + gw + 4] = 1;
                        packed[g::IN_HEADER + gw + 5] = out[b + 6];
                        packed[4] = 0;
                    }
                }
                g::ACT_GOAL_STOP | g::ACT_GOAL_STOP_D => {
                    if p0 < N_GOALS {
                        self.goals[p0].running = false;
                        packed[g::IN_HEADER + gw + 4] = 0;
                        if self.goals[p0].kind == g::GK_STROLL
                            && self.nav_state == g::NAV_PATH_ACTIVE
                        {
                            self.nav_state = g::NAV_NONE;
                            packed[11] = self.nav_state;
                        }
                    }
                }
                g::ACT_GOAL_TICK_V => { /* the virtual melee body (scripted approach below) */ }
                g::ACT_LOOK_TICK_AROUND => {
                    if p0 < N_GOALS {
                        self.goals[p0].state = p1;
                        packed[g::IN_HEADER + gw + 5] = p1;
                    }
                }
                g::ACT_LOOK_TICK_AT => {
                    if p0 < N_GOALS {
                        self.goals[p0].state = p1;
                        packed[g::IN_HEADER + gw + 5] = self.packed_lookat_state(p0, players);
                    }
                }
                g::ACT_REF_SET_PLAYER => {
                    if p0 < N_GOALS {
                        match self.goals[p0].kind {
                            g::GK_LOOKAT_PLAYER => {
                                self.lookat_player = p1;
                                if p1 < 0 {
                                    self.goals[p0].state &= !((1 << 31) | (1 << 30));
                                }
                                packed[g::IN_HEADER + gw + 5] =
                                    self.packed_lookat_state(p0, players);
                            }
                            g::GK_NEAREST_TARGET => {
                                self.target_player = p1;
                                if p1 >= 0 {
                                    if let Some(p) = players.get(p1 as usize) {
                                        self.approach_x = p.x;
                                        self.approach_z = p.z;
                                    }
                                }
                                // the mf word rides target_player
                                packed[1] = MF_BASE
                                    | if self.target_player >= 0 {
                                        g::MF_HAS_TARGET
                                    } else {
                                        0
                                    };
                            }
                            _ => {}
                        }
                    }
                }
                g::ACT_MOVE_ZZA0 => { /* moveControl WAIT — no forward input this tick */ }
                g::ACT_NAV_IDLE => { /* idle nav bookkeeping */ }
                _ => {}
            }
        }
    }

    /// One full AI tick: the aiRun round loop (cap 32) with the resolver
    /// in the request arm. Returns the accounting.
    ///
    /// WAVE2 hot-path contract:
    /// - `R: Resolver + ?Sized` — concrete resolvers monomorphize (no
    ///   vtable in the inner loop); `&mut dyn Resolver` still works.
    /// - `scratch` — the shared aiPack buffer (one per EXECUTOR); grown
    ///   once, reused across every mob and round: zero steady-state
    ///   allocation (the old code allocated + memset a fresh Vec PER
    ///   ROUND).
    /// - `out` — the shared floor-output buffer (one per EXECUTOR). The
    ///   floor's write discipline (verified: done/park write {0..6},
    ///   fallback writes {0,5,7}, Out::push writes full act slots, and
    ///   every executor read word is write-before-read EXCEPT the RNG
    ///   handoff pair on the fallback path) lets us reset just
    ///   `out[..OUT_RESET]` per mob instead of memsetting 3 KiB per mob.
    ///   `reused_out_buffer_matches_fresh_zero` gates this bit-for-bit.
    /// WAVE4: `pack` — the resident-pack state (ONE per executor; see the
    /// PackState doc for the static/tick/round word split). `pack.full =
    /// true` restores the wave3 full-pack semantics (the twin gate).
    /// `tfp` — the caller's per-tick player fingerprint (`MobAI::tick_fp`
    /// over THIS tick's players slice; the stamp check is free in the
    /// mob loop).
    #[allow(clippy::too_many_arguments)]
    pub fn ai_tick<R: Resolver + ?Sized>(
        &mut self,
        players: &[PlayerSnap],
        game_time: i32,
        r: &mut R,
        scratch: &mut Vec<i32>,
        out: &mut [i32],
        pack: &mut PackState,
        tfp: u64,
    ) -> AiStat {
        debug_assert!(out.len() >= OUT_LEN, "out buffer too small");
        // the sub-phase ledger gate (1-in-SUB_EVERY mobs on sampled
        // ticks; the guard drops at scope end)
        let _sub = crate::ledger::begin_mob();
        // WAVE6: the fold contract — None keeps every request parked (the
        // Java-mirror twin); Some lets ai_floor_ext resolve the pure-v0
        // requests in place. Built ONCE per mob-tick from the resolver +
        // the mob's carried target/approach; None degrades to FoldCtx::off.
        let mut fold = r.fold_ctx(self).unwrap_or_else(g::FoldCtx::off);
        // WAVE6 FIX: the native executor exchanges LOS candidates through the
        // EXT tail in BOTH modes (folded AND the parked twin) — the legacy
        // wire slot out[8] IS action slot 0 and let a same-round action be
        // replaced by the candidate list (the phantom ACT_GOAL_START p0=0).
        fold.ext_cands = true;
        self.tick_count = self.tick_count.wrapping_add(1);
        self.no_action = self.no_action.saturating_add(1);
        let mut stat = AiStat::default();
        let (mut cursor, mut res, mut req_op) = (0i32, 0i32, 0i32);
        let (mut rx, mut ry, mut rz) = (0f64, 0f64, 0f64);
        out[..OUT_RESET].fill(0);
        let need = g::IN_HEADER + N_GOALS * g::GOAL_STRIDE + players.len() * g::PSTRIDE;
        if scratch.len() < need {
            scratch.resize(need, 0);
        }
        // WAVE4: the static + tick halves are stamped BEFORE the round loop
        // (constant across rounds; the stamp checks cost nothing per mob
        // once the caller hoisted tfp to the tick level)
        if !pack.full {
            if pack.fp != Some(self.pack_fp) {
                self.pack_state_static(scratch);
                pack.fp = Some(self.pack_fp);
            }
            if pack.tick != Some((game_time, players.len(), tfp)) {
                Self::pack_tick_words(scratch, players, game_time);
                pack.tick = Some((game_time, players.len(), tfp));
            }
        }
        for _round in 0..32 {
            {
                let _p = crate::ledger::sg_pack();
                if pack.full {
                    self.pack_into(
                        scratch, players, game_time, cursor, res, req_op, rx, ry, rz,
                    );
                } else {
                    // WAVE4 resident round pack: statics + tick half already
                    // stamped above; the ~46 per-mob round words
                    // (cursor/res/op/seed/mf/pos/goals) are written EVERY
                    // round, fresh OR resumed (the wave3 delta arm is
                    // subsumed; a park token of 0 — SEL_TARGET/PH_CLEANUP/
                    // gi0 — resumes through THIS same arm and carries its
                    // request words).
                    self.pack_into_resident(
                        scratch, players, cursor, res, req_op, rx, ry, rz,
                    );
                }
                #[cfg(debug_assertions)]
                {
                    // the wave3+wave4 invariant probe: a fresh FULL pack must
                    // equal the maintained scratch, word for word, EVERY
                    // round (covers the resident round-1 arm AND the delta
                    // arm; the round-variable words differ ONLY where the
                    // resolution result landed — which this fresh pack also
                    // carries)
                    let mut probe = scratch.to_vec();
                    self.pack_into(
                        &mut probe,
                        players,
                        game_time,
                        cursor,
                        res,
                        req_op,
                        rx,
                        ry,
                        rz,
                    );
                    for i in 0..need {
                        assert_eq!(
                            scratch[i], probe[i],
                            "pack drift at word {i} (round {cursor:x})"
                        );
                    }
                }
            }
            let st = {
                let _f = crate::ledger::sg_floor();
                // WAVE6: ALWAYS the ext entry — fold.on=false keeps the exact
                // parked semantics while still honoring ext_cands (the
                // plain ai_floor entry would build an internal FoldCtx::off
                // and fall back to the colliding wire slot)
                g::ai_floor_ext(scratch, out, &mut fold)
            };
            stat.rounds += 1;
            // seed handoff (aiRun: out -> SHARED_RANDOM after EVERY round)
            self.seed = ((out[4] as u64) << 32) | (out[3] as u64 & 0xFFFF_FFFF);
            // WAVE6: the external stream handoff — applied ONLY when the
            // floor actually drew (fold.stroll_drew): the out buffer is
            // SHARED across mobs, so an unconditional read would clobber
            // the stream with the previous mob's value. The floor can draw
            // at most once per call; the value chains exactly.
            if fold.stroll_drew && out.len() >= g::OUT_EXT_LO + 2 {
                let s = ((out[g::OUT_EXT_LO + 1] as u64) << 32)
                    | (out[g::OUT_EXT_LO] as u64 & 0xFFFF_FFFF);
                r.set_target_rng(s);
                fold.stroll_drew = false; // re-arm for the next round
            }
            {
                let _a = crate::ledger::sg_acts();
                self.apply_actions(out, players, scratch);
            }
            if st == g::ST_FALLBACK {
                stat.status = st;
                stat.fallback = out[7];
                return stat;
            }
            if st != g::ST_REQUEST {
                stat.status = st;
                return stat;
            }
            stat.requests += 1;
            cursor = out[6];
            req_op = out[1];
            // out[2] is the LOCAL index within the parked selector (the
            // Java driver indexes ctx.tGoals/gGoals with it); the goals
            // array here is the CONCATENATED table (target goals first),
            // so the goal-selector parks offset by the target count.
            let gi_local = out[2].max(0) as usize;
            let sel = ((cursor >> 9) & 1) as usize;
            let gi = if sel == g::SEL_TARGET {
                gi_local
            } else {
                4 + gi_local // TARGET_SEL_COUNT — the zombie table's target half
            };
            stat.last_req = (req_op, gi);
            let _rs = crate::ledger::sg_resolve();
            (res, rx, ry, rz) = match req_op {
                g::REQ_LOS_SCAN => {
                    // WAVE6 FIX: the cand count rides the EXT tail (native
                    // ext_cands mode) — out[8] is the JNI wire slot that
                    // overlaps action slot 0
                    let n = if fold.ext_cands && out.len() >= g::OUT_EXT_LEN {
                        out[g::OUT_EXT_LO + 2].max(0) as usize
                    } else {
                        out[8].max(0) as usize
                    };
                    (r.los(n), 0.0, 0.0, 0.0)
                }
                g::REQ_STROLL_POS => match r.stroll_pos(self) {
                    Some((x, y, z)) => {
                        // stash for the start resolution
                        self.pend_x = x;
                        self.pend_z = z;
                        (1, x, y, z)
                    }
                    None => (0, 0.0, 0.0, 0.0),
                },
                g::REQ_START_VANILLA => {
                    if gi < N_GOALS {
                        r.start_vanilla(self, gi);
                        // wave3: the resolver mutated live state (running,
                        // state, no_action, nav_state) — re-derive the
                        // packed words from it
                        self.repack_goal_words(gi, scratch, players);
                    }
                    (1, 0.0, 0.0, 0.0)
                }
                g::REQ_CANUSE_VANILLA => {
                    if gi < N_GOALS {
                        let kind = self.goals[gi].kind;
                        let pass = r.canuse_vanilla(self, kind);
                        // the 26.3 MeleeAttackGoal.canUse sets lastCanUseCheck
                        // AFTER the throttle gate — the state word is the
                        // executor's (aiPackSel reads it live)
                        if pass && kind == g::GK_ZOMBIE_ATTACK {
                            self.goals[gi].state = game_time;
                            self.repack_goal_words(gi, scratch, players);
                        }
                        (pass as i32, 0.0, 0.0, 0.0)
                    } else {
                        (0, 0.0, 0.0, 0.0)
                    }
                }
                g::REQ_CC_VANILLA => {
                    let kind = self.goals.get(gi).map(|rec| rec.kind).unwrap_or(0);
                    if kind == g::GK_ZOMBIE_ATTACK || kind == g::GK_NEAREST_TARGET {
                        let cont = r.cc_vanilla(self, kind);
                        if !cont {
                            // the target/attack chain drops; the
                            // engine's next walk stops the goals
                            self.target_player = -1;
                            // wave3: mf rides target_player
                            scratch[1] = MF_BASE;
                        }
                        (cont as i32, 0.0, 0.0, 0.0)
                    } else {
                        (0, 0.0, 0.0, 0.0)
                    }
                }
                g::REQ_LOOKAT_QUERY => {
                    (r.lookat_query(self, players) as i32, 0.0, 0.0, 0.0)
                }
                g::REQ_FIND_TARGET => {
                    (r.find_target(self, players) as i32, 0.0, 0.0, 0.0)
                }
                _ => (0, 0.0, 0.0, 0.0), // unknown ops: fail closed
            };
            drop(_rs);
        }
        // the 32-round cap: the Java side falls back (mirror covers the
        // tick) — surface it in the accounting
        stat.status = g::ST_FALLBACK;
        stat.fallback = g::FB_JAVA_ROUND_CAP;
        stat
    }

    /// The movement slice: nav/approach input -> travel -> setPos.
    /// Open-ground routing (the proven case); block friction 0.6,
    /// friction/air-drag modifiers 1.0, speed attribute 0.23, gravity
    /// 0.08 — the zombie defaults the parity legs used.
    pub fn move_tick(&mut self, mth: &Mth) {
        let mut zza = 0.0f32;
        if self.target_player >= 0 && self.goals[5].running {
            // attacking: face and approach the target
            let (ddx, ddz) = (self.approach_x - self.x, self.approach_z - self.z);
            let d2 = ddx * ddx + ddz * ddz;
            if d2 > 2.25 {
                // beyond melee reach: walk in
                self.face(ddx, ddz, d2);
                zza = 0.98;
            }
        } else if self.nav_state == g::NAV_PATH_ACTIVE {
            let (ddx, ddz) = (self.nav_x - self.x, self.nav_z - self.z);
            let d2 = ddx * ddx + ddz * ddz;
            if d2 < 0.25 {
                self.nav_state = g::NAV_PATH_DONE;
            } else {
                self.face(ddx, ddz, d2);
                zza = 0.98;
            }
        }
        // WAVE6: the friction/drag chain is CONSTANT for every mob every
        // tick under the open-ground contract (block friction 0.6, friction
        // and air-drag modifiers 1.0, speed 0.23, gravity 0.08) — folded
        // once into consts, pinned bit-exactly against the runtime fns by
        // travel_open_ground_consts_pin. The per-mob recomputation of the
        // same five f32 values (~10 flops + 2 casts per mob-tick) is gone.
        let (ix, _iy, iz) = mobtravel::get_input_vector(mth, 0.0, 0.0, zza as f64, OG_FIS, self.yaw);
        let (mx, mz) = (self.dx + ix, self.dz + iz);
        // move consumes the pre-delta; the open ground clips nothing
        // horizontally; the setPos condition is the exact 26.3 predicate
        if mobtravel::move_sets_position((mx, self.dy, mz)) {
            self.x += mx;
            self.z += mz;
        }
        // the drag tail (dm.y is ground-clipped to 0 — the folded constant
        // gravity tail; the same slice travel_in_air_open_ground runs)
        self.dx = mx * OG_HF;
        self.dy = OG_GRAV_TAIL;
        self.dz = mz * OG_HF;
    }

    #[inline]
    fn face(&mut self, ddx: f64, ddz: f64, d2: f64) {
        // vanilla yaw: 0 = +Z, clockwise; input (0,0,zza) rotated by yaw
        // must point at the target: yaw = atan2(-nx, nz)
        // wave2: the caller already computed d2 = ddx*ddx + ddz*ddz (the
        // SAME product bits — deterministic ops) — sqrt it here instead of
        // re-multiplying; bit-identical, two flops cheaper per moving mob.
        let len = d2.sqrt();
        if len > 1e-9 {
            let (nx, nz) = (ddx / len, ddz / len);
            self.yaw = (-(nx).atan2(nz) * (180.0f64 / std::f64::consts::PI)) as f32;
        }
    }
}

// ---------------------------------------------------------------------------
// tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// the goals.rs parity fixture's idle build (build_in) — the table
    /// PARITY #369 proved bit-identical, with the executor's follow/cfg2
    /// values patched in
    fn fixture_in(seed: u64, tick_count: i32, id: i32, no_action: i32) -> Vec<i32> {
        let mut b = vec![0i32; g::IN_HEADER + N_GOALS * g::GOAL_STRIDE];
        b[1] = MF_BASE;
        b[2] = tick_count;
        b[3] = id;
        b[4] = no_action;
        b[5] = 4;
        b[6] = 7;
        b[g::IN_SEED_LO] = seed as i32;
        b[g::IN_SEED_HI] = (seed >> 32) as i32;
        b[g::IN_FOLLOW] = FOLLOW_RANGE.to_bits() as i32;
        let t = zombie_table();
        for (i, rec) in t.iter().enumerate() {
            let o = g::IN_HEADER + i * g::GOAL_STRIDE;
            b[o] = rec.kind;
            b[o + 1] = rec.prio;
            b[o + 2] = rec.flags as i32;
            b[o + 3] = rec.cfg;
            b[o + 6] = rec.cfg2;
            b[o + 7] = rec.cfg3;
        }
        b
    }

    #[test]
    fn pack_matches_parity_fixture() {
        let mut m = MobAI::new(2, 777, 0.0, 0.0, 0.0);
        m.tick_count = 100;
        m.no_action = 42;
        let packed = m.pack(&[], 0, 0, 0, 0, 0.0, 0.0, 0.0);
        let mut want = fixture_in(777, 100, 2, 42);
        want[2] = 100;
        want[4] = 42;
        let put = |b: &mut Vec<i32>, idx: usize, v: f64| {
            let bits = v.to_bits();
            b[idx] = bits as i32;
            b[idx + 1] = (bits >> 32) as i32;
        };
        // the fixture leaves the mob pos/eye zeroed; the executor packs
        // the live values (mob at the origin, eye 1.62)
        put(&mut want, g::IN_MOBX, 0.0);
        put(&mut want, g::IN_MOBY, 0.0);
        put(&mut want, g::IN_MOBZ, 0.0);
        put(&mut want, g::IN_EYEY, EYE_Y);
        assert_eq!(packed.len(), want.len());
        for i in 0..want.len() {
            assert_eq!(
                packed[i], want[i],
                "pack word {i} diverges from the parity fixture"
            );
        }
    }

    #[test]
    fn pack_matches_parity_fixture_with_player() {
        // the goals.rs snapshot_one_player layout: player at (3,0,4)
        let mut m = MobAI::new(2, 777, 0.0, 0.0, 0.0);
        m.tick_count = 100;
        m.no_action = 42;
        let players = [PlayerSnap {
            x: 3.0,
            y: 0.0,
            z: 4.0,
            id: 42,
            flags: g::PF_ALIVE,
        }];
        let packed = m.pack(&players, 0, 0, 0, 0, 0.0, 0.0, 0.0);
        let mut want = fixture_in(777, 100, 2, 42);
        want[g::IN_PCOUNT] = 1;
        let pb = g::IN_HEADER + N_GOALS * g::GOAL_STRIDE;
        want.resize(pb + g::PSTRIDE, 0);
        assert_eq!(packed.len(), pb + g::PSTRIDE);
        let put = |b: &mut Vec<i32>, idx: usize, v: f64| {
            let bits = v.to_bits();
            b[idx] = bits as i32;
            b[idx + 1] = (bits >> 32) as i32;
        };
        put(&mut want, g::IN_MOBX, 0.0);
        put(&mut want, g::IN_MOBY, 0.0);
        put(&mut want, g::IN_MOBZ, 0.0);
        put(&mut want, g::IN_EYEY, EYE_Y);
        put(&mut want, pb, 3.0);
        put(&mut want, pb + 2, 0.0);
        put(&mut want, pb + 4, 4.0);
        put(&mut want, pb + 6, EYE_Y);
        put(&mut want, pb + 8, 1.0);
        want[pb + 10] = g::PF_ALIVE;
        want[pb + 11] = 42;
        for i in 0..want.len() {
            assert_eq!(
                packed[i], want[i],
                "pack+player word {i} diverges from the parity fixture"
            );
        }
    }

    /// a resolver that answers like the goals.rs unit drives: nothing
    /// resolves, nothing starts
    struct Zero;
    impl Resolver for Zero {
        fn stroll_pos(&mut self, _m: &mut MobAI) -> Option<(f64, f64, f64)> {
            None
        }
        fn los(&mut self, _n: usize) -> i32 {
            0
        }
        fn canuse_vanilla(&mut self, _m: &MobAI, _kind: i32) -> bool {
            false
        }
        fn cc_vanilla(&mut self, _m: &MobAI, _kind: i32) -> bool {
            false
        }
        fn start_vanilla(&mut self, _m: &mut MobAI, _gi: usize) {}
    }

    #[test]
    fn idle_tick_pulls_exactly_six_values() {
        // the goals.rs idle_even_tick contract through the EXECUTOR:
        // 3 × nextInt(10) + nextInt(60) + nextFloat + nextFloat
        let mut m = MobAI::new(2, 777, 0.0, 0.0, 0.0);
        m.tick_count = 99; // packs 100; (100 + 2) even, > 1
        m.no_action = 41; // packs 42
        let mut scratch = Vec::new();
        let mut out = vec![0i32; out_len()];
        let stat = m.ai_tick(&[], 0, &mut Zero, &mut scratch, &mut out, &mut PackState::default(), MobAI::tick_fp(&[]));
        assert_eq!(stat.status, g::ST_DONE, "stat {stat:?}");
        assert_eq!(stat.fallback, 0);
        let mut r = 777u64;
        for _ in 0..3 {
            lcg_next_int(&mut r, 10);
        }
        lcg_next_int(&mut r, 60);
        lcg_next(&mut r, 24);
        lcg_next(&mut r, 24);
        assert_eq!(m.seed, r, "the executor must consume exactly 6 pulls");
    }

    #[test]
    fn no_fallback_across_long_idle_runs() {
        // 300 mobs x 400 ticks of the idle stream: the protocol must
        // never fall back (a fallback would be a pack/resume bug)
        let mth = Mth::new();
        // wave2: ONE buffer pair reused across all 300 mobs (reuse is the
        // production shape — this doubles as a reuse-safety soak)
        let mut scratch = Vec::new();
        let mut out = vec![0i32; out_len()];
        for i in 0..300i64 {
            let mut m = MobAI::new(i + 1, 0x1000 + i as u64, i as f64, 0.0, -i as f64);
            m.tick_count = 99;
            for _ in 0..400 {
                let stat = m.ai_tick(&[], 0, &mut Zero, &mut scratch, &mut out, &mut PackState::default(), MobAI::tick_fp(&[]));
                assert_eq!(stat.status, g::ST_DONE, "mob {i} stat {stat:?}");
                m.move_tick(&mth);
            }
            assert!(
                m.x.is_finite() && m.z.is_finite(),
                "mob {i} physics diverged"
            );
        }
    }

    #[test]
    fn executor_is_deterministic() {
        let mth = Mth::new();
        let mut a = MobAI::new(7, 0xFEED, 1000.0, 64.0, 1000.0);
        let mut b = MobAI::new(7, 0xFEED, 1000.0, 64.0, 1000.0);
        let mut w1 = FlatWorld::new(0xBEEF);
        let mut w2 = FlatWorld::new(0xBEEF);
        let mut scratch = Vec::new();
        let mut out = vec![0i32; out_len()];
        for _ in 0..600 {
            a.ai_tick(&[], 0, &mut w1, &mut scratch, &mut out, &mut PackState::default(), MobAI::tick_fp(&[]));
            b.ai_tick(&[], 0, &mut w2, &mut scratch, &mut out, &mut PackState::default(), MobAI::tick_fp(&[]));
            a.move_tick(&mth);
            b.move_tick(&mth);
        }
        assert_eq!(a.x, b.x, "x diverged");
        assert_eq!(a.z, b.z, "z diverged");
        assert_eq!(a.seed, b.seed, "seed diverged");
        assert_eq!(a.nav_state, b.nav_state);
    }

    #[test]
    fn stroll_walks_to_target_and_stops() {
        // the stroll gate (nextInt(60) == 0) must pass within the
        // noActionTime < 100 window for SOME small seed; the mob then
        // walks to the scripted target (east +10) and the goal retires
        let mth = Mth::new();
        // resolver: fixed target 10 east of wherever the mob is
        struct East;
        impl Resolver for East {
            fn stroll_pos(&mut self, m: &mut MobAI) -> Option<(f64, f64, f64)> {
                Some((m.x + 10.0, m.y, m.z))
            }
            fn los(&mut self, _n: usize) -> i32 {
                0
            }
            fn canuse_vanilla(&mut self, _m: &MobAI, _kind: i32) -> bool {
                false
            }
            fn cc_vanilla(&mut self, _m: &MobAI, _kind: i32) -> bool {
                false
            }
            fn start_vanilla(&mut self, m: &mut MobAI, gi: usize) {
                // the FlatWorld stroll start semantics
                if m.goals[gi].kind == g::GK_STROLL {
                    m.nav_state = g::NAV_PATH_ACTIVE;
                    m.nav_x = m.pend_x;
                    m.nav_z = m.pend_z;
                    m.goals[gi].running = true;
                    m.no_action = 0;
                }
            }
        }
        let mut walked = false;
        let mut arrived = false;
        let mut scratch = Vec::new();
        let mut out = vec![0i32; out_len()];
        // seed scan: the first seed whose stroll gate fires early enough
        let mut mob: Option<MobAI> = None;
        for seed in 1..200u64 {
            let mut m = MobAI::new(1, seed, 0.0, 64.0, 0.0);
            for t in 0..90i32 {
                m.ai_tick(&[], t, &mut East, &mut scratch, &mut out, &mut PackState::default(), MobAI::tick_fp(&[]));
                if m.nav_state == g::NAV_PATH_ACTIVE {
                    mob = Some(m);
                    break;
                }
            }
            if mob.is_some() {
                break;
            }
        }
        let mut m = mob.expect("no seed in 1..200 passes the stroll gate");
        for _ in 0..600 {
            m.ai_tick(&[], 200, &mut East, &mut scratch, &mut out, &mut PackState::default(), MobAI::tick_fp(&[]));
            let (x0, z0) = (m.x, m.z);
            m.move_tick(&mth);
            if (m.x - x0).abs() + (m.z - z0).abs() > 1e-4 {
                walked = true;
            }
            if walked && !arrived && m.nav_state != g::NAV_PATH_ACTIVE {
                arrived = true;
                // arrival: within one tick-step of the target
                let dx = (m.x - m.nav_x).abs();
                assert!(dx < 1.0, "stopped {dx} short of the target");
                assert!(m.x > 8.5, "the east walk covered only {}", m.x);
            }
        }
        assert!(walked, "the mob never walked");
        assert!(arrived, "the stroll never retired");
        assert!(m.x > 8.5, "final x {}", m.x);
    }

    #[test]
    fn targeted_zombie_chases_the_player() {
        // a visible player 20 blocks away: the nearest gate stages a scan,
        // the LoS resolves, the target locks, the melee goal starts and
        // the mob closes the distance
        let mth = Mth::new();
        let mut m = MobAI::new(3, 12345, 0.0, 64.0, 0.0);
        m.tick_count = 99;
        let players = [PlayerSnap {
            x: 20.0,
            y: 64.0,
            z: 0.0,
            id: 42,
            flags: g::PF_ALIVE,
        }];
        let mut w = FlatWorld::new(0xC0FFEE);
        let mut saw_target = false;
        let mut lock_tick = 0i32;
        let mut d_start = 0.0f64;
        let mut scratch = Vec::new();
        let mut out = vec![0i32; out_len()];
        for tick in 0..400i32 {
            m.ai_tick(&players, tick, &mut w, &mut scratch, &mut out, &mut PackState::default(), MobAI::tick_fp(&players));
            m.move_tick(&mth);
            if m.target_player >= 0 {
                if !saw_target {
                    saw_target = true;
                    lock_tick = tick;
                    d_start = (m.approach_x - m.x).abs() + (m.approach_z - m.z).abs();
                } else if tick - lock_tick > 25 {
                    // the melee goal arms after the 20-tick canUse throttle
                    // (lastCanUseCheck) — well inside a 25-tick window
                    assert!(
                        m.goals[5].running || m.goals[6].running,
                        "target locked but no melee goal running (tick {tick})"
                    );
                }
            }
        }
        let d_end = (m.approach_x - m.x).abs() + (m.approach_z - m.z).abs();
        assert!(saw_target, "the player was never targeted");
        assert!(
            d_end < d_start * 0.5,
            "the chase did not close: {d_start} -> {d_end}"
        );
    }

    #[test]
    fn lcg_math_matches_engine_reference() {
        // the raw-seed stream the seed handoff carries (NO constructor
        // scramble — ai_floor reads SHARED_RANDOM.value directly):
        // from seed 42: nextInt(60), nextFloat, nextDouble — the goals.rs
        // Lcg recurrence computed by hand (python cross-check)
        let mut s2 = 42u64;
        let ni = lcg_next_int(&mut s2, 60);
        let nf = lcg_next(&mut s2, 24) as f32 / 16777216.0f32;
        let nd = lcg_next_f64(&mut s2);
        assert_eq!(ni, 6, "nextInt(60) from raw seed 42");
        assert_eq!(nf, 0.7016322f32, "nextFloat");
        assert_eq!(nd, 0.9207663615147803f64, "nextDouble");
        assert_eq!(s2, 149370390209998u64, "stream position after 4 advances");
    }

    /// THE wave2 reuse gate: a DIRTY shared out buffer (pre-filled with
    /// garbage by a different mob's ticks, never re-zeroed by the caller)
    /// must produce bit-identical results to the old fresh-zero-per-tick
    /// semantics. This pins the floor's write-discipline contract: the
    /// only words read before write are the fallback RNG handoff pair,
    /// and `ai_tick` resets exactly those (out[..OUT_RESET]).
    #[test]
    fn reused_out_buffer_matches_fresh_zero() {
        let mth = Mth::new();
        let mk = |seed: u64| -> MobAI {
            let mut m = MobAI::new(9, seed, 5.0, 64.0, -3.0);
            m.goals = zombie_table();
            m
        };
        let players = [PlayerSnap {
            x: 15.0,
            y: 64.0,
            z: 3.0,
            id: 1,
            flags: g::PF_ALIVE,
        }];
        // twin A: the OLD semantics — fresh zero out buffer every tick
        let mut a = mk(0xABCD_1234);
        // twin B: the NEW semantics — one dirty buffer, reused forever
        let mut b = mk(0xABCD_1234);
        let mut scratch = Vec::new();
        let mut dirty = vec![0x7BAD_C0DEi32; out_len() * 2]; // over-long + garbage
        for tick in 0..400i32 {
            let mut fresh = vec![0i32; out_len()];
            let sa = a.ai_tick(&players, tick, &mut FlatWorld::new(0xC0FFEE), &mut scratch, &mut fresh, &mut PackState::default(), MobAI::tick_fp(&players));
            let sb = b.ai_tick(&players, tick, &mut FlatWorld::new(0xC0FFEE), &mut scratch, &mut dirty, &mut PackState::default(), MobAI::tick_fp(&players));
            assert_eq!(sa.rounds, sb.rounds, "tick {tick}: rounds diverged");
            assert_eq!(sa.requests, sb.requests, "tick {tick}: requests diverged");
            assert_eq!(sa.status, sb.status, "tick {tick}: status diverged");
            assert_eq!(sa.fallback, sb.fallback, "tick {tick}: fallback diverged");
            a.move_tick(&mth);
            b.move_tick(&mth);
            // bit-compare the whole AI-visible state every tick
            assert_eq!(a.x.to_bits(), b.x.to_bits(), "tick {tick}: x diverged");
            assert_eq!(a.z.to_bits(), b.z.to_bits(), "tick {tick}: z diverged");
            assert_eq!(a.seed, b.seed, "tick {tick}: seed diverged");
            assert_eq!(a.yaw.to_bits(), b.yaw.to_bits(), "tick {tick}: yaw diverged");
            assert_eq!(a.nav_state, b.nav_state, "tick {tick}: nav diverged");
            assert_eq!(a.target_player, b.target_player, "tick {tick}: target diverged");
            for i in 0..N_GOALS {
                assert_eq!(a.goals[i].running, b.goals[i].running, "tick {tick}: goal {i} running");
                assert_eq!(a.goals[i].state, b.goals[i].state, "tick {tick}: goal {i} state");
            }
        }
    }

    /// THE wave4 gate: the resident pack (static half via fp, tick half
    /// via the game_time/pcount/player-hash stamp, ~46-word round-1) must
    /// be BIT-IDENTICAL to the wave3 full-pack-every-round semantics
    /// across a chased, strolling, looking population.
    #[test]
    fn resident_pack_matches_full_pack_twin() {
        let mth = Mth::new();
        let players = [PlayerSnap {
            x: 15.0,
            y: 64.0,
            z: 3.0,
            id: 1,
            flags: g::PF_ALIVE,
        }];
        let mut a = MobAI::new(9, 0xABCD_1234, 5.0, 64.0, -3.0);
        a.goals = zombie_table();
        let mut b = MobAI::new(9, 0xABCD_1234, 5.0, 64.0, -3.0);
        b.goals = zombie_table();
        let mut scratch_a = Vec::new();
        let mut scratch_b = Vec::new();
        let mut out_a = vec![0i32; out_len()];
        let mut out_b = vec![0i32; out_len()];
        let mut pa = PackState {
            fp: None,
            tick: None,
            full: true,
        };
        let mut pb = PackState::default();
        for tick in 0..400i32 {
            let sa = a.ai_tick(
                &players,
                tick,
                &mut FlatWorld::new(0xC0FFEE),
                &mut scratch_a,
                &mut out_a,
                &mut pa,
                MobAI::tick_fp(&players),
            );
            let sb = b.ai_tick(
                &players,
                tick,
                &mut FlatWorld::new(0xC0FFEE),
                &mut scratch_b,
                &mut out_b,
                &mut pb,
                MobAI::tick_fp(&players),
            );
            assert_eq!(sa.rounds, sb.rounds, "tick {tick}: rounds");
            assert_eq!(sa.status, sb.status, "tick {tick}: status");
            a.move_tick(&mth);
            b.move_tick(&mth);
            assert_eq!(a.x.to_bits(), b.x.to_bits(), "tick {tick}: x");
            assert_eq!(a.z.to_bits(), b.z.to_bits(), "tick {tick}: z");
            assert_eq!(a.seed, b.seed, "tick {tick}: seed");
            assert_eq!(a.yaw.to_bits(), b.yaw.to_bits(), "tick {tick}: yaw");
            assert_eq!(a.nav_state, b.nav_state, "tick {tick}: nav");
            assert_eq!(a.target_player, b.target_player, "tick {tick}: target");
            assert_eq!(a.lookat_player, b.lookat_player, "tick {tick}: lookat");
            for i in 0..N_GOALS {
                assert_eq!(a.goals[i].running, b.goals[i].running, "tick {tick}: goal {i} running");
                assert_eq!(a.goals[i].state, b.goals[i].state, "tick {tick}: goal {i} state");
            }
        }
        // the twin must actually exercise the live paths (starts, locks,
        // the target chain) — otherwise the gate proves nothing
        assert!(
            b.target_player >= 0 || b.goals.iter().any(|gr| gr.running),
            "twin never exercised a start/lock"
        );
    }

    /// The production scratch is SHARED across mobs: the resident path
    /// must rewrite every per-mob word (id/seed/pos/goal live state) when
    /// the scratch switches owners mid-tick. Two mobs alternating on ONE
    /// scratch vs each on its own — bit-equal outcomes.
    #[test]
    fn resident_pack_survives_mob_switching() {
        let players = [PlayerSnap {
            x: 8.0,
            y: 64.0,
            z: -2.0,
            id: 3,
            flags: g::PF_ALIVE,
        }];
        let mk = |id: i64| -> MobAI {
            let mut m = MobAI::new(id, 0x5EED_0000 + id as u64, id as f64 * 4.0, 64.0, id as f64);
            m.goals = zombie_table();
            m
        };
        // shared-scratch pair
        let (mut a1, mut b1) = (mk(1), mk(2));
        // per-scratch control pair
        let (mut a2, mut b2) = (mk(1), mk(2));
        let mut shared = Vec::new();
        let (mut sa2, mut sb2): (Vec<i32>, Vec<i32>) = (Vec::new(), Vec::new());
        let mut out = vec![0i32; out_len()];
        let mut pk_shared = PackState::default();
        let (mut pk_a2, mut pk_b2) = (PackState::default(), PackState::default());
        for tick in 0..300i32 {
            let st1 = a1.ai_tick(&players, tick, &mut FlatWorld::new(0xBEEF), &mut shared, &mut out, &mut pk_shared, MobAI::tick_fp(&players));
            let st1b = b1.ai_tick(&players, tick, &mut FlatWorld::new(0xBEEF), &mut shared, &mut out, &mut pk_shared, MobAI::tick_fp(&players));
            let _ = st1;
            let _ = st1b;
            let mut out2 = vec![0i32; out_len()];
            a2.ai_tick(&players, tick, &mut FlatWorld::new(0xBEEF), &mut sa2, &mut out2, &mut pk_a2, MobAI::tick_fp(&players));
            b2.ai_tick(&players, tick, &mut FlatWorld::new(0xBEEF), &mut sb2, &mut out2, &mut pk_b2, MobAI::tick_fp(&players));
            assert_eq!(a1.x.to_bits(), a2.x.to_bits(), "tick {tick}: mob A x");
            assert_eq!(a1.z.to_bits(), a2.z.to_bits(), "tick {tick}: mob A z");
            assert_eq!(a1.seed, a2.seed, "tick {tick}: mob A seed");
            assert_eq!(b1.x.to_bits(), b2.x.to_bits(), "tick {tick}: mob B x");
            assert_eq!(b1.z.to_bits(), b2.z.to_bits(), "tick {tick}: mob B z");
            assert_eq!(b1.seed, b2.seed, "tick {tick}: mob B seed");
        }
    }

    /// WAVE6 arithmetic gate: the folded travel constants equal the runtime
    /// functions BIT-FOR-BIT (the const-fn float ops are the same IEEE ops).
    #[test]
    fn travel_open_ground_consts_pin() {
        let fr = mobtravel::compute_modified_friction(0.6f32, 1.0f32);
        assert_eq!(OG_FRICTION, fr);
        assert_eq!(
            OG_FIS,
            mobtravel::get_friction_influenced_speed(true, fr, 0.23, 0.02)
        );
        assert_eq!(OG_GM, mobtravel::compute_modified_friction(0.91f32, 1.0f32));
        assert_eq!(OG_DRAG_I, mobtravel::compute_modified_friction(0.98f32, 1.0f32));
        assert_eq!(OG_H, fr * OG_GM);
        assert_eq!(OG_HF, OG_H as f64);
        assert_eq!(OG_GRAV_TAIL, (0.0f64 - 0.08f64) * (OG_DRAG_I as f64));
        // NOTE: the f32 chain returns 0.6f32 EXACTLY (1-(1-0.6) double-
        // rounding compensates), so the speed gate `friction > 0.6` is
        // FALSE (not >) and the speed arm is the plain 0.23.
        assert_eq!(OG_FRICTION, 0.6f32);
        assert_eq!(OG_FIS, 0.23f32);
    }

    /// WAVE6 stream gate: the floor's fold draws are the EXACT resolver
    /// draws — splitmix16 (FlatWorld flavor) and LCG-16 (WorldRes flavor),
    /// word-for-word over a seed sweep.
    #[test]
    fn fold_streams_match_resolver_math() {
        for s in 0..5000u64 {
            let (mut a, mut b) = (s ^ 0x9E37_79B9, s ^ 0x9E37_79B9);
            let va = (splitmix(&mut a) & 0xFFFF) as i32;
            let vb = g::stroll_draw_splitmix16(&mut b);
            assert_eq!(va, vb, "splitmix16 @ seed {s}");
            let (mut c, mut d) = (s.wrapping_mul(0x51CE_9D1B), s.wrapping_mul(0x51CE_9D1B));
            let vc = lcg_next(&mut c, 16);
            let vd = g::stroll_draw_lcg16(&mut d);
            assert_eq!(vc, vd, "lcg16 @ seed {s}");
        }
    }

    /// THE WAVE6 twin gate: the FOLDED executor (fold resolves in place,
    /// zero park round-trips) must be BIT-IDENTICAL to the PARKED executor
    /// (every request staged and resumed — the pre-wave6 semantics) over a
    /// chased, strolling, look-around population: positions, seeds, goal
    /// states, nav, target, rel — the whole mob, word for word.
    #[test]
    fn folded_ai_tick_matches_parked_ai_tick() {
        let players = vec![PlayerSnap {
            x: 3.0,
            y: 64.0,
            z: 4.0,
            id: 1,
            flags: 1,
        }];
        let mth = Mth::new();
        let run = |no_fold: bool| -> Vec<(u64, u64, u64, i32, i32, i32, u64, u64)> {
            let mut w = FlatWorld::new(0x9E37_79B9_7F4A);
            w.no_fold = no_fold;
            let mut m = MobAI::new(1, 0x1234_5678_9abc_def0, 0.0, 64.0, 0.0);
            m.goals = zombie_table();
            let mut scratch: Vec<i32> = Vec::new();
            let mut out = vec![0i32; out_len()];
            let mut pk = PackState::default();
            for t in 0..400i32 {
                let _ = m.ai_tick(
                    &players,
                    t,
                    &mut w,
                    &mut scratch,
                    &mut out,
                    &mut pk,
                    MobAI::tick_fp(&players),
                );
                m.move_tick(&mth);
            }
            let goals: Vec<(bool, i32)> = m.goals.iter().map(|gr| (gr.running, gr.state)).collect();
            let mut gbits = (0u64, 0u64);
            for (i, (r, s)) in goals.iter().enumerate() {
                let w = ((*r as u64) << 63) | ((*s as u32 as u64) << 32) | i as u64;
                if i < 6 {
                    gbits.0 ^= w.wrapping_mul(0x9E37_79B9_7F4A_7C15);
                } else {
                    gbits.1 ^= w.wrapping_mul(0x9E37_79B9_7F4A_7C15);
                }
            }
            vec![
                (m.x.to_bits(), m.z.to_bits(), m.seed, m.nav_state, m.target_player, m.no_action, gbits.0, gbits.1),
            ]
        };
        let parked = run(true);
        let folded = run(false);
        assert_eq!(
            parked, folded,
            "the folded executor must match the parked twin bit-for-bit"
        );
        // the scenario must actually exercise the folds (a started stroll or
        // lookaround moves the mob / flips goal words; a target is acquired)
        let (x_bits, _z, _seed, _nav, target, _na, g0, g1) = folded[0];
        let active = x_bits != 0u64 || target >= 0 || g0 != 0 || g1 != 0;
        assert!(active, "the twin scenario must exercise goal activity");
    }
}
