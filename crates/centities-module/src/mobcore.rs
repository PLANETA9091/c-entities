//! N2-M0: the wholesale native-mob wire — the dumb JNI shim over
//! `centities_native::MobBatch` (the JNI-free protocol, batch.rs: the
//! bit-identity twin against the direct executor lives THERE). One
//! persistent executor PER LEVEL (the phase-2d split: `itemTickFlush` is
//! woven per Level — a 3-dimension boot fires the boundary 3x per server
//! tick and the ONE executor would tick every store 3x; the split keys
//! the store, the players snapshot and the despawn cfg by the level's
//! small id, and each boundary now ticks ONLY its own level's
//! population). One batch call per level tick.
//!
//! Exports (registered on HOOKS_CLASS next to the L22a per-mob wire):
//!   nativeMobInit(J)V          — the stroll-stream seed (global; each
//!                                level's executor clones it)
//!   nativeMobReset()V          — drop ALL (world unload / dimension change)
//!   nativeMobAdd(IIJDDDI)I     — (level, eid, seed, x, y, z, kind)
//!   nativeMobRemove(IJ)I       — (level, eid)
//!   nativeMobPlayers(I[D[I)V   — (level, [x,y,z]*P, [id,flags]*P)
//!   nativeMobBatch(III[J[D)I   — (level, game_time, difficulty) -> n
//!   nativeDespawnCfg(III[D[I)Z — (level, ellipsoid, present, geom, meta)

use centities_native::MobBatch;
use jvmti_bindings::env::JniEnv;
use jvmti_bindings::sys::jni;
use std::cell::RefCell;
use std::sync::atomic::{AtomicI64, Ordering};

thread_local! {
    /// Server-thread confined (the entity loop runs on one thread; the
    /// L22a wire holds the same confinement assumption). PHASE-2D: keyed
    /// by the level's small id — each Level ticks its OWN executor.
    static EXEC: RefCell<std::collections::HashMap<i64, MobBatch>> =
        RefCell::new(std::collections::HashMap::new());
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

/// The per-level access: the executor for `level` is created lazily from
/// the init seed (a level with no wire arm stays uncreated). None = the
/// wire not armed (the seed never landed).
fn with_level<R>(level: i32, f: impl FnOnce(&mut MobBatch) -> R) -> Option<R> {
    EXEC.with(|c| {
        let mut g = c.borrow_mut();
        let e = g.entry(level as i64).or_insert_with(|| {
            let s = MOB_SEED.load(Ordering::Acquire);
            MobBatch::new(s as u64)
        });
        if MOB_SEED.load(Ordering::Acquire) == i64::MIN {
            // the wire never armed — drop the just-created shell so a
            // pre-arm call cannot tick an empty executor
            g.remove(&(level as i64));
            return None;
        }
        Some(f(e))
    })
}

pub extern "system" fn n_mob_init(_env: *mut jni::JNIEnv, _cls: jni::jclass, seed: i64) {
    MOB_SEED.store(seed, Ordering::Release);
    // the existing executors keep their state (a re-init mid-run is not a
    // contract this wire supports); fresh levels clone the new seed
}

pub extern "system" fn n_mob_reset(_env: *mut jni::JNIEnv, _cls: jni::jclass) {
    MOB_SEED.store(i64::MIN, Ordering::Release);
    EXEC.with(|c| {
        c.borrow_mut().clear();
    });
}

pub extern "system" fn n_mob_add(
    _env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    level: i32,
    eid: i32,
    seed: i64,
    x: f64,
    y: f64,
    z: f64,
    kind: i32,
) -> i32 {
    match with_level(level, |e| e.add_kind(eid as i64, seed as u64, x, y, z, kind)) {
        Some(n) => n as i32,
        None => -1,
    }
}

pub extern "system" fn n_mob_remove(
    _env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    level: i32,
    eid: i64,
) -> i32 {
    match with_level(level, |e| if e.remove(eid) { 1 } else { 0 }) {
        Some(v) => v,
        None => -1,
    }
}

pub extern "system" fn n_mob_players(
    env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    level: i32,
    xyz: jni::jdoubleArray,
    pf: jni::jintArray,
) { let _site_guard = centities_core::SiteGuard::enter(12); 
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
                let _ = with_level(level, |ex| {
                    ex.set_players(&xv[..p * 3], &pv[..p * 2]);
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
    level: i32,
    game_time: i32,
    difficulty: i32,
    ids: jni::jlongArray,
    pos: jni::jdoubleArray,
) -> i32 { let _site_guard = centities_core::SiteGuard::enter(13); 
    unsafe {
        let e = JniEnv::from_raw(env);
        let raw = e.raw();
        let vt = *raw;
        let cap_ids = ((*vt).GetArrayLength)(raw, ids) as usize;
        let cap_pos = ((*vt).GetArrayLength)(raw, pos) as usize;
        // (population, read-back fit) — decided inside the borrows, the
        // Java array writes happen there too (raw/vt are plain pointers)
        let mut out: Option<(usize, bool)> = None;
        let _ = with_level(level, |ex| {
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
// N3-N1 (doc 61) phase-2a/2d: the resolved despawn config table. The push
// rides the boundary ONCE per level (CeHooks.pushDespawnCfgOnce); the
// phase-2d split lands the table on THAT LEVEL's executor — the cfg is
// per-level state exactly like the paper-world.yml it came from.
// ----------------------------------------------------------------------

/// `(III[D[I)Z` — (level, ellipsoid, present-bitmask, (h2, ratio)*8*2,
/// v*8*2) -> true when the table landed. A refuse (cats > 8, bad strides)
/// leaves the previous state; the Java side logs DESPAWN_CFG_LIVE=false
/// and the M2c skip stays (the SAFE default).
pub extern "system" fn n_despawn_cfg(
    env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    level: i32,
    ellipsoid: i32,
    present: i32,
    geom: jni::jdoubleArray,
    meta: jni::jintArray,
) -> u8 { let _site_guard = centities_core::SiteGuard::enter(11); 
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
        // the phase-2d split: the table lands on THIS LEVEL's executor —
        // the cfg is per-level state (the paper config is per-world). The
        // executor is created lazily here if the push races the first mob
        // (the push rides the boundary, before the first batch tick of the
        // level that pushed it).
        let landed = with_level(level, |ex| {
            ex.set_despawn_cfg(Some(cfg));
            1u8
        });
        landed.unwrap_or(0)
    }
}
