//! L111g differential fixture: the INLINE floor walk (`MobAI::ai_tick`) must
//! be DECISION-IDENTICAL to the parked protocol (`MobAI::ai_tick_parked`) —
//! same emitted actions (via the final state they produce), same seed out,
//! same per-goal state, same targets — over randomized scenarios.
//!
//! The randomization sweeps every request arm: LOS parks (players inside and
//! outside the follow range, some failing the flags), stroll gates (interval
//! 60), lookat gates (0.02), lookaround starts (0.02 — the drawing start
//! body), zombie-attack canUse (the 20-tick throttle + the target gate),
//! canContinueToUse stops (nav done / lookTime expired / range exceeded),
//! and the noActionTime>=100 stroll gate. Both 1-tick scenarios (isolate the
//! first divergence) and 40-tick chains (catch visibility cascades).

use centities_core::goals as g;
use centities_native::world::{OpenGround, PlayerTick, Snapshot, WorldRes};
use swarm_sim::mob::{MobAI, PlayerSnap, zombie_table};

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ z >> 31
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

const PF_ALIVE: i32 = 1;
const PF_SPECTATOR: i32 = 1 << 1;

fn rand_players(r: &mut Rng) -> (Vec<PlayerTick>, Vec<PlayerSnap>) {
    let n = r.below(4) as usize; // 0..3 players
    let mut ticks = Vec::with_capacity(n);
    let mut snaps = Vec::with_capacity(n);
    for i in 0..n {
        let p = PlayerTick {
            x: (r.below(80)) as f64 - 40.0 + (i as f64) * 0.31,
            y: 64.0,
            z: (r.below(80)) as f64 - 40.0,
            id: (100 + i) as i32,
            // mostly alive; sometimes spectator (exercising the flag gates)
            flags: if r.below(8) == 0 { PF_SPECTATOR } else { PF_ALIVE },
        };
        snaps.push(PlayerSnap {
            x: p.x,
            y: p.y,
            z: p.z,
            id: p.id,
            flags: p.flags,
        });
        ticks.push(p);
    }
    (ticks, snaps)
}

fn rand_mob(r: &mut Rng, seed: u64) -> MobAI {
    let mut m = MobAI::new(
        (r.below(10_000)) as i64,
        seed,
        (r.below(60)) as f64 - 30.0,
        64.0,
        (r.below(60)) as f64 - 30.0,
    );
    m.goals = zombie_table();
    m.tick_count = r.below(200) as i32;
    m.no_action = r.below(140) as i32; // sweeps the >=100 stroll gate
    m.nav_state = r.below(3) as i32;
    m.nav_x = (r.below(40)) as f64 - 20.0;
    m.nav_z = (r.below(40)) as f64 - 20.0;
    m.target_player = -1;
    m.lookat_player = -1;
    let gt = 20 + r.below(900) as i32;
    // running sets must be FLAG-COLLISION-FREE per selector (two goals on
    // one flag running at once = the FB_LOCK_COLLIDE robustness path —
    // unreachable for the classified table, and the parked protocol's
    // fallback seed handoff differs from the inline abort by construction)
    // target selector: HurtBy(FL_TARGET), Nearest x3 (FL_TARGET) — at most
    // one target goal running
    let tg_run = r.below(4);
    if tg_run != 0 {
        m.goals[(tg_run - 1) as usize].running = true;
    }
    // goal selector flag groups (the zombie table): SPEAR4/ZA5/ZA6/RLA10
    // hold FL_ML (bits 0+1); MTV7/STROLL8 hold FL_MOVE (bit 0); LOOKAT9
    // holds FL_LOOK (bit 1). Valid running sets: ONE ML goal alone, OR
    // (MTV|STROLL) with an optional LOOKAT, OR a single MTV/STROLL/LOOKAT.
    let pick = r.below(7);
    match pick {
        0 => {}
        1 => m.goals[4].running = true, // SPEAR
        2 => m.goals[5].running = true, // ZA1
        3 => m.goals[6].running = true, // ZA2
        4 => m.goals[10].running = true, // LOOKAROUND
        5 => {
            m.goals[7].running = true; // MTV (FL_MOVE)
            if r.below(2) == 1 {
                m.goals[9].running = true; // + LOOKAT (FL_LOOK)
            }
        }
        _ => {
            m.goals[8].running = true; // STROLL (FL_MOVE)
            if r.below(2) == 1 {
                m.goals[9].running = true; // + LOOKAT (FL_LOOK)
            }
        }
    }
    for gi in 0..m.goals.len() {
        if !m.goals[gi].running {
            m.goals[gi].state = 0;
            continue;
        }
        let kind = m.goals[gi].kind;
        // a plausible state word per kind
        m.goals[gi].state = match kind {
            g::GK_LOOKAROUND => r.below(30) as i32,
            g::GK_LOOKAT_PLAYER => {
                // alive bit + maybe range bit + lookTime
                let mut st = (r.below(40) as i32) | (1 << 31);
                if r.below(2) == 0 {
                    st |= 1 << 30;
                }
                st
            }
            g::GK_ZOMBIE_ATTACK => gt - r.below(50) as i32,
            g::GK_MOVE_THROUGH_VILLAGE => r.below(3) as i32,
            g::GK_HURT_BY => gt - r.below(3) as i32,
            _ => 0,
        };
    }
    // sometimes the mob already holds a target/lookat reference
    if r.below(3) == 0 {
        m.target_player = (r.below(4) as i32) - 1; // -1..2 (may be out of range)
        m.approach_x = (r.below(40)) as f64 - 20.0;
        m.approach_z = (r.below(40)) as f64 - 20.0;
    }
    if r.below(3) == 0 {
        m.lookat_player = (r.below(4) as i32) - 1;
    }
    m
}

