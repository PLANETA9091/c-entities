//! c-entities CRUSSTY module (v0.2): exact entity proximity index.
//!
//! Weave targets (Paper 1.21.8, mojang-mapped):
//!   1. Level.getEntities(Entity, AABB, Predicate)  — replace_body → CeHooks.getEntities
//!      (covers push via Level.getPushableEntities and the raw overload)
//!   2. Level.getEntitiesOfClass(Class, AABB, Predicate) — replace_body →
//!      CeHooks.getEntitiesOfClass (covers item merge, mob sensor scans)
//!   3. EntityLookup.addEntity(Entity, boolean, boolean) — prefix probe
//!   4. EntityLookup.removeEntity(Entity)               — prefix probe
//!   5. Entity.setPos(double,double,double)             — prefix probe
//!   6. Entity.setBoundingBox(AABB)                     — prefix probe (exact box)
//!
//! Kill switch: any patch failure leaves the class untouched (vanilla runs);
//! any runtime anomaly flips CeHooks.ACTIVE=false → bridges take the
//! vanilla-mirror path.

#[doc(hidden)]
pub mod patch;

use cplug_abi::{CPluginApi, JavaVmPtr};
use jvmti_bindings::prelude::*;
use jvmti_bindings::sys::jni as jni;
use std::ffi::{c_char, c_void};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;

const LEVEL_CLASS: &str = "net/minecraft/world/level/Level";
const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const LOOKUP_CLASS: &str = "ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup";
const HOOKS_CLASS: &str = "dev/planeta9091/centities/CeHooks";

static INDEX: Mutex<Option<centities_core::EntityIndex>> = Mutex::new(None);
static ARMED: AtomicBool = AtomicBool::new(false);
static BRIDGE_READY: AtomicBool = AtomicBool::new(false);
static PATCHES_OK: AtomicU64 = AtomicU64::new(0);
static PATCHES_FAIL: AtomicU64 = AtomicU64::new(0);

fn with_index<R>(f: impl FnOnce(&mut centities_core::EntityIndex) -> R) -> Option<R> {
    let mut g = INDEX.lock().ok()?;
    let idx = g.get_or_insert_with(centities_core::EntityIndex::new);
    Some(f(idx))
}

// ---------------------------------------------------------------------------
// entry point
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn cplugin_init(
    api: *const CPluginApi,
    vm: JavaVmPtr,
    _options: *const c_char,
) -> i32 {
    cplug_sdk::init(api, vm);

    // Byte hooks fire at class load AND at our own retransform passes.
    cplug_sdk::hooks::register_bytes(LEVEL_CLASS, |_n, b| patch_level(b));
    cplug_sdk::hooks::register_bytes(ENTITY_CLASS, |_n, b| patch_entity(b));
    cplug_sdk::hooks::register_bytes(LOOKUP_CLASS, |_n, b| patch_lookup(b));

    // When the first ServerLevel exists: define bridge + natives, then
    // retransform the targets and arm.
    cplug_sdk::on_kernel_ready("net/minecraft/server/level/ServerLevel", || {
        std::thread::spawn(arm_module);
    });

    cplug_sdk::log::info("[c-entities] module init: hooks registered, waiting for kernel");
    0
}

// ---------------------------------------------------------------------------
// arming
// ---------------------------------------------------------------------------

