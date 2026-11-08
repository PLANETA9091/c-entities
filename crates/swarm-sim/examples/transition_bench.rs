//! TRANSITION BENCH — the boundary-crossing cost of the 10^9 layer.
//!
//! What a tick pays when players move:
//!   - steady walk (1 chunk): 17 in + 17 out chunk transitions
//!   - teleport storm: the full 289-in + 289-out worst case (radius 8)
//!   - ring fill: T0 admission for the hot square
//!
//! Run: cargo run --release -p swarm-sim --example transition_bench

use std::time::Instant;
use swarm_core::ChunkDelta;
use swarm_sim::{HotRing, SwarmEngine};

fn main() {
    let mut eng = SwarmEngine::new(0, 8);

    // far field: 1M chunks × 40 = 40M entities (the ledger, ~tens of MB)
    let mut batch = Vec::with_capacity(1_000_000);
    for cx in -500..500 {
        for cz in -500..500 {
            batch.push(ChunkDelta::new(cx, cz, 40));
        }
    }
    let t0 = Instant::now();
    eng.seed_far(&batch).unwrap();
    let total = eng.world_total();
    println!(
        "seed: 40M entities / 1M chunks in {:?} — ledger total {total}",
        t0.elapsed()
    );

    // ---- steady walk: 1-chunk steps (the common case) ------------------
    let mut worst = std::time::Duration::ZERO;
    let mut sum = std::time::Duration::ZERO;
    let n_steps = 400;
    for i in 0..n_steps {
        let t = Instant::now();
        let _c = eng.set_players(&[(i - 200, 0)]).unwrap();
        let dt = t.elapsed();
        sum += dt;
        if dt > worst {
            worst = dt;
        }
    }
    let avg = sum / n_steps as u32;
    println!("walk  {n_steps}×1-chunk (17in+17out, ~1360 ops): avg {avg:?}/step, worst {worst:?}");

    // ---- teleport storm: full square swap every jump -------------------
    let mut tp = Vec::new();
    for i in 0..25 {
        let t = Instant::now();
        let _c = eng.set_players(&[(i * 30 - 300, i * 17 - 200)]).unwrap();
        tp.push(t.elapsed());
    }
    let avg_tp = tp.iter().sum::<std::time::Duration>() / tp.len() as u32;
    let worst_tp = tp.iter().max().unwrap();
    println!("teleport 25×(289in+289out, ~23k ops): avg {avg_tp:?}, worst {worst_tp:?}");

    // ---- T0 ring fill (at the CURRENT player position) -----------------
    let last_pc = (24 * 30 - 300, 24 * 17 - 200);
    let mut ring = HotRing::new(8192);
    let t = Instant::now();
    let n = eng.fill_ring(&mut ring, last_pc, 16);
    println!(
        "ring fill {n}/8192 ents (33×33 chunks @ {last_pc:?}): {:?}",
        t.elapsed()
    );

    // ---- accounting sanity ----------------------------------------------
    assert_eq!(eng.world_total(), total, "the total never moves");
    let mat = eng.materialized();
    println!(
        "accounting: world total {total} = ledger {} + materialized {mat} — invariant held",
        eng.ledger.total()
    );

    // ---- the 20-TPS verdict ---------------------------------------------
    println!("--- 20 TPS verdict (50 ms budget) ---");
    println!(
        "steady walk: {avg:?} = {:.2}% of one 50 ms tick",
        avg.as_secs_f64() / 0.05 * 100.0
    );
    println!("worst teleport: {worst_tp:?} — a single spike, amortizable across ticks");
}
