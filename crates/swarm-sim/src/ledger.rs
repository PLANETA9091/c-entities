//! swarm-sim::ledger — the wave2 AI sub-phase counters (pack / floor /
//! resolve / acts). The executor (centities-native) turns them on 1-in-17
//! ticks through `set_on`; standalone swarm users never do, so the guard
//! cost off-sampled is ONE relaxed bool load.
//!
//! This is the no-perf attribution layer: the phase ledger (profile.rs)
//! says "ai is hot"; THIS ledger says which HALF of ai (the wire pack or
//! the selector walk or the world answers) — the strike target for the
//! next arithmetic wave.
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering::Relaxed};
use std::time::Instant;

pub static PACK_NS: AtomicU64 = AtomicU64::new(0);
pub static FLOOR_NS: AtomicU64 = AtomicU64::new(0);
pub static RESOLVE_NS: AtomicU64 = AtomicU64::new(0);
pub static ACTS_NS: AtomicU64 = AtomicU64::new(0);
static ON: AtomicBool = AtomicBool::new(false);
/// WAVE2 tail fix: sub-phase clocks run 1-in-SUB_EVERY MOBS inside a
/// sampled tick (1-in-289 mob-ticks total). Measuring every mob of a
/// sampled tick cost ~240k Instant::now calls on that ONE tick — it
/// inflated the bench p99 from 11 to 22 ms purely by clock reads. The
/// SUB flag is the per-mob gate the four guards check.
static SUB: AtomicBool = AtomicBool::new(false);
static MOB_N: AtomicU32 = AtomicU32::new(0);
/// sub-phase sampling stride (per mob, within sampled ticks only)
pub const SUB_EVERY: u32 = 17;

/// The host's sampled-tick switch (1-in-SAMPLE_EVERY in the native
/// executor). Off everywhere else — the counters then stay zero.
#[inline]
pub fn set_on(on: bool) {
    ON.store(on, Relaxed);
}

/// Called ONCE per mob at ai_tick entry: picks whether THIS mob's
/// sub-phases are clocked. Returns the same decision (for callers that
/// want it); the guards read the SUB flag (one relaxed load).
#[inline]
pub fn begin_mob() -> bool {
    let sub = ON.load(Relaxed) && MOB_N.fetch_add(1, Relaxed) % SUB_EVERY == 0;
    SUB.store(sub, Relaxed);
    sub
}

/// Drain the four sub-phase cells (nanoseconds accumulated on sampled
/// ticks only) and reset them.
pub fn drain() -> (u64, u64, u64, u64) {
    (
        PACK_NS.swap(0, Relaxed),
        FLOOR_NS.swap(0, Relaxed),
        RESOLVE_NS.swap(0, Relaxed),
        ACTS_NS.swap(0, Relaxed),
    )
}

pub struct SubGuard {
    cell: &'static AtomicU64,
    t0: Option<Instant>,
}

impl Drop for SubGuard {
    #[inline]
    fn drop(&mut self) {
        if let Some(t0) = self.t0.take() {
            let ns = t0.elapsed().as_secs() * 1_000_000_000 + t0.elapsed().subsec_nanos() as u64;
            self.cell.fetch_add(ns, Relaxed);
        }
    }
}

#[inline]
fn sub(cell: &'static AtomicU64) -> SubGuard {
    SubGuard {
        cell,
        t0: if SUB.load(Relaxed) {
            Some(Instant::now())
        } else {
            None
        },
    }
}

#[inline]
pub fn sg_pack() -> SubGuard {
    sub(&PACK_NS)
}
#[inline]
pub fn sg_floor() -> SubGuard {
    sub(&FLOOR_NS)
}
#[inline]
pub fn sg_resolve() -> SubGuard {
    sub(&RESOLVE_NS)
}
#[inline]
pub fn sg_acts() -> SubGuard {
    sub(&ACTS_NS)
}