fn arm_module() {
    let ok = cplug_sdk::jni_util::with_attached(define_bridge).unwrap_or(false);
    if !ok {
        cplug_sdk::log::info("[c-entities] arm: bridge setup failed — staying vanilla");
        return;
    }
    BRIDGE_READY.store(true, Ordering::Release);

    // 1. Wait for the server to be FULLY booted before any retransform:
    //    retransforming classes mid-boot (while worlds load) deadlocks the
    //    VM. Steady-state ticking reaches the safepoint instantly.
    let mut ready = false;
    for _ in 0..240 {
        // poll up to 120s
        let r = cplug_sdk::jni_util::with_attached(|env| {
            let cls = cplug_sdk::classes::find_class(HOOKS_CLASS)?;
            let Some(mid) = env.get_static_method_id(cls.0, "serverReady", "()I") else {
                cplug_sdk::jni_util::clear_exception(env);
                return None;
            };
            Some(env.call_static_int_method(cls.0, mid, &[]))
        });
        if r == Some(Some(1)) {
            ready = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    if !ready {
        cplug_sdk::log::info("[c-entities] arm: server never became ready — staying vanilla");
        return;
    }

    // 2. Weave the mutation probes (add/remove/setPos/setBoundingBox). They
    //    are no-ops while ACTIVE=false, so behavior is unchanged.
    for cls in [ENTITY_CLASS, LOOKUP_CLASS] {
        let rc = cplug_sdk::retransform_class(cls);
        cplug_sdk::log::info(&format!("[c-entities] retransform {cls}: rc={rc}"));
    }
    cplug_sdk::jni_util::with_attached(cplug_sdk::jni_util::clear_exception);
    let fail_n = PATCHES_FAIL.load(Ordering::Acquire);
    if fail_n > 0 {
        cplug_sdk::log::info(&format!(
            "[c-entities] NOT armed (probe patches failed: {fail_n}) — vanilla behavior kept"
        ));
        return;
    }

    // 3. Fill the index from the live world state (still vanilla queries).
    let boot = cplug_sdk::jni_util::with_attached(|env| {
        let cls = cplug_sdk::classes::find_class(HOOKS_CLASS)?;
        let Some(mid) = env.get_static_method_id(cls.0, "bootstrap", "()V") else {
            cplug_sdk::jni_util::clear_exception(env);
            return None;
        };
        env.call_static_void_method(cls.0, mid, &[]);
        Some(())
    });
    if boot.is_none() {
        cplug_sdk::log::info("[c-entities] arm: bootstrap fill failed — staying vanilla");
        return;
    }
    let Some(s) = with_index(|idx| idx.stats()) else { return };
    cplug_sdk::log::info(&format!(
        "[c-entities] bootstrap filled: entities={} cells={}", s.entities, s.cells
    ));

    // 4. Flip the kill switch: probes start feeding the (complete) index;
    //    queries are still vanilla until step 5 lands.
    let armed = cplug_sdk::jni_util::with_attached(|env| {
        let cls = cplug_sdk::classes::find_class(HOOKS_CLASS)?;
        let Some(mid) = env.get_static_method_id(cls.0, "setActive", "(Z)V") else {
            cplug_sdk::jni_util::clear_exception(env);
            return None;
        };
        env.call_static_void_method(cls.0, mid, &[jni::jvalue { z: 1 }]);
        Some(())
    })
    .is_some();
    if !armed {
        cplug_sdk::log::info("[c-entities] arm: setActive failed — staying vanilla");
        return;
    }

    // 5. Swap the query paths (Level.getEntities + getEntitiesOfClass).
    let rc = cplug_sdk::retransform_class(LEVEL_CLASS);
    cplug_sdk::log::info(&format!("[c-entities] retransform {LEVEL_CLASS}: rc={rc}"));
    cplug_sdk::jni_util::with_attached(cplug_sdk::jni_util::clear_exception);
    let ok_n = PATCHES_OK.load(Ordering::Acquire);
    let fail_n = PATCHES_FAIL.load(Ordering::Acquire);
    if ok_n < 3 || fail_n > 0 {
        // queries never swapped: stop feeding the index and stay vanilla
        cplug_sdk::jni_util::with_attached(|env| {
            if let Some(cls) = cplug_sdk::classes::find_class(HOOKS_CLASS) {
                if let Some(mid) = env.get_static_method_id(cls.0, "setActive", "(Z)V") {
                    env.call_static_void_method(cls.0, mid, &[jni::jvalue { z: 0 }]);
                }
            }
        });
        cplug_sdk::log::info(&format!(
            "[c-entities] NOT armed (ok={ok_n}, fail={fail_n}) — vanilla behavior kept"
        ));
        return;
    }

    ARMED.store(true, Ordering::Release);
    cplug_sdk::log::info("[c-entities] ARMED: index fast path active");
    spawn_stats_loop();
}

/// Periodic diagnostic heartbeat: index size + patch health. Takes the
/// index lock briefly every 30 s — irrelevant at that cadence.
fn spawn_stats_loop() {
    std::thread::spawn(|| {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(30));
            if !ARMED.load(Ordering::Acquire) {
                break; // disarmed → stop the heartbeat
            }
            let Some(s) = with_index(|idx| idx.stats()) else { break };
            cplug_sdk::log::info(&format!(
                "[c-entities] stats: entities={} cells={} oversize={} ok={} fail={}",
                s.entities, s.cells, s.oversize,
                PATCHES_OK.load(Ordering::Acquire),
                PATCHES_FAIL.load(Ordering::Acquire),
            ));
        }
    });
}

