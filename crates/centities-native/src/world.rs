//! The world seam: `WorldRes` — the Resolver the native executor runs
//! against. Player targeting/lookat/LoS resolve against the REAL player
//! snapshot the host pushes each tick; stroll targets and block safety go
//! through the `BlockFacts` seam (v0 = the open-ground contract; the
//! section-copy protocol plugs in here — doc 42 §6).
//!
//! Why a Resolver at all: the goal-selector floor (`centities_core::goals`
//! — bit-exact vs Paper 1.21.8, CI run #369) parks on world requests
//! instead of guessing them; the production Java executor resolves them in
//! `CeHooks.aiRun`. ZERO-Java means Rust resolves them: this file. The
//! request set is exactly the trait's five methods, plus the two optional
//! player-facing predicates (lookat/find-target) the floor parks on.
use swarm_sim::mob::{lcg_next, lcg_next_f64, lcg_next_int, Resolver};

/// Block facts seam. Horizon answers every block question with an O(1) row
/// over the block state (`BlockFacts`) — the pivot keeps that shape; v0
/// implements the open-ground contract (every position stands), v1 feeds it
/// from per-section copies the host pushes once per area per tick.
pub trait BlockFacts {
    /// "Can the mob stand at (x, floor_y, z)" — the path-type-0 contract
    /// (`path_type_static == 0` malus-free, Horizon path.rs:97).
    fn can_stand(&self, x: f64, _y: f64, z: f64) -> bool;
}

/// The v0 facts: open ground everywhere (the swarm bench contract).
pub struct OpenGround;

impl BlockFacts for OpenGround {
    #[inline]
    fn can_stand(&self, _x: f64, _y: f64, _z: f64) -> bool {
        true
    }
}

/// The world resolver over the live snapshot. NOT Clone — owns the
/// splitmix target stream (the stroll draw stream is deliberately NOT the
/// mob's RNG — swarm-sim honesty note 1 — and v0 keeps that split; the
/// vanilla stroll start body's own draws are consumed from the mob's
/// stream by the floor).
pub struct WorldRes<'a, F: BlockFacts> {
    /// the host-pushed per-tick state (players) — the ONLY world input the
    /// AI floor's player-facing requests read in v0
    pub snapshot: &'a Snapshot,
    pub facts: F,
    /// separate target stream
    pub target_rng: u64,
    pub stroll_radius: f64,
    /// follow range for the target/attack contracts (the zombie default the
    /// parity legs used)
    pub follow_range: f64,
}

impl WorldRes<'_, OpenGround> {
    /// Named v0 constructor — the open-ground contract (doc 42 §8): stroll
    /// targets splitmix-scripted, LoS always true. N1 replaces the facts.
    pub fn open_ground_v0(snapshot: &Snapshot, target_seed: u64) -> WorldRes<'_, OpenGround> {
        WorldRes {
            snapshot,
            facts: OpenGround,
            target_rng: target_seed,
            stroll_radius: 10.0,
            follow_range: 35.0,
        }
    }
}

impl<'a, F: BlockFacts> WorldRes<'a, F> {
    /// Generic constructor for the N1 facts (the section-copy protocol
    /// plugs its BlockFacts impl here).
    pub fn new_with_facts(snapshot: &'a Snapshot, facts: F, target_seed: u64) -> Self {
        WorldRes {
            snapshot,
            facts,
            target_rng: target_seed,
            stroll_radius: 10.0,
            follow_range: 35.0,
        }
    }

    /// The nearest alive player within the follow range (index or -1).
    /// A linear walk over P is the Horizon `players()` default contract
    /// (getNearestPlayer is O(P), never a grid scan — the same reason our
    /// Player-lane exists).
    #[inline]
    fn nearest_player_in_range(&self, x: f64, z: f64) -> i32 {
        let r2 = self.follow_range * self.follow_range;
        let players: &[PlayerTick] = &self.snapshot.players;
        let mut best = -1i32;
        let mut best_d2 = r2;
        for (i, p) in players.iter().enumerate() {
            if p.flags & 1 == 0 {
                continue; // !alive
            }
            let dx = p.x - x;
            let dz = p.z - z;
            let d2 = dx * dx + dz * dz;
            if d2 <= best_d2 {
                best_d2 = d2;
                best = i as i32;
            }
        }
        best
    }
}