/// The full state equality check (everything the tick touches).
fn assert_same(a: &MobAI, b: &MobAI, tag: &str) {
    // decision-visible state FIRST (the seed is the last thing to diverge
    // when the RNG streams split), seed last
    for i in 0..a.goals.len() {
        assert_eq!(
            a.goals[i].running, b.goals[i].running,
            "{tag}: goal[{i}].running"
        );
        assert_eq!(a.goals[i].state, b.goals[i].state, "{tag}: goal[{i}].state");
    }
    assert_eq!(a.target_player, b.target_player, "{tag}: target_player");
    assert_eq!(a.lookat_player, b.lookat_player, "{tag}: lookat_player");
    assert_eq!(a.nav_state, b.nav_state, "{tag}: nav_state");
    assert_eq!(a.no_action, b.no_action, "{tag}: no_action");
    assert_eq!(a.tick_count, b.tick_count, "{tag}: tick_count");
    assert_eq!(a.seed, b.seed, "{tag}: seed");
    assert_eq!(a.nav_x.to_bits(), b.nav_x.to_bits(), "{tag}: nav_x");
    assert_eq!(a.nav_z.to_bits(), b.nav_z.to_bits(), "{tag}: nav_z");
    assert_eq!(a.rel_x.to_bits(), b.rel_x.to_bits(), "{tag}: rel_x");
    assert_eq!(a.rel_z.to_bits(), b.rel_z.to_bits(), "{tag}: rel_z");
    assert_eq!(a.pend_x.to_bits(), b.pend_x.to_bits(), "{tag}: pend_x");
    assert_eq!(a.pend_z.to_bits(), b.pend_z.to_bits(), "{tag}: pend_z");
    assert_eq!(
        a.approach_x.to_bits(),
        b.approach_x.to_bits(),
        "{tag}: approach_x"
    );
    assert_eq!(
        a.approach_z.to_bits(),
        b.approach_z.to_bits(),
        "{tag}: approach_z"
    );
}

