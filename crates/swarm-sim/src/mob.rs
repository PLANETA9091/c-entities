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
}

impl MobAI {
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
}

/// The flat open-ground world: nothing occludes, stroll targets are
/// splitmix-scripted, melee approaches while within 16 blocks.
pub struct FlatWorld {
    /// separate target stream (NOT the mob's RNG — see honesty note 1)
    pub target_rng: u64,
    pub stroll_radius: f64,
}

impl FlatWorld {
    pub fn new(target_seed: u64) -> Self {
        FlatWorld {
            target_rng: target_seed,
            stroll_radius: 10.0,
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

const OUT_LEN: usize = g::OUT_HEADER + g::MAX_ACTS * g::OUT_ACT;

impl MobAI {
    /// Pack the aiPack wire format from the live state.
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
        let mut b =
            vec![0i32; g::IN_HEADER + N_GOALS * g::GOAL_STRIDE + players.len() * g::PSTRIDE];
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
        put_f64(&mut b, g::IN_RES_X, rx);
        put_f64(&mut b, g::IN_RES_X + 2, ry);
        put_f64(&mut b, g::IN_RES_X + 4, rz);
        b[22] = 0; // lastHurtByMobTimestamp (the bench mob is never hurt)
        b[23] = game_time;
        b[g::IN_PCOUNT] = players.len() as i32;
        b[g::IN_FOLLOW] = FOLLOW_RANGE.to_bits() as i32;
        put_f64(&mut b, g::IN_MOBX, self.x);
        put_f64(&mut b, g::IN_MOBY, self.y);
        put_f64(&mut b, g::IN_MOBZ, self.z);
        put_f64(&mut b, g::IN_EYEY, self.y + EYE_Y);
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
                g::GK_LOOKAT_PLAYER => {
                    let mut st = (rec.state as u32 & 0x3FFF_FFFF) as i32;
                    if self.lookat_player >= 0 {
                        // alive bit + within-range bit (the open ground:
                        // every player is alive; range = lookDistance)
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
                _ => 0,
            };
            b[o + 6] = rec.cfg2;
            b[o + 7] = rec.cfg3;
        }
        // player snapshot records (level.players() order)
        let pb = base + N_GOALS * g::GOAL_STRIDE;
        for (i, p) in players.iter().enumerate() {
            let o = pb + i * g::PSTRIDE;
            put_f64(&mut b, o, p.x);
            put_f64(&mut b, o + 2, p.y);
            put_f64(&mut b, o + 4, p.z);
            put_f64(&mut b, o + 6, p.y + EYE_Y);
            put_f64(&mut b, o + 8, 1.0); // visibility 100%
            b[o + 10] = p.flags;
            b[o + 11] = p.id;
        }
        b
    }

    /// Apply the emitted actions to the live state (the aiApply mirror).
    fn apply_actions(&mut self, out: &[i32], players: &[PlayerSnap]) {
        let n = out[5].max(0) as usize;
        for a in 0..n {
            let b = g::OUT_HEADER + a * g::OUT_ACT;
            let op = out[b];
            let p0 = out[b + 1] as usize;
            let p1 = out[b + 2];
            match op {
                g::ACT_GOAL_START => {
                    if p0 < N_GOALS {
                        self.goals[p0].running = true;
                        self.no_action = 0;
                        // the start BODIES the executor runs itself (the
                        // engine emitted the transition, vanilla runs
                        // start() inline — aiApply: lockWrite + wg.start()):
                        match self.goals[p0].kind {
                            // LookAtPlayerGoal.start(): lookTime =
                            // adjustedTickDelay(40), no RNG (javap 26.3)
                            g::GK_LOOKAT_PLAYER => {
                                self.goals[p0].state = 40;
                            }
                            // RandomStrollGoal.start(): navigation.moveTo(
                            //   wantedPos, speedModifier) — the position the
                            //   REQ_STROLL_POS resolution stored on the goal
                            g::GK_STROLL => {
                                self.nav_state = g::NAV_PATH_ACTIVE;
                                self.nav_x = self.pend_x;
                                self.nav_z = self.pend_z;
                            }
                            _ => {}
                        }
                    }
                }
                g::ACT_GOAL_LOCK_MAP => { /* map-only; running flips at start */ }
                g::ACT_GOAL_STOP | g::ACT_GOAL_STOP_D => {
                    if p0 < N_GOALS {
                        self.goals[p0].running = false;
                        if self.goals[p0].kind == g::GK_STROLL
                            && self.nav_state == g::NAV_PATH_ACTIVE
                        {
                            self.nav_state = g::NAV_NONE;
                        }
                    }
                }
                g::ACT_GOAL_TICK_V => { /* the virtual melee body (scripted approach below) */ }
                g::ACT_LOOK_TICK_AROUND => {
                    if p0 < N_GOALS {
                        self.goals[p0].state = p1;
                    }
                }
                g::ACT_LOOK_TICK_AT => {
                    if p0 < N_GOALS {
                        self.goals[p0].state = p1;
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
                            }
                            g::GK_NEAREST_TARGET => {
                                self.target_player = p1;
                                if p1 >= 0 {
                                    if let Some(p) = players.get(p1 as usize) {
                                        self.approach_x = p.x;
                                        self.approach_z = p.z;
                                    }
                                }
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
    pub fn ai_tick(
        &mut self,
        players: &[PlayerSnap],
        game_time: i32,
        r: &mut dyn Resolver,
    ) -> AiStat {
        self.tick_count = self.tick_count.wrapping_add(1);
        self.no_action = self.no_action.saturating_add(1);
        let mut stat = AiStat::default();
        let (mut cursor, mut res, mut req_op) = (0i32, 0i32, 0i32);
        let (mut rx, mut ry, mut rz) = (0f64, 0f64, 0f64);
        let mut out = [0i32; OUT_LEN];
        for _round in 0..32 {
            let inb = self.pack(players, game_time, cursor, res, req_op, rx, ry, rz);
            let st = g::ai_floor(&inb, &mut out);
            stat.rounds += 1;
            // seed handoff (aiRun: out -> SHARED_RANDOM after EVERY round)
            self.seed = ((out[4] as u64) << 32) | (out[3] as u64 & 0xFFFF_FFFF);
            self.apply_actions(&out, players);
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
            (res, rx, ry, rz) = match req_op {
                g::REQ_LOS_SCAN => {
                    let n = out[8].max(0) as usize;
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
                self.face(ddx, ddz);
                zza = 0.98;
            }
        } else if self.nav_state == g::NAV_PATH_ACTIVE {
            let (ddx, ddz) = (self.nav_x - self.x, self.nav_z - self.z);
            let d2 = ddx * ddx + ddz * ddz;
            if d2 < 0.25 {
                self.nav_state = g::NAV_PATH_DONE;
            } else {
                self.face(ddx, ddz);
                zza = 0.98;
            }
        }
        // the travel slice (open-ground routing): friction 0.6, fm 1.0
        let friction = mobtravel::compute_modified_friction(0.6f32, 1.0f32);
        let fis = mobtravel::get_friction_influenced_speed(true, friction, 0.23, 0.02);
        let (ix, _iy, iz) = mobtravel::get_input_vector(mth, 0.0, 0.0, zza as f64, fis, self.yaw);
        let (mx, mz) = (self.dx + ix, self.dz + iz);
        // move consumes the pre-delta; the open ground clips nothing
        // horizontally; the setPos condition is the exact 26.3 predicate
        if mobtravel::move_sets_position((mx, self.dy, mz)) {
            self.x += mx;
            self.z += mz;
        }
        // the drag tail (the same slice travel_in_air_open_ground runs;
        // dy = (0 - gravity) * i, dm.y was ground-clipped to 0)
        let dm = (mx, 0.0f64, mz);
        let d = dm.1 - 0.08f64;
        let gm = mobtravel::compute_modified_friction(0.91f32, 1.0f32);
        let h = friction * gm;
        let i = mobtravel::compute_modified_friction(0.98f32, 1.0f32);
        self.dx = dm.0 * h as f64;
        self.dy = d * i as f64;
        self.dz = dm.2 * h as f64;
    }

    #[inline]
    fn face(&mut self, ddx: f64, ddz: f64) {
        // vanilla yaw: 0 = +Z, clockwise; input (0,0,zza) rotated by yaw
        // must point at the target: yaw = atan2(-nx, nz)
        let len = (ddx * ddx + ddz * ddz).sqrt();
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
        let stat = m.ai_tick(&[], 0, &mut Zero);
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
        for i in 0..300i64 {
            let mut m = MobAI::new(i + 1, 0x1000 + i as u64, i as f64, 0.0, -i as f64);
            m.tick_count = 99;
            for _ in 0..400 {
                let stat = m.ai_tick(&[], 0, &mut Zero);
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
        for _ in 0..600 {
            a.ai_tick(&[], 0, &mut w1);
            b.ai_tick(&[], 0, &mut w2);
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
        // seed scan: the first seed whose stroll gate fires early enough
        let mut mob: Option<MobAI> = None;
        for seed in 1..200u64 {
            let mut m = MobAI::new(1, seed, 0.0, 64.0, 0.0);
            for t in 0..90i32 {
                m.ai_tick(&[], t, &mut East);
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
            m.ai_tick(&[], 200, &mut East);
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
        for tick in 0..400i32 {
            m.ai_tick(&players, tick, &mut w);
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
}
