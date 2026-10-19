//! c-entities CRUSSTY module (v0.1): exact entity proximity index.
//!
//! Weave targets (Paper 1.21.8, mojang-mapped):
//!   1. Level.getEntities(Entity, AABB, Predicate)  — replace_body → CeHooks.getEntities
//!   2. EntityLookup.addEntity(Entity, boolean, boolean) — prefix probe
//!   3. EntityLookup.removeEntity(Entity)               — prefix probe
//!   4. Entity.setPos(double,double,double)             — prefix probe
//!   5. Entity.setBoundingBox(AABB)                     — prefix probe (exact box)
//!
//! Kill switch: any patch failure leaves the class untouched (vanilla runs);
//! any runtime anomaly flips CeHooks.ACTIVE=false → bridges take the
//! vanilla-mirror path.

#[doc(hidden)]
pub mod patch;

use cplug_abi::{CPluginApi, JavaVmPtr};
use jvmti_bindings::prelude::*;
use jvmti_bindings::sys::jni as jni;
use std::collections::HashMap;
use std::ffi::{c_char, c_void};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex};

const LEVEL_CLASS: &str = "net/minecraft/world/level/Level";
const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const LOOKUP_CLASS: &str = "ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup";
const HOOKS_CLASS: &str = "dev/planeta9091/centities/CeHooks";

static INDEX: Mutex<Option<centities_core::EntityIndex>> = Mutex::new(None);
static ARMED: AtomicBool = AtomicBool::new(false);
static BRIDGE_READY: AtomicBool = AtomicBool::new(false);
static PATCHES_OK: AtomicU64 = AtomicU64::new(0);
static PATCHES_FAIL: AtomicU64 = AtomicU64::new(0);
/// Global ref of the Level class loader, captured once at arm time. Byte
/// hooks run inside JVMTI ClassFileLoadHook callbacks where class queries
/// (GetLoadedClasses etc.) are restricted — they must NEVER call find_class;
/// they resolve everything from this cached loader instead.
static LOADER: AtomicUsize = AtomicUsize::new(0);
/// Original class bytes seen at boot load (stash — hooks may not do heavy
/// work inside the ClassFileLoadHook callback).
static STASH: LazyLock<Mutex<HashMap<&'static str, Vec<u8>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
/// Precomputed rewrites, produced OUTSIDE any hook callback at arm time.
/// The Level.getEntities rewrite runs the Java ASM helper with
/// COMPUTE_FRAMES; ASM resolves common superclasses via loadClass — class
/// loading inside the callback re-enters the runtime's non-reentrant hook
/// registry and self-deadlocks. So: rewrite here, the hook just returns it.
static PRECOMPUTED: LazyLock<Mutex<HashMap<&'static str, Vec<u8>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

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

    // NOTE: never call cplug_sdk::log::info here — cplugin_init may run on a
    // fresh native thread during Agent_OnLoad, before the VM heap exists;
    // AttachCurrentThread at that point SIGSEGVs (TLAB init on null heap).
    // Plain stderr is safe; after kernel-ready the VM is fully initialized.
    eprintln!("[c-entities] module init: hooks registered, waiting for kernel");
    0
}

// ---------------------------------------------------------------------------
// arming
// ---------------------------------------------------------------------------

fn arm_module() {
    // define_bridge returns a GLOBAL ref of the Level class loader (safe to
    // use from other threads / later attach scopes).
    let Some(Some(loader)) = cplug_sdk::jni_util::with_attached(define_bridge) else {
        cplug_sdk::log::info("[c-entities] arm: bridge setup failed — staying vanilla");
        return;
    };
    LOADER.store(loader as usize, Ordering::Release);

    // Precompute the Level.getEntities rewrite OUTSIDE any ClassFileLoadHook
    // callback: replace_body runs the Java ASM helper with COMPUTE_FRAMES,
    // which resolves common superclasses via loadClass — class loading inside
    // the callback self-deadlocks the runtime's non-reentrant hook registry.
    // Uses the bytes stashed by the boot-time hook.
    precompute_level(loader);

    BRIDGE_READY.store(true, Ordering::Release);

    // Retransform → ClassFileLoadHook → our byte hooks patch the classes.
    for cls in [LEVEL_CLASS, ENTITY_CLASS, LOOKUP_CLASS] {
        let rc = cplug_sdk::retransform_class(cls);
        cplug_sdk::log::info(&format!(
            "[c-entities] retransform {cls}: rc={rc}"
        ));
    }

    let ok_n = PATCHES_OK.load(Ordering::Acquire);
    let fail_n = PATCHES_FAIL.load(Ordering::Acquire);
    if ok_n >= 3 && fail_n == 0 {
        // CeHooks.setActive(true)
        let armed = cplug_sdk::jni_util::with_attached(|env| {
            let cls = cplug_sdk::classes::find_class(HOOKS_CLASS)?;
            let mid = env.get_static_method_id(cls.0, "setActive", "(Z)V")?;
            env.call_static_void_method(cls.0, mid, &[jni::jvalue { z: 1 }]);
            Some(())
        })
        .is_some();
        if armed {
            ARMED.store(true, Ordering::Release);
            cplug_sdk::log::info("[c-entities] ARMED: index fast path active");
            return;
        }
    }
    cplug_sdk::log::info(&format!(
        "[c-entities] NOT armed (ok={ok_n}, fail={fail_n}) — vanilla behavior kept"
    ));
}

