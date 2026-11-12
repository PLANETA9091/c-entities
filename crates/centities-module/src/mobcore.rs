//! N2-M0: the wholesale native-mob wire — the dumb JNI shim over
//! `centities_native::MobBatch` (the JNI-free protocol, batch.rs: the
//! bit-identity twin against the direct executor lives THERE). One
//! persistent executor per server thread; ONE batch call per level tick.
//!
//! Exports (registered on HOOKS_CLASS next to the L22a per-mob wire):
//!   nativeMobInit(J)V        — the executor (the stroll-stream seed)
//!   nativeMobReset()V        — drop (world unload / dimension change)
//!   nativeMobAdd(IJDDDI)I    — (eid, seed, x, y, z, kind) -> population
//!   nativeMobRemove(J)I      — eid -> 1 known / 0 unknown / -1 no executor
//!   nativeMobPlayers([D[I)V  — [x,y,z]*P + [id,flags]*P (the snapshot)
//!   nativeMobBatch(I[J[D)I   — game_time + out arrays -> n filled, or
//!                              -population when the arrays are too small
//!                              (the tick ALWAYS happens; the host grows
//!                              the arrays and reads next tick)

use centities_native::MobBatch;
use jvmti_bindings::env::JniEnv;
use jvmti_bindings::sys::jni;
use std::cell::RefCell;
use std::sync::atomic::{AtomicI64, Ordering};

thread_local! {
    /// Server-thread confined (the entity loop runs on one thread; the
    /// L22a wire holds the same confinement assumption).
    static EXEC: RefCell<Option<MobBatch>> = const { RefCell::new(None) };
    static IDS_BUF: RefCell<Vec<i64>> = const { RefCell::new(Vec::new()) };
    static POS_BUF: RefCell<Vec<f64>> = const { RefCell::new(Vec::new()) };
    static XYZ_BUF: RefCell<Vec<f64>> = const { RefCell::new(Vec::new()) };
    static PF_BUF: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
}

/// The seed handed at init (i64::MIN = "no init yet"). The arm runs on the
/// module-init thread; the TICK runs on the server thread — the executor is
/// created LAZILY on the first native call of whichever thread it lands on
/// (in a live server that is exactly one thread, the entity loop).
static MOB_SEED: AtomicI64 = AtomicI64::new(i64::MIN);

fn with_exec<R>(f: impl FnOnce(Option<&mut MobBatch>) -> R) -> R {
    EXEC.with(|c| {
        let mut g = c.borrow_mut();
        if g.is_none() {
            let s = MOB_SEED.load(Ordering::Acquire);
            if s != i64::MIN {
                *g = Some(MobBatch::new(s as u64));
            }
        }
        f(g.as_mut())
    })
}

pub extern "system" fn n_mob_init(_env: *mut jni::JNIEnv, _cls: jni::jclass, seed: i64) {
    MOB_SEED.store(seed, Ordering::Release);
    EXEC.with(|c| {
        *c.borrow_mut() = Some(MobBatch::new(seed as u64));
    });
}

pub extern "system" fn n_mob_reset(_env: *mut jni::JNIEnv, _cls: jni::jclass) {
    MOB_SEED.store(i64::MIN, Ordering::Release);
    EXEC.with(|c| {
        *c.borrow_mut() = None;
    });
}

pub extern "system" fn n_mob_add(
    _env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    eid: i32,
    seed: i64,
    x: f64,
    y: f64,
    z: f64,
    kind: i32,
) -> i32 {
    with_exec(|e| match e {
        Some(e) => e.add_kind(eid as i64, seed as u64, x, y, z, kind) as i32,
        None => -1,
    })
}

pub extern "system" fn n_mob_remove(_env: *mut jni::JNIEnv, _cls: jni::jclass, eid: i64) -> i32 {
    with_exec(|e| match e {
        Some(e) => {
            if e.remove(eid) {
                1
            } else {
                0
            }
        }
        None => -1,
    })
}

pub extern "system" fn n_mob_players(
    env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    xyz: jni::jdoubleArray,
    pf: jni::jintArray,
) {
    unsafe {
        let e = JniEnv::from_raw(env);
        let raw = e.raw();
        let vt = *raw;
        let n_xyz = ((*vt).GetArrayLength)(raw, xyz) as usize;
        let n_pf = ((*vt).GetArrayLength)(raw, pf) as usize;
        if n_pf / 2 * 3 != n_xyz {
            // host-side stride bug — refuse the snapshot, keep the old one
            return;
        }
        let p = n_pf / 2;
        XYZ_BUF.with(|c| {
            PF_BUF.with(|oc| {
                let mut xv = c.borrow_mut();
                let mut pv = oc.borrow_mut();
                xv.resize(n_xyz, 0.0);
                pv.resize(n_pf, 0);
                if n_xyz > 0 {
                    ((*vt).GetDoubleArrayRegion)(raw, xyz, 0, n_xyz as i32, xv.as_mut_ptr());
                }
                if n_pf > 0 {
                    ((*vt).GetIntArrayRegion)(raw, pf, 0, n_pf as i32, pv.as_mut_ptr());
                }
                with_exec(|ex| {
                    if let Some(ex) = ex {
                        ex.set_players(&xv[..p * 3], &pv[..p * 2]);
                    }
                });
                if e.exception_check() {
                    e.exception_clear();
                }
            })
        });
    }
}

