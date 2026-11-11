//! The stress twin: the EXACT native_bench world (mobs-25k shape, scaled)
//! driven through TWO executors — parked (fold_off) vs folded — tick by
//! tick, dumping an fnv over the full store per tick. The first diverging
//! tick + mob localizes any fold that is not behavior-identical under the
//! bench's density (the 96-mob fixture twin can miss rare chained cases).
use centities_native::NativeTick;
use centities_native::world::PlayerTick;
use swarm_sim::mob::{MobAI, zombie_table};
use centities_core::goals as g;

fn build(fold_off: bool, n: usize, seed: u64) -> NativeTick {
    let mut nt = NativeTick::new(seed);
    nt.set_fold_off(fold_off);
    for i in 0..n {
        let x = (i % 250) as f64 * 0.6 - 75.0 + 0.05 * ((i / 7) % 3) as f64;
        let z = (i / 250) as f64 * 0.6 - 75.0;
        let mut m = MobAI::new(1000 + i as i64, seed ^ (i as u64).wrapping_mul(0x9E37), x, 64.0, z);
        m.goals = swarm_sim::mob::zombie_table_res();
        nt.add_mob(m);
    }
    nt.push_players(vec![PlayerTick {
        x: 0.0,
        y: 64.0,
        z: 0.0,
        id: 1,
        flags: 1,
    }]);
    nt
}

fn dump(nt: &NativeTick) -> (u64, u64, i64) {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    let mut chasing = 0u64;
    let mut first_div = -1i64;
    for s in &nt.store.mobs {
        if !s.alive {
            continue;
        }
        let m = &s.mob;
        if m.target_player >= 0 {
            chasing += 1;
        }
        for w in [
            m.id as u64,
            m.x.to_bits(),
            m.y.to_bits(),
            m.z.to_bits(),
            m.seed,
            m.yaw.to_bits() as u64,
            m.dx.to_bits(),
            m.dy.to_bits(),
            m.dz.to_bits(),
            m.nav_state as u64,
            m.target_player as u64,
            m.no_action as u64,
            m.tick_count as u64,
            m.rel_x.to_bits(),
            m.rel_z.to_bits(),
            m.nav_x.to_bits(),
            m.nav_z.to_bits(),
            m.pend_x.to_bits(),
            m.pend_z.to_bits(),
        ] {
            h = (h ^ w).wrapping_mul(0x1000_0000_01b3);
        }
        for g in &m.goals {
            h = (h ^ (g.running as u64)).wrapping_mul(0x1000_0000_01b3);
            h = (h ^ (g.state as u64)).wrapping_mul(0x1000_0000_01b3);
        }
    }
    (h, chasing, first_div)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = args.get(1).map_or(4000, |s| s.parse().unwrap_or(4000));
    let ticks: i32 = args.get(2).map_or(120, |s| s.parse().unwrap_or(120));
    let seed: u64 = args.get(3).map_or(0xC0FFEE, |s| s.parse().unwrap_or(0xC0FFEE));
    let mut parked = build(true, n, seed);
    let mut folded = build(false, n, seed);
    for t in 0..ticks {
        let sa = parked.tick(t).to_vec();
        let sb = folded.tick(t).to_vec();
        // round-cap + fallback accounting: the parked flow can hit the
        // 32-round cap (the fold exists to avoid the round-trips); every
        // cap hit truncates the walk — a chasing delta from the cap is an
        // artifact of the BENCH (no Java to cover the fallback), not a
        // fold semantic divergence.
        let caps_a: usize = sa.iter().filter(|s| s.status == g::ST_FALLBACK).count();
        let caps_b: usize = sb.iter().filter(|s| s.status == g::ST_FALLBACK).count();
        let (ha, ca, _) = dump(&parked);
        let (hb, cb, _) = dump(&folded);
        if ha != hb || ca != cb {
            println!(
                "DIVERGED at tick {t}: fnv {ha:016x} vs {hb:016x} | chasing {ca} vs {cb} | fallbacks {caps_a} vs {caps_b}"
            );
            // first differing mob
            for (a, b) in parked.store.mobs.iter().zip(folded.store.mobs.iter()) {
                if a.mob.target_player != b.mob.target_player
                    || a.mob.x.to_bits() != b.mob.x.to_bits()
                    || a.mob.goals.iter().map(|g| g.running as u8).collect::<Vec<_>>()
                        != b.mob.goals.iter().map(|g| g.running as u8).collect::<Vec<_>>()
                {
                    println!(
                        "  mob {}: parked tgt={} x={} goals=[{:?}] folded tgt={} x={} goals=[{:?}]",
                        a.mob.id,
                        a.mob.target_player,
                        a.mob.x,
                        a.mob.goals.iter().map(|g| (g.kind, g.running, g.state)).collect::<Vec<_>>(),
                        b.mob.target_player,
                        b.mob.x,
                        b.mob.goals.iter().map(|g| (g.kind, g.running, g.state)).collect::<Vec<_>>(),
                    );
                    break;
                }
            }
            return;
        }
    }
    let (_, ca, _) = dump(&parked);
    let (_, cb, _) = dump(&folded);
    println!("IDENTICAL through {ticks} ticks (chasing {ca} vs {cb})");
}