fn precompute_level(loader: jni::jobject) {
    let stash = STASH.lock().ok().and_then(|mut s| s.remove(LEVEL_CLASS));
    let Some(bytes) = stash else {
        PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
        cplug_sdk::log::info("[c-entities] precompute Level.getEntities: original bytes not seen at boot");
        return;
    };
    let patched = cplug_sdk::jni_util::with_attached(|env| {
        cplug_sdk::asm::replace_body(env, loader, &bytes, &level_spec())
    });
    match patched {
        Some(Some(b)) if !b.is_empty() => {
            // No PATCHES_OK here: the counter is bumped by the byte hook when
            // the rewrite is actually APPLIED via retransform.
            if let Ok(mut pc) = PRECOMPUTED.lock() {
                pc.insert(LEVEL_CLASS, b);
            }
            cplug_sdk::log::info("[c-entities] precomputed Level.getEntities rewrite");
        }
        _ => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] FAILED to precompute Level.getEntities — vanilla kept");
        }
    }
}

fn define_bridge(env: &JniEnv) -> Option<jni::jobject> {
    cplug_sdk::log::info("[c-entities] bridge: step 1 claim class");
    if !cplug_sdk::claim(&format!("class:{HOOKS_CLASS}")) {
        cplug_sdk::log::info("[c-entities] bridge class name already claimed");
        return None;
    }
    cplug_sdk::log::info("[c-entities] bridge: step 2 find Level");
    let Some(level_cls) = cplug_sdk::classes::find_class(LEVEL_CLASS) else {
        cplug_sdk::log::info("[c-entities] bridge: Level not found");
        return None;
    };
    cplug_sdk::log::info("[c-entities] bridge: step 3 getClassLoader mid");
    // level_cls.0 is a jclass == an instance of java.lang.Class: getClassLoader
    // must be resolved on java.lang.Class, not on ServerLevel itself.
    let class_class = env.get_object_class(level_cls.0);
    let Some(mid) =
        env.get_method_id(class_class, "getClassLoader", "()Ljava/lang/ClassLoader;")
    else {
        cplug_sdk::log::info("[c-entities] bridge: getClassLoader mid not found");
        return None;
    };
    let loader = env.call_object_method(level_cls.0, mid, &[]);
    cplug_sdk::log::info("[c-entities] bridge: step 4 define_class");
    let Some(cls) = env.define_class(HOOKS_CLASS, loader, HOOKS_CLASS_BYTES) else {
        cplug_sdk::log::info("[c-entities] bridge define_class failed");
        return None;
    };
    cplug_sdk::log::info("[c-entities] bridge: step 5 register_natives");
    if !register_natives(env, cls) {
        return None;
    }
    // Promote the loader to a global ref while we are still attached (the
    // local ref dies with this attach scope).
    Some(env.new_global_ref(loader))
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

fn level_spec() -> cplug_sdk::asm::ReplaceBody<'static> {
    cplug_sdk::asm::ReplaceBody {
        method_name: "getEntities",
        method_desc: "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
        bridge_owner: HOOKS_CLASS,
        bridge_name: "getEntities",
        bridge_desc: "(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
        args: &[
            cplug_sdk::asm::ArgSpec::Local { slot: 0, ty: b'L' },
            cplug_sdk::asm::ArgSpec::Local { slot: 1, ty: b'L' },
            cplug_sdk::asm::ArgSpec::Local { slot: 2, ty: b'L' },
            cplug_sdk::asm::ArgSpec::Local { slot: 3, ty: b'L' },
        ],
    }
}

fn patch_level(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        // Boot load: stash the original bytes and stay vanilla. The actual
        // rewrite happens at arm time OUTSIDE the hook callback (see
        // precompute_level) — never inside this callback.
        let mut st = STASH.lock().ok()?;
        st.entry(LEVEL_CLASS).or_insert_with(|| bytes.to_vec());
        return None;
    }
    // Retransform pass: hand back the precomputed rewrite (no heavy work
    // allowed in-callback).
    let pre = PRECOMPUTED.lock().ok()?.get(LEVEL_CLASS).cloned();
    match pre {
        Some(b) if !b.is_empty() => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] patched Level.getEntities (precomputed)");
            Some(b)
        }
        _ => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] FAILED to patch Level.getEntities — no precomputed rewrite, vanilla kept");
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
    let mut out: Vec<i64> = Vec::new();
    let hit = with_index(|idx| {
        idx.query(
            world as u16,
            &centities_core::Aabb::new(x0, y0, z0, x1, y1, z1),
            &mut out,
        );
    });
    if hit.is_none() {
        return std::ptr::null_mut();
    }
    unsafe {
        let e = JniEnv::from_raw(env);
        let Some(arr) = e.new_long_array(out.len() as i32) else {
            return std::ptr::null_mut();
        };
        let buf: Vec<jni::jlong> = out.iter().map(|v| *v as jni::jlong).collect();
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
