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
/// WAVE10: the odd-tick no-op skip (the maintained every-tick bit). ON by
/// default; CE_NO_ODD_SKIP=1 / set_odd_skip(false) restores the full floor
/// (the A/B arm + the twin's B side).
static ODD_SKIP: AtomicBool = AtomicBool::new(true);

#[inline]
pub fn odd_skip_on() -> bool {
    ODD_SKIP.load(Relaxed)
}

pub fn set_odd_skip(on: bool) {
    ODD_SKIP.store(on, Relaxed);
}
/// sub-phase sampling stride (per mob, within sampled ticks only)
pub const SUB_EVERY: u32 = 17;

/// The host's sampled-tick switch (1-in-SAMPLE_EVERY in the native
/// executor). Off everywhere else — the counters then stay zero.
/// WAVE5: an OFF switch resets SUB — begin_mob's zero-write fast path
/// leaves it stale, and a stale true would make the guards pay
/// Instant::now on UNSAMPLED mobs (the meter's shadow, wave2 lesson).
#[inline]
pub fn set_on(on: bool) {
    ON.store(on, Relaxed);
    if !on {
        SUB.store(false, Relaxed);
    }
}

/// Called ONCE per mob at ai_tick entry: picks whether THIS mob's
/// sub-phases are clocked. Returns the same decision (for callers that
/// want it); the guards read the SUB flag (one relaxed load).
/// WAVE5: the DISARMED fast path pays ONE relaxed load and ZERO writes —
/// the old shape ran a locked fetch_add + a store PER MOB PER TICK
/// (~25k locked RMW + 25k stores per tick) to count samples that would
/// never be taken (ON is true 1 tick in 17). The counters now move only
/// inside a sampled tick; set_on resets SUB so the guards never fire on
/// an unsampled tick's stale flag.
#[inline]
pub fn begin_mob() -> bool {
    if !ON.load(Relaxed) {
        return false;
    }
    let sub = MOB_N.fetch_add(1, Relaxed) % SUB_EVERY == 0;
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
