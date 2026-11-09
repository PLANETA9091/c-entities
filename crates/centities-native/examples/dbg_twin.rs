//! WAVE6 debug: run the folded vs parked executor tick by tick and print
//! the FIRST diverging mob's full state at the first diverging tick.
use centities_native::NativeTick;
use centities_native::world::PlayerTick;
use swarm_sim::mob::{MobAI, zombie_table};

fn build(fold_off: bool) -> NativeTick {
    let mut nt = NativeTick::new(0xC0FFEE);
    nt.set_fold_off(fold_off);
    for i in 0..96 {
        let mut m = MobAI::new(
            1000 + i,
            0xC0FFEE ^ (i as u64).wrapping_mul(0x9E37),
            (i % 12) as f64 * 3.0,
            64.0,
            (i / 12) as f64 * 3.0,
        );
        m.goals = zombie_table();
        nt.add_mob(m);
    }
    nt.push_players(vec![PlayerTick {
        x: 12.0,
        y: 64.0,
        z: 4.0,
        id: 1,
        flags: 1,
    }]);
    nt
}


fn trace_mode(fold_off: bool, label: &str) {
    let mut nt = build(fold_off);
    for t in 0..3i32 {
        centities_core::goals::mob_ledger_arm(true);
        let _ = nt.tick(t);
    }
    if let Some(recs) = centities_core::goals::mob_ledger_take(false) {
        println!("== {label} mob-1023 serve chain @ tick2:");
        for r in &recs {
            let id = (r[0] & 0xFFFF_FFFF) as i64;
            let tc = (r[0] >> 32) as i64;
            if id == 1023 && tc <= 3 {
                let cursor = r[4] & 0x00FF_FFFF_FFFF_FFFF;
                let status = (r[4] >> 56) as i64;
                let kinds = (r[3] >> 32) as u32;
                let draws = (r[3] & 0xFFFF_FFFF) as u32;
                let evals = (r[6] >> 32) as u32;
                let running = (r[6] & 0xFFFF_FFFF) as u32;
                println!(
                    "  seq={} seed_in={:012x} seed_out={:012x} draws={} kinds={:09b} cursor=sel{} ph{} gi{} status={} evals={:09b} running={:09b}",
                    r[5], r[1], r[2], draws, kinds, (cursor >> 9) & 1, (cursor >> 6) & 7, cursor & 63, status, evals, running
                );
            }
        }
    }
    centities_core::goals::mob_ledger_arm(false);
}

fn main() {
    trace_mode(true, "PARKED");
    trace_mode(false, "FOLDED");
}