fn define_bridge(env: &JniEnv) -> bool {
    if !cplug_sdk::claim(&format!("class:{HOOKS_CLASS}")) {
        cplug_sdk::log::info("[c-entities] bridge class name already claimed");
        return false;
    }
    let Some(level_cls) = cplug_sdk::classes::find_class(LEVEL_CLASS) else {
        cplug_sdk::log::info("[c-entities] bridge: Level not found");
        return false;
    };
    // getClassLoader() is declared on java.lang.Class, NOT on Level:
    // GetMethodID must target the Class class, invoked on the Level jclass
    // object (a java.lang.Class instance).
    let Some(class_cls) = cplug_sdk::classes::find_class("java/lang/Class") else {
        cplug_sdk::log::info("[c-entities] bridge: java/lang/Class not found");
        return false;
    };
    let Some(mid) = env.get_method_id(
        class_cls.0,
        "getClassLoader",
        "()Ljava/lang/ClassLoader;",
    ) else {
        cplug_sdk::jni_util::clear_exception(env);
        cplug_sdk::log::info("[c-entities] bridge: getClassLoader not resolved");
        return false;
    };
    let loader = env.call_object_method(level_cls.0, mid, &[]);
    let Some(cls) = env.define_class(HOOKS_CLASS, loader, HOOKS_CLASS_BYTES) else {
        cplug_sdk::jni_util::clear_exception(env);
        cplug_sdk::log::info("[c-entities] bridge define_class failed");
        return false;
    };
    register_natives(env, cls)
}

fn register_natives(env: &JniEnv, cls: jni::jclass) -> bool {
    if !cplug_sdk::claim(&format!("native:{HOOKS_CLASS}")) {
        return false;
    }
    let mut methods: Vec<jni::JNINativeMethod> = Vec::new();
    macro_rules! push {
        ($name:literal, $sig:literal, $f:expr) => {{
            let n: &'static [u8] = concat!($name, "\0").as_bytes();
            let s: &'static [u8] = concat!($sig, "\0").as_bytes();
            methods.push(jni::JNINativeMethod {
                name: n.as_ptr() as *mut c_char,
                signature: s.as_ptr() as *mut c_char,
                fnPtr: $f as *mut c_void,
            });
        }};
    }
    push!("queryNative", "(IDDDDDD)[J",
        n_query as extern "system" fn(*mut jni::JNIEnv, jni::jclass, i32, f64, f64, f64, f64, f64, f64) -> jni::jlongArray);
    push!("nativeAdd", "(JIDDDDDD)V",
        n_add as extern "system" fn(*mut jni::JNIEnv, jni::jclass, i64, i32, f64, f64, f64, f64, f64, f64));
    push!("nativeBox", "(JIDDDDDD)V",
        n_box as extern "system" fn(*mut jni::JNIEnv, jni::jclass, i64, i32, f64, f64, f64, f64, f64, f64));
    push!("nativeRemove", "(J)V",
        n_remove as extern "system" fn(*mut jni::JNIEnv, jni::jclass, i64));
    push!("nativePos", "(JIDDDDDDDDD)V",
        n_pos as extern "system" fn(*mut jni::JNIEnv, jni::jclass, i64, i32, f64, f64, f64, f64, f64, f64, f64, f64, f64));
    match env.register_natives(cls, &methods) {
        Ok(()) => {
            cplug_sdk::log::info("[c-entities] natives registered");
            true
        }
        Err(rc) => {
            cplug_sdk::log::info(&format!("[c-entities] register_natives failed rc={rc}"));
            false
        }
    }
}