fn run_scenario(r: &mut Rng, ticks: i32) -> (u32, u32) {
    let seed = r.next();
    let gt0 = 20 + r.below(900) as i32;
    let (pticks, psnaps) = rand_players(r);
    let mut mob_parked = rand_mob(r, seed);
    let mut mob_inline =
        MobAI::new(mob_parked.id, seed, mob_parked.x, mob_parked.y, mob_parked.z);
    mob_inline.goals = mob_parked.goals;
    mob_inline.tick_count = mob_parked.tick_count;
    mob_inline.no_action = mob_parked.no_action;
    mob_inline.nav_state = mob_parked.nav_state;
    mob_inline.nav_x = mob_parked.nav_x;
    mob_inline.nav_z = mob_parked.nav_z;
    mob_inline.target_player = mob_parked.target_player;
    mob_inline.lookat_player = mob_parked.lookat_player;
    mob_inline.approach_x = mob_parked.approach_x;
    mob_inline.approach_z = mob_parked.approach_z;
    for gi in 0..mob_parked.goals.len() {
        mob_inline.goals[gi].running = mob_parked.goals[gi].running;
        mob_inline.goals[gi].state = mob_parked.goals[gi].state;
    }

    let snapshot = Snapshot {
        players: pticks.clone(),
    };
    let mut total_req_p = 0u32;
    let mut total_req_i = 0u32;
    for t in 0..ticks {
        let game_time = gt0 + t;
        // the executor's per-tick approach refresh (both sides identical)
        for m in [&mut mob_parked, &mut mob_inline] {
            if m.target_player >= 0 {
                match snapshot.players.get(m.target_player as usize) {
                    Some(p) => {
                        m.approach_x = p.x;
                        m.approach_z = p.z;
                    }
                    None => m.target_player = -1,
                }
            }
        }
        let mut buf_p: Vec<i32> = Vec::new();
        let mut buf_i: Vec<i32> = Vec::new();
        // two INDEPENDENT resolvers with the SAME stroll stream seed (the
        // streams must advance identically while the decisions match; any
        // decision divergence shifts the draws and the state check fires)
        let mut res_p = WorldRes::<OpenGround>::open_ground_v0(&snapshot, 0xABCD_1234);
        let mut res_i = WorldRes::<OpenGround>::open_ground_v0(&snapshot, 0xABCD_1234);
        let sp = mob_parked.ai_tick_parked(&psnaps, game_time, &mut res_p, &mut buf_p);
        let si = mob_inline.ai_tick(&psnaps, game_time, &mut res_i, &mut buf_i);
        total_req_p += sp.requests;
        total_req_i += si.requests;
        if sp.status != si.status || sp.fallback != si.fallback {
            panic!(
                "DIVERGENCE t={t} seed={seed:016x} gt={game_time}\n  players={:?}\n  running_p={:?} running_i={:?}\n  fb_p={} fb_i={} st_p={} st_i={}\n  state_p goal1={} state_i goal1={}",
                snapshot.players.iter().map(|p| (p.x, p.z, p.flags)).collect::<Vec<_>>(),
                mob_parked.goals.iter().enumerate().filter(|(_, gr)| gr.running).map(|(ii, _)| ii).collect::<Vec<_>>(),
                mob_inline.goals.iter().enumerate().filter(|(_, gr)| gr.running).map(|(ii, _)| ii).collect::<Vec<_>>(),
                sp.fallback, si.fallback, sp.status, si.status,
                mob_parked.goals[1].state, mob_inline.goals[1].state,
            );
        }
        assert_eq!(sp.fallback, si.fallback, "t={t}: fallback reason");
        // On ST_FALLBACK the floor's decision contract ENDS — the executor
        // routes the mob to the vanilla mirror and the engine's model stops
        // at the abort point (the parked protocol's out carries the last
        // park's seed, the inline abort carries none — both are executor-
        // defined post-abort shapes, not floor decisions). The FB paths are
        // unreachable for the classified table (the fixture's randomized
        // states cannot collide by construction either). Compare floor
        // decisions only for the non-fallback statuses.
        if sp.status != -1 {
            assert_same(&mob_parked, &mob_inline, &format!("t={t} seed={seed:016x}"));
        }
    }
    (total_req_p, total_req_i)
}

#[test]
fn inline_walk_is_decision_identical_single_tick() {
    let mut r = Rng(0x111D_A911_0001);
    let mut exercised = 0u32;
    for _ in 0..3000 {
        let (rp, ri) = run_scenario(&mut r, 1);
        assert_eq!(rp, ri, "request counts must match");
        exercised += rp;
    }
    assert!(
        exercised > 1500,
        "the fixture must exercise the resolution paths: got {exercised} resolves"
    );
}

#[test]
fn inline_walk_is_decision_identical_chained() {
    // 40-tick chains: any visibility cascade (mf_live/na_live shadows,
    // same-tick ZA starts, displaced stops) shows up as a state split
    let mut r = Rng(0x5EED_911C_0A10);
    for _ in 0..200 {
        run_scenario(&mut r, 40);
    }
}
