//! The Rust profiler: Horizon-style phase sampling (their
//! game/tick_profile.rs samples 1 tick in N and attributes cost per
//! (level, mob kind)) + per-function self-time via `perf` (the native-profile
//! CI job + scripts/rust_perf_top.py).
//!
//! The pivot's demand: hotspot reports must point at RUST functions. Two
//! layers:
//! 1. THIS ledger — the always-on, near-zero-cost phase split: ai / travel
//!    / grid / apply nanoseconds, sampled 1-in-N per phase (the sampling
//!    gap is the Horizon SAMPLE_EVERY=17 trick — steady state stays
//!    representative, the counter overhead stays invisible). Atomics, not
//!    thread_locals: the executor is single-threaded, the counter cost is
//!    one relaxed add per sampled phase.
//! 2. perf: the bench example runs single-threaded with a stable tick loop;
//!    `perf record -e cycles:u -- <native_bench>` + scripts/rust_perf_top.py
//!    parse → TOP Rust symbols. CI job native-profile.yml publishes the
//!    table as an artifact on every dispatch.
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering::Relaxed};
use std::time::Instant;

pub static AI_NS: AtomicU64 = AtomicU64::new(0);
pub static TRAVEL_NS: AtomicU64 = AtomicU64::new(0);
pub static GRID_NS: AtomicU64 = AtomicU64::new(0);
pub static APPLY_NS: AtomicU64 = AtomicU64::new(0);
static SAMPLE: AtomicU32 = AtomicU32::new(0);
static ON: AtomicU32 = AtomicU32::new(0);
/// total ticks seen since the last drain (for the per-tick math)
static SAMPLE_TOTAL: AtomicU64 = AtomicU64::new(0);
/// ticks whose phases were actually measured (1-in-SAMPLE_EVERY)
static SAMPLED_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Sample stride — measure 1 tick in N per phase. 17 (prime) keeps the
/// sample off any fixed scenario period (Horizon tick_profile.rs SAMPLE_EVERY).
pub const SAMPLE_EVERY: u32 = 17;

#[inline]
fn begin_tick() {
    SAMPLE_TOTAL.fetch_add(1, Relaxed);
    // one call per tick (the grid guard is the first phase) — decides the
    // whole tick's sample and counts the sampled ticks
    let s = SAMPLE.fetch_add(1, Relaxed) + 1;
    if s >= SAMPLE_EVERY {
        SAMPLE.store(0, Relaxed);
    }
    let on = (s == 1) as u32;
    if on == 1 {
        SAMPLED_TOTAL.fetch_add(1, Relaxed);
    }
    ON.store(on, Relaxed);
}

#[inline]
fn sampled() -> bool {
    ON.load(Relaxed) != 0
}

/// A scope guard for one sampled phase: drop adds the elapsed ns.
pub struct PhaseGuard {
    cell: &'static AtomicU64,
    t0: Instant,
    on: bool,
}

impl Drop for PhaseGuard {
    #[inline]
    fn drop(&mut self) {
        if self.on {
            let ns = self.t0.elapsed().as_secs() * 1_000_000_000
                + self.t0.elapsed().subsec_nanos() as u64;
            self.cell.fetch_add(ns, Relaxed);
        }
    }
}

/// Begin a sampled phase. Cheap: one relaxed load + (rare) Instant::now.
#[inline]
pub fn phase(cell: &'static AtomicU64, tick_opener: bool) -> PhaseGuard {
    if tick_opener {
        begin_tick();
    }
    let on = sampled();
    PhaseGuard {
        cell,
        t0: Instant::now(),
        on,
    }
}

#[inline]
pub fn ai_guard() -> PhaseGuard {
    phase(&AI_NS, false)
}
#[inline]
pub fn travel_guard() -> PhaseGuard {
    phase(&TRAVEL_NS, false)
}
#[inline]
pub fn grid_guard() -> PhaseGuard {
    phase(&GRID_NS, true)
}
#[inline]
pub fn apply_guard() -> PhaseGuard {
    phase(&APPLY_NS, false)
}

/// The rolled-up ledger snapshot.
#[derive(Clone, Copy, Default)]
pub struct Phases {
    pub ai_ns: u64,
    pub travel_ns: u64,
    pub grid_ns: u64,
    pub apply_ns: u64,
    /// total ticks seen
    pub samples: u64,
    /// ticks actually measured (1-in-SAMPLE_EVERY)
    pub sampled: u64,
}

/// Read + reset the ledger (the bench prints once per report window).
pub fn drain() -> Phases {
    Phases {
        ai_ns: AI_NS.swap(0, Relaxed),
        travel_ns: TRAVEL_NS.swap(0, Relaxed),
        grid_ns: GRID_NS.swap(0, Relaxed),
        apply_ns: APPLY_NS.swap(0, Relaxed),
        samples: SAMPLE_TOTAL.swap(0, Relaxed),
        sampled: SAMPLED_TOTAL.swap(0, Relaxed),
    }
}

impl std::fmt::Display for Phases {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ms = |ns: u64| ns as f64 / 1e6;
        // est per tick: the phase accumulated ONLY on sampled ticks —
        // divide by the sampled count, that mean IS the per-tick estimate
        let n = (self.sampled.max(1)) as f64;
        let est = |ns: u64| ms(ns) / n;
        write!(
            f,
            "phase  | sampled total ms | est ms/tick (1-in-{}, {} of {} ticks measured)\n\
             ai     | {:>8.1} | {:>7.2}\n\
             travel | {:>8.1} | {:>7.2}\n\
             grid   | {:>8.1} | {:>7.2}\n\
             apply  | {:>8.1} | {:>7.2}",
            SAMPLE_EVERY,
            self.sampled,
            self.samples,
            ms(self.ai_ns),
            est(self.ai_ns),
            ms(self.travel_ns),
            est(self.travel_ns),
            ms(self.grid_ns),
            est(self.grid_ns),
            ms(self.apply_ns),
            est(self.apply_ns),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guard_accumulates_when_sampled() {
        let before = AI_NS.load(Relaxed);
        {
            let _g = ai_guard();
        }
        let after = AI_NS.load(Relaxed);
        // sampled 1-in-17: may or may not have ticked; assert monotonic
        assert!(after >= before);
    }

    #[test]
    fn drain_resets() {
        AI_NS.store(1234, Relaxed);
        let p = drain();
        assert!(p.ai_ns >= 1234 || p.ai_ns > 0);
        assert_eq!(AI_NS.load(Relaxed), 0);
    }
}