// ---------------------------------------------------------------------------
// byte hooks
// ---------------------------------------------------------------------------

fn patch_level(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        return None; // boot load stays vanilla; we patch via retransform
    }
    // Pure-Rust body replacement: straight-line bridge calls, no SMT, no
    // exception table, and ZERO JVM calls inside this hook — the ASM path
    // (defineClass inside a retransform while the world is stopped) is a
    // proven deadlock, so it must not be used from here.
    let patched = (|| {
        let b = patch::replace_body_with_static(
            bytes.to_vec(),
            "getEntities",
            "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
            HOOKS_CLASS,
            "getEntities",
            "(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
            &[0, 1, 2, 3],
        )?;
        patch::replace_body_with_static(
            b,
            "getEntitiesOfClass",
            "(Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
            HOOKS_CLASS,
            "getEntitiesOfClass",
            "(Lnet/minecraft/world/level/Level;Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
            &[0, 1, 2, 3],
        )
    })();
    match patched {
        Ok(b) if !b.is_empty() => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] patched Level.getEntities + getEntitiesOfClass");
            Some(b)
        }
        Ok(_) => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] FAILED to patch Level query methods — vanilla kept");
            None
        }
        Err(e) => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(&format!(
                "[c-entities] FAILED to patch Level query methods ({e}) — vanilla kept"
            ));
            None
        }
    }
}

fn patch_lookup(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        return None;
    }
    let add = patch::insert_static_call_at_start(
        bytes.to_vec(),
        "addEntity",
        "(Lnet/minecraft/world/entity/Entity;ZZ)Z",
        HOOKS_CLASS,
        "probeAdd",
        "(Lnet/minecraft/world/entity/Entity;)V",
        &[(1, b'L')],
    );
    let bytes2 = match add {
        Ok(b) => b,
        Err(e) => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(&format!("[c-entities] addEntity probe refused: {e}"));
            return None;
        }
    };
    match patch::insert_static_call_at_start(
        bytes2,
        "removeEntity",
        "(Lnet/minecraft/world/entity/Entity;)V",
        HOOKS_CLASS,
        "probeRemove",
        "(Lnet/minecraft/world/entity/Entity;)V",
        &[(1, b'L')],
    ) {
        Ok(b) => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] patched EntityLookup add/remove probes");
            Some(b)
        }
        Err(e) => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(&format!("[c-entities] removeEntity probe refused: {e}"));
            None
        }
    }
}

fn patch_entity(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        return None;
    }
    let pos = patch::insert_static_call_at_start(
        bytes.to_vec(),
        "setPos",
        "(DDD)V",
        HOOKS_CLASS,
        "probePos",
        "(Lnet/minecraft/world/entity/Entity;DDD)V",
        &[(0, b'L'), (1, b'D'), (3, b'D'), (5, b'D')],
    );
    let bytes2 = match pos {
        Ok(b) => b,
        Err(e) => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(&format!("[c-entities] setPos probe refused: {e}"));
            return None;
        }
    };
    match patch::insert_static_call_at_start(
        bytes2,
        "setBoundingBox",
        "(Lnet/minecraft/world/phys/AABB;)V",
        HOOKS_CLASS,
        "probeBox",
        "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)V",
        &[(0, b'L'), (1, b'L')],
    ) {
        Ok(b) => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] patched Entity setPos/setBoundingBox probes");
            Some(b)
        }
        Err(e) => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(&format!("[c-entities] setBoundingBox probe refused: {e}"));
            None
        }
    }
}


// ---------------------------------------------------------------------------
// natives — all AABB doubles arrive as arguments (no JNI field reads)
// ---------------------------------------------------------------------------

const ESTIMATE_CAP: usize = 8192;