impl<'a, F: BlockFacts> Resolver for WorldRes<'a, F> {
    fn stroll_pos(&mut self, mob: &mut swarm_sim::mob::MobAI) -> Option<(f64, f64, f64)> {
        // v0: the splitmix circle draw, gated by BlockFacts. The vanilla
        // body (WaterAvoidingRandomStrollGoal) draws 10x7 attempts with
        // block-safety per attempt; v0 draws ONE and tests the seam. When
        // the seam rejects, the goal fails its canUse honestly (res 0) —
        // the floor continues with the next goal.
        let r = self.stroll_radius;
        let dx = ((lcg_next(&mut self.target_rng, 16) as f64) / 65535.0) * 2.0 * r - r;
        let dz = ((lcg_next(&mut self.target_rng, 16) as f64) / 65535.0) * 2.0 * r - r;
        let (x, z) = (mob.x + dx, mob.z + dz);
        if self.facts.can_stand(x, mob.y, z) {
            Some((x, mob.y, z))
        } else {
            None
        }
    }

    fn los(&mut self, n_cands: usize) -> i32 {
        // v0 open ground: every staged candidate passes (the swarm
        // contract). v1: DDA clear_between against the section copies.
        if n_cands >= 32 {
            -1
        } else {
            (1i32 << n_cands) - 1
        }
    }

    fn canuse_vanilla(&mut self, mob: &swarm_sim::mob::MobAI, kind: i32) -> bool {
        // the classified zombie set parks exactly {ZombieAttack}: alive
        // target + the pathfinding half. v0: target within follow range.
        matches!(kind, centities_core::goals::GK_ZOMBIE_ATTACK)
            && mob.target_player >= 0
            && mob.target_player < self.snapshot.players.len() as i32
            && self.nearest_player_in_range(mob.x, mob.z) == mob.target_player
    }

    fn cc_vanilla(&mut self, mob: &swarm_sim::mob::MobAI, kind: i32) -> bool {
        if mob.target_player < 0 {
            return false;
        }
        let (tx, tz) = (mob.approach_x, mob.approach_z);
        let (dx, dz) = (tx - mob.x, tz - mob.z);
        match kind {
            centities_core::goals::GK_NEAREST_TARGET => {
                dx * dx + dz * dz <= self.follow_range * self.follow_range
            }
            // MeleeAttack chases (the open-ground straight line is the
            // proven path)
            _ => true,
        }
    }

    fn lookat_query(&mut self, mob: &swarm_sim::mob::MobAI, _players: &[swarm_sim::mob::PlayerSnap]) -> bool {
        // vanilla getNearestPlayer contract, resolved natively: any alive,
        // non-spectator player within the look distance. The floor picks
        // the exact candidate (bit-exact distance/tie logic); the resolver
        // answers the conditions. v0: LoS always true (open ground).
        // PF_ALIVE=1, PF_SPECTATOR=1<<1, PF_INVISIBLE=1<<3 (goals.rs).
        let _ = mob;
        self.snapshot.players.iter().any(|p| {
            p.flags & 1 != 0 && (p.flags & (1 << 1)) == 0 && (p.flags & (1 << 3)) == 0
        })
    }

    fn find_target(&mut self, mob: &swarm_sim::mob::MobAI, _players: &[swarm_sim::mob::PlayerSnap]) -> bool {
        // NearestAttackableTargetGoal.findTarget conditions, v0: an alive,
        // targetable player within the follow range (LoS open-ground).
        let _ = mob;
        self.snapshot
            .players
            .iter()
            .any(|p| p.flags & 1 != 0 && (p.flags & (1 << 1)) == 0)
    }

    fn start_vanilla(&mut self, mob: &mut swarm_sim::mob::MobAI, gi: usize) {
        use centities_core::goals as g;
        match mob.goals[gi].kind {
            g::GK_STROLL => {
                mob.nav_state = g::NAV_PATH_ACTIVE;
                mob.nav_x = mob.pend_x;
                mob.nav_z = mob.pend_z;
                mob.goals[gi].running = true;
                mob.no_action = 0;
            }
            g::GK_LOOKAROUND => {
                // the DRAW-EXACT RandomLookAround start (javap 26.3)
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

/// Per-tick host push: the player snapshot + game time. The Java side
/// becomes a COPY layer (doc 42 §5): players, add/remove diffs, block
/// subscriptions — never entity logic.
#[derive(Clone, Default)]
pub struct PlayerTick {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub id: i32,
    pub flags: i32,
}

/// The store's per-tick host-pushed state (players ride separately from the
/// mobs so the AI pack can borrow it immutably while the mobs mutate).
#[derive(Clone, Default)]
pub struct Snapshot {
    pub players: Vec<PlayerTick>,
}