pub extern "system" fn n_mob_batch(
    env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    game_time: i32,
    difficulty: i32,
    ids: jni::jlongArray,
    pos: jni::jdoubleArray,
) -> i32 {
    unsafe {
        let e = JniEnv::from_raw(env);
        let raw = e.raw();
        let vt = *raw;
        let cap_ids = ((*vt).GetArrayLength)(raw, ids) as usize;
        let cap_pos = ((*vt).GetArrayLength)(raw, pos) as usize;
        // (population, read-back fit) — decided inside the borrows, the
        // Java array writes happen there too (raw/vt are plain pointers)
        let mut out: Option<(usize, bool)> = None;
        with_exec(|ex| {
            if let Some(ex) = ex {
                let pop = ex.population();
                IDS_BUF.with(|c| {
                    POS_BUF.with(|oc| {
                        let mut iv = c.borrow_mut();
                        let mut pv = oc.borrow_mut();
                        iv.resize(pop * 2, 0);
                        pv.resize(pop * 4, 0.0);
                        // the tick ALWAYS happens — the world advances; the
                        // read-back is pure and may lag one tick on a
                        // population jump past the host arrays
                        let _ = ex.tick(game_time, difficulty, &mut iv, &mut pv);
                        let fit = pop * 2 <= cap_ids && pop * 4 <= cap_pos;
                        if fit && pop > 0 {
                            ((*vt).SetLongArrayRegion)(raw, ids, 0, (pop * 2) as i32, iv.as_ptr());
                            ((*vt).SetDoubleArrayRegion)(
                                raw,
                                pos,
                                0,
                                (pop * 4) as i32,
                                pv.as_ptr(),
                            );
                        }
                        out = Some((pop, fit));
                    })
                });
            }
        });
        if e.exception_check() {
            e.exception_clear();
        }
        match out {
            Some((pop, true)) => pop as i32,
            Some((pop, false)) => -(pop as i32),
            None => -1,
        }
    }
}

// ----------------------------------------------------------------------
// N3-N1 (doc 61) phase-2a: the resolved despawn config table. The push
// rides the boundary ONCE per level (CeHooks.pushDespawnCfgOnce); the
// decision itself stays UNWIRED until phase-2b (the executor slot + the
// RNG order + the outbox). Thread-local like EXEC: the boundary and the
// future decision read it on the same server thread.
// ----------------------------------------------------------------------

thread_local! {
    static DESPAWN_CFG: RefCell<Option<(bool, centities_native::despawn::DespawnCfg)>> =
        const { RefCell::new(None) };
}

/// `(II[D[I)Z` — (ellipsoid, present-bitmask, (h2, ratio)*8*2, v*8*2) ->
/// true when the table landed. A refuse (cats > 8, bad strides) leaves
/// the previous state; the Java side logs DESPAWN_CFG_LIVE=false and the
/// M2c skip stays (the SAFE default).
pub extern "system" fn n_despawn_cfg(
    env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    ellipsoid: i32,
    present: i32,
    geom: jni::jdoubleArray,
    meta: jni::jintArray,
) -> u8 {
    unsafe {
        let e = JniEnv::from_raw(env);
        let raw = e.raw();
        let vt = *raw;
        let n_geom = ((*vt).GetArrayLength)(raw, geom) as usize;
        let n_meta = ((*vt).GetArrayLength)(raw, meta) as usize;
        let cats = n_meta / 2;
        let ok = cats <= 8
            && n_meta == cats * 2
            && n_geom == cats * 2 * 2
            && (cats == 0 || (cats * 2 * 2 == n_geom));
        if !ok {
            return 0;
        }
        let mut g = vec![0.0f64; n_geom];
        let mut m = vec![0i32; n_meta];
        if n_geom > 0 {
            ((*vt).GetDoubleArrayRegion)(raw, geom, 0, n_geom as i32, g.as_mut_ptr());
        }
        if n_meta > 0 {
            ((*vt).GetIntArrayRegion)(raw, meta, 0, n_meta as i32, m.as_mut_ptr());
        }
        let mut pairs = [None, None, None, None, None, None, None, None];
        for c in 0..cats {
            if present & (1 << c) == 0 {
                continue;
            }
            let mk = |slot: usize| centities_native::despawn::DespawnRange {
                h2: g[slot * 2],
                ratio: g[slot * 2 + 1],
                // the Java push resolves verticalLimit.or(default) into ONE
                // number — both fields carry it (should_despawn reads v)
                v_default: m[slot],
                v: m[slot],
            };
            pairs[c] = Some(centities_native::despawn::DespawnRangePair {
                hard: mk(c * 2),
                soft: mk(c * 2 + 1),
            });
        }
        let cfg = centities_native::despawn::DespawnCfg {
            ellipsoid: ellipsoid == 1,
            pairs,
        };
        // sanity: a present category must have FINITE geometry (a NaN
        // would poison the phase-2 decision — refuse the whole table)
        for c in 0..cats {
            if present & (1 << c) == 0 {
                continue;
            }
            for slot in [c * 2, c * 2 + 1] {
                if !g[slot * 2].is_finite() || !g[slot * 2 + 1].is_finite() {
                    return 0;
                }
            }
        }
        DESPAWN_CFG.with(|d| *d.borrow_mut() = Some((ellipsoid == 1, cfg.clone())));
        // N3-N1 phase-2b: the table lands on THIS thread's executor too —
        // the decision reads the batch's own copy (the push runs before
        // the first batch tick of the level that pushed it; a multi-level
        // boot re-pushes per level and the LAST push rides the single
        // executor — the M0 one-executor ledger, doc 56 §3)
        with_exec(|ex| {
            if let Some(ex) = ex {
                ex.set_despawn_cfg(Some(cfg));
            }
        });
        1
    }
}

/// Phase-2b read: the live config on the calling (server) thread.
pub fn despawn_cfg() -> Option<(bool, centities_native::despawn::DespawnCfg)> {
    DESPAWN_CFG.with(|d| d.borrow().as_ref().map(|(e, c)| (*e, c.clone())))
}
