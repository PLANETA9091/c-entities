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
    _kind: i32,
) -> i32 {
    with_exec(|e| match e {
        Some(e) => e.add(eid as i64, seed as u64, x, y, z) as i32,
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
                        let _ = ex.tick(game_time, &mut iv, &mut pv);
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