pub extern "system" fn n_query(
    env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    world: i32,
    x0: f64,
    y0: f64,
    z0: f64,
    x1: f64,
    y1: f64,
    z1: f64,
) -> jni::jlongArray {
    let (out, visited): (Vec<i64>, usize) = match with_index(|idx| {
        // pre-walk cost estimate: for huge dense queries the native walk
        // itself (~3 ns/entry) is worth skipping — let Java mirror directly
        let est = idx.query_population(world as u16, &centities_core::Aabb::new(x0, y0, z0, x1, y1, z1));
        if est > ESTIMATE_CAP {
            return None;
        }
        let mut buf = Vec::new();
        let v = idx.query(
            world as u16,
            &centities_core::Aabb::new(x0, y0, z0, x1, y1, z1),
            &mut buf,
        );
        Some((buf, v))
    }) {
        Some(Some((b, v))) => (b, v),
        _ => {
            // marker reply: [0] = -1, [1] = estimated population
            let est = with_index(|idx| {
                idx.query_population(world as u16, &centities_core::Aabb::new(x0, y0, z0, x1, y1, z1))
            })
            .unwrap_or(0);
            unsafe {
                let e = JniEnv::from_raw(env);
                let Some(arr) = e.new_long_array(2) else {
                    return std::ptr::null_mut();
                };
                let buf: Vec<jni::jlong> = vec![-1, est as jni::jlong];
                e.set_long_array_region(arr, 0, 2, &buf);
                return arr;
            }
        }
    };
    unsafe {
        let e = JniEnv::from_raw(env);
        // layout: [0] = candidate count, [1] = entries walked (for the
        // Java-side fast-vs-mirror cost heuristic), [2..] = candidate ids
        let n = out.len();
        let Some(arr) = e.new_long_array((n + 2) as i32) else {
            return std::ptr::null_mut();
        };
        let mut buf: Vec<jni::jlong> = Vec::with_capacity(n + 2);
        buf.push(n as jni::jlong);
        buf.push(visited as jni::jlong);
        for v in out.iter() {
            buf.push(*v as jni::jlong);
        }
        e.set_long_array_region(arr, 0, buf.len() as i32, &buf);
        arr
    }
}

pub extern "system" fn n_add(
    _env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    id: i64,
    world: i32,
    x0: f64,
    y0: f64,
    z0: f64,
    x1: f64,
    y1: f64,
    z1: f64,
) {
    let b = centities_core::Aabb::new(x0, y0, z0, x1, y1, z1);
    let _ = with_index(|idx| idx.upsert(id, world as u16, &b));
}

pub extern "system" fn n_box(
    _env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    id: i64,
    world: i32,
    x0: f64,
    y0: f64,
    z0: f64,
    x1: f64,
    y1: f64,
    z1: f64,
) {
    let b = centities_core::Aabb::new(x0, y0, z0, x1, y1, z1);
    let _ = with_index(|idx| idx.upsert(id, world as u16, &b));
}

pub extern "system" fn n_remove(_env: *mut jni::JNIEnv, _cls: jni::jclass, id: i64) {
    let _ = with_index(|idx| idx.remove(id));
}

/// setPos probe: old box (constant dims) re-anchored to the new position.
/// Exactness for pose changes is restored by the setBoundingBox probe.
pub extern "system" fn n_pos(
    _env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    id: i64,
    world: i32,
    x: f64,
    y: f64,
    z: f64,
    _ox0: f64,
    oy0: f64,
    _oz0: f64,
    ox1: f64,
    oy1: f64,
    _oz1: f64,
) {
    let w = (ox1 - _ox0) * 0.5;
    let h = oy1 - oy0;
    let new = centities_core::Aabb::new(x - w, y, z - w, x + w, y + h, z + w);
    let _ = with_index(|idx| idx.upsert(id, world as u16, &new));
}

// bridge class bytes (compiled by build.rs against the paper jar)
static HOOKS_CLASS_BYTES: &[u8] = include_bytes!(concat!(env!("CE_HOOKS_CLASS")));
