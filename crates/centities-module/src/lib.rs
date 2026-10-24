//! c-entities CRUSSTY module (v0.2): exact entity proximity index.
//!
//! Weave targets (Paper 26.3, mojang-mapped):
//!   1. Level.getEntities(Entity, AABB, Predicate)  — replace_body → CeHooks.getEntities
//!   2. EntityLookup.addEntity(Entity, boolean, boolean) — prefix probe
//!   3. EntityLookup.removeEntity(Entity)               — prefix probe
//!   4. Entity.setBoundingBox(AABB)                     — prefix probe (exact box;
//!      single box-change choke point: setPos -> setPosRaw -> setBoundingBox)
//!
//! Kill switch: any patch failure leaves the class untouched (vanilla runs);
//! any runtime anomaly flips CeHooks.ACTIVE=false → bridges take the
//! vanilla-mirror path.

pub mod itemcore;
pub mod pushcore;
#[doc(hidden)]
pub mod patch;

use cplug_abi::{CPluginApi, JavaVmPtr};
use jvmti_bindings::prelude::*;
use jvmti_bindings::sys::jni;
use std::collections::HashMap;
use std::ffi::{c_char, c_void};
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex};

const LEVEL_CLASS: &str = "net/minecraft/world/level/Level";
const ENTITY_CLASS: &str = "net/minecraft/world/entity/Entity";
const LOOKUP_CLASS: &str = "ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup";
const ITEM_ENTITY_CLASS: &str = "net/minecraft/world/entity/item/ItemEntity";
const SERVER_LEVEL_CLASS: &str = "net/minecraft/server/level/ServerLevel";
const LIVING_ENTITY_CLASS: &str = "net/minecraft/world/entity/LivingEntity";
const SERVER_ENTITY_CLASS: &str = "net/minecraft/server/level/ServerEntity";
const ENTITY_TICK_LIST_CLASS: &str = "net/minecraft/world/level/entity/EntityTickList";
const HOOKS_CLASS: &str = "dev/planeta9091/centities/CeHooks";

// ---------------------------------------------------------------------------
// 4b.2a — zero-JNI steady state: the native side publishes every changed
// cell's id row into Java-visible arrays (CeHooks.ROWS). Java then serves
// small queries from those rows without crossing JNI at all.
//
// Row layout (one long[] per published cell, lazily allocated):
//   [0] = cell key (the SAME 64-bit packing centities_core uses)
//   [1] = (publish stamp << 11) | id_count   (count == ROW_SENTINEL = dense)
//   [2..] = entity ids homed in the cell at publish time
// Validity: row[0] == expected key AND row[1] >>> 11 == PUBLISH_SEQ.
// The stamp changes ONLY on n_reset (index dropped) — everything else keeps
// the invariant "stamp == seq implies row content is exact", which holds
// because every index change that alters a cell's id list marks the cell
// dirty, and dirty cells are republished under the SAME index mutex before
// the query runs (flush-merged), on the server thread.
// ---------------------------------------------------------------------------

const ROW_CAP: usize = 510; // cells denser than this are never published
const ROW_SENTINEL: i64 = 0x7FF; // count-field value marking a killed row
const DIRECT_CAP: usize = 1 << 16; // off-thread event queue bound

pub(crate) static INDEX: Mutex<Option<CeState>> = Mutex::new(None);
static ARMED: AtomicBool = AtomicBool::new(false);
static BRIDGE_READY: AtomicBool = AtomicBool::new(false);
static PATCHES_OK: AtomicU64 = AtomicU64::new(0);
static PATCHES_FAIL: AtomicU64 = AtomicU64::new(0);

/// Global ref of the Level class loader, captured once at arm time. Byte
/// hooks run inside JVMTI ClassFileLoadHook callbacks where class queries
/// (GetLoadedClasses etc.) are restricted — they must NEVER call find_class;
/// they resolve everything from this cached loader instead.
static LOADER: AtomicUsize = AtomicUsize::new(0);
/// Global ref of the DEFINED CeHooks class (the one the woven code calls).
pub(crate) static HOOKS_CLASS_GLOBAL: AtomicUsize = AtomicUsize::new(0);
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

/// JNI surface of the Java row table, resolved once at arm time.
/// The raw JNI refs live as global refs for the process lifetime and are
/// dereferenced only inside attach scopes — Send is sound.
#[derive(Clone, Copy)]
struct PubInfo {
    rows: jni::jobject, // global ref of CeHooks.ROWS (long[][])
    cls: jni::jobject,  // global ref of CeHooks
    fid_pop: jni::jfieldID,
}
unsafe impl Send for PubInfo {}

static PUB_INFO: Mutex<Option<PubInfo>> = Mutex::new(None);
/// Stamp the next publisher instance uses; n_reset bumps it past everything
/// ever published so stale rows can never validate again.
static NEXT_SEQ: AtomicI64 = AtomicI64::new(1);

/// Off-thread probe events (worldgen workers spawning entities). They are
/// QUEUED and drained by the next server-thread apply (n_query/n_flush) —
/// the index and the published rows stay single-thread-writer, and the
/// drain-before-query discipline keeps the flush-in-query window.
enum DirEv {
    Upsert {
        id: i64,
        wid: u16,
        b: centities_core::Aabb,
    },
    Remove {
        id: i64,
    },
}
static DIRECT_Q: Mutex<Vec<DirEv>> = Mutex::new(Vec::new());

struct Publisher {
    info: Option<PubInfo>, // None = publishing disabled (arm incomplete / JNI error)
    dirty: Vec<u32>,
    in_dirty: Vec<bool>, // parallel to the index's cell ids
    dense: Vec<bool>,    // sticky: cells that ever exceeded ROW_CAP
    oversize_changed: bool,
    buf: Vec<i64>, // row write scratch (2 + ROW_CAP)
    ids: Vec<i64>, // cell_scan scratch
    seq: i64,      // stamp this publisher instance writes into rows
}

struct CeState {
    idx: centities_core::EntityIndex,
    /// L12: pushable-only index + the per-entity push mirror (the kernel's
    /// arena). Fed by the same probe batches, read by nativePush.
    pidx: centities_core::EntityIndex,
    push: pushcore::PushCore,
    p: Publisher,
    items: Option<itemcore::ItemCore>,
}

pub(crate) fn new_state() -> CeState {
    CeState {
        idx: centities_core::EntityIndex::new(),
        pidx: centities_core::EntityIndex::new(),
        push: pushcore::PushCore::new(),
        items: None,
        p: Publisher {
            info: PUB_INFO.lock().ok().and_then(|g| *g),
            dirty: Vec::new(),
            in_dirty: Vec::new(),
            dense: Vec::new(),
            oversize_changed: false,
            buf: Vec::new(),
            ids: Vec::new(),
            seq: NEXT_SEQ.load(Ordering::Acquire),
        },
    }
}

impl CeState {
    fn mark_dirty(&mut self, cell: u32) {
        let ci = cell as usize;
        if ci >= self.p.in_dirty.len() {
            self.p.in_dirty.resize(ci + 1, false);
            self.p.dense.resize(ci + 1, false);
        }
        if !self.p.in_dirty[ci] {
            self.p.in_dirty[ci] = true;
            self.p.dirty.push(cell);
        }
    }

    fn note_upsert(&mut self, ev: centities_core::UpsertEv) {
        use centities_core::UpsertEv as E;
        match ev {
            E::Same | E::OversizeSame => {}
            E::New {
                cell,
                from_oversize,
            } => {
                self.mark_dirty(cell);
                if from_oversize {
                    self.p.oversize_changed = true;
                }
            }
            E::Moved { old, new } => {
                self.mark_dirty(old);
                self.mark_dirty(new);
            }
            E::ToOversize { old } => {
                if let Some(c) = old {
                    self.mark_dirty(c);
                }
                self.p.oversize_changed = true;
            }
        }
    }

    fn note_remove(&mut self, ev: centities_core::RemoveEv) {
        use centities_core::RemoveEv as E;
        match ev {
            E::Absent => {}
            E::Normal { cell } => self.mark_dirty(cell),
            E::Oversize => self.p.oversize_changed = true,
        }
    }

    fn upsert(&mut self, id: i64, wid: u16, b: &centities_core::Aabb) {
        let ev = self.idx.upsert(id, wid, b);
        self.note_upsert(ev);
    }

    fn remove(&mut self, id: i64) {
        let ev = self.idx.remove(id);
        self.note_remove(ev);
    }
}

fn push_direct(ev: DirEv) {
    for _ in 0..20_000 {
        if let Ok(mut q) = DIRECT_Q.lock() {
            if q.len() < DIRECT_CAP {
                q.push(ev);
                return;
            }
        }
        std::thread::yield_now();
        std::thread::sleep(std::time::Duration::from_micros(200));
    }
    // Queue stuck (server thread not draining): fall back to a direct apply.
    // Correct for the index; the row may go briefly stale — only reachable
    // when queries are not happening anyway.
    let _ = with_state(|st| match ev {
        DirEv::Upsert { id, wid, b } => st.upsert(id, wid, &b),
        DirEv::Remove { id } => st.remove(id),
    });
}

fn drain_direct(st: &mut CeState) {
    let taken = match DIRECT_Q.lock() {
        Ok(mut q) => {
            if q.is_empty() {
                return;
            }
            std::mem::take(&mut *q)
        }
        Err(_) => return,
    };
    for ev in taken {
        match ev {
            DirEv::Upsert { id, wid, b } => st.upsert(id, wid, &b),
            DirEv::Remove { id } => st.remove(id),
        }
    }
}

fn with_state<R>(f: impl FnOnce(&mut CeState) -> R) -> Option<R> {
    let mut g = INDEX.lock().ok()?;
    let st = g.get_or_insert_with(new_state);
    Some(f(st))
}

#[inline]
fn row_slot(key: u64) -> i32 {
    // [wid:3][cx:10][cz:10] — must match CeHooks.rowQuery. Folding the world
    // into the slot keeps another dimension's cell from evicting this one's
    // row (observed: nether spawn-chunk entities at (0,0) ping-ponged the
    // overworld's row out of the slot on every publish).
    let wid = ((key >> 44) & 0xFFFF) as i32;
    let (cx, cz) = centities_core::key_coords(key);
    ((wid & 0x7) << 20) | (cx & 0x3FF) | ((cz & 0x3FF) << 10)
}

unsafe fn set_static_long(env: &JniEnv, cls: jni::jclass, fid: jni::jfieldID, v: i64) {
    let e = env.raw();
    let vt = *e;
    ((*vt).SetStaticLongField)(e, cls, fid, v);
}

unsafe fn set_static_int(env: &JniEnv, cls: jni::jclass, fid: jni::jfieldID, v: i32) {
    let e = env.raw();
    let vt = *e;
    ((*vt).SetStaticIntField)(e, cls, fid, v);
}

/// Publish every dirty cell's id row into the Java table. Runs ONLY on the
/// server thread, under the INDEX mutex, inside an existing JNI call — so
/// the server-thread Java readers need no synchronization, and no write
/// ever races a row read.
fn publish(env: &JniEnv, st: &mut CeState) {
    let Some(info) = st.p.info else { return };
    if st.p.dirty.is_empty() && !st.p.oversize_changed {
        return;
    }
    let cells = std::mem::take(&mut st.p.dirty);
    for c in &cells {
        let ci = *c as usize;
        if ci < st.p.in_dirty.len() {
            st.p.in_dirty[ci] = false;
        }
    }
    st.p.oversize_changed = false;
    let cls = info.cls as jni::jclass;
    let pop = st.idx.stats().oversize as i32;
    unsafe { set_static_int(env, cls, info.fid_pop, pop) };

    for cell in cells {
        let ci = cell as usize;
        if st.p.dense.get(ci).copied().unwrap_or(false) {
            continue; // killed rows stay killed; zero publish cost
        }
        let Some(key) = st.idx.cell_key_of(cell) else {
            continue;
        };
        let slot = row_slot(key);
        // L4c windowed rows: entries carry a conservative packed x-interval
        // (id + xLo + xHi, 1/1024-block units, cell-relative) so the Java row
        // scan skips x-disjoint candidates without dereferencing the Entity.
        // See EntityIndex::cell_scan_packed for the conservativeness contract.
        let (count, overflow) = st.idx.cell_scan_packed(cell, &mut st.p.ids, ROW_CAP);
        if overflow {
            // sticky dense: kill any published row (Java sees the sentinel)
            if st.p.dense.len() <= ci {
                st.p.dense.resize(ci + 1, false);
            }
            st.p.dense[ci] = true;
            let row = env.get_object_array_element(info.rows as jni::jobjectArray, slot);
            if !row.is_null() {
                st.p.buf.resize(2 + ROW_CAP, 0);
                st.p.buf[0] = key as i64;
                st.p.buf[1] = (st.p.seq << 11) | ROW_SENTINEL;
                env.set_long_array_region(row as jni::jlongArray, 0, 2, &st.p.buf[..2]);
            }
            continue;
        }
        st.p.buf.resize(2 + ROW_CAP, 0);
        st.p.buf[0] = key as i64;
        st.p.buf[1] = (st.p.seq << 11) | count as i64;
        for (k, id) in st.p.ids.iter().enumerate() {
            st.p.buf[2 + k] = *id;
        }
        let mut row = env.get_object_array_element(info.rows as jni::jobjectArray, slot);
        if row.is_null() {
            match env.new_long_array((2 + ROW_CAP) as i32) {
                Some(narr) => {
                    env.set_object_array_element(info.rows as jni::jobjectArray, slot, narr);
                    row = narr;
                }
                None => {
                    st.p.info = None; // OOM: publishing off for good
                    return;
                }
            }
        }
        env.set_long_array_region(
            row as jni::jlongArray,
            0,
            (2 + count) as i32,
            &st.p.buf[..2 + count],
        );
    }
    if env.exception_check() {
        env.exception_describe();
        env.exception_clear();
        st.p.info = None; // any JNI anomaly: publishing off, fast path stays exact
    }
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
    cplug_sdk::hooks::register_bytes(ITEM_ENTITY_CLASS, |_n, b| patch_item(b));
    cplug_sdk::hooks::register_bytes(SERVER_LEVEL_CLASS, |_n, b| patch_serverlevel(b));
    cplug_sdk::hooks::register_bytes(LIVING_ENTITY_CLASS, |_n, b| patch_livingentity(b));
    cplug_sdk::hooks::register_bytes(SERVER_ENTITY_CLASS, |_n, b| patch_serverentity(b));
    cplug_sdk::hooks::register_bytes(ENTITY_TICK_LIST_CLASS, |_n, b| patch_entityticklist(b));

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
    // define_bridge returns (GLOBAL ref of the Level class loader, GLOBAL ref
    // of the defined CeHooks class). Arming MUST go through the exact class
    // object define_class returned: resolving CeHooks again via FindClass can
    // land on a DIFFERENT loader's copy (kernel/paperclip loader labyrinths),
    // arming a class the woven code never calls — the module silently stays
    // vanilla (observed 2026-10-19: ARMED logged, every query mirrored).
    let Some(Some((loader, hooks_cls))) = cplug_sdk::jni_util::with_attached(define_bridge) else {
        cplug_sdk::log::info("[c-entities] arm: bridge setup failed — staying vanilla");
        return;
    };
    LOADER.store(loader as usize, Ordering::Release);
    HOOKS_CLASS_GLOBAL.store(hooks_cls as usize, Ordering::Release);

    // Precompute the Level.getEntities rewrite OUTSIDE any ClassFileLoadHook
    // callback: replace_body runs the Java ASM helper with COMPUTE_FRAMES,
    // which resolves common superclasses via loadClass — class loading inside
    // the callback self-deadlocks the runtime's non-reentrant hook registry.
    // Uses the bytes stashed by the boot-time hook.
    precompute_level(loader);
    force_init_item_entity_raw(loader);
    precompute_item(loader);
    precompute_serverlevel(loader);
    precompute_livingentity(loader);
    precompute_serverentity(loader);
    precompute_entityticklist(loader);

    BRIDGE_READY.store(true, Ordering::Release);

    // Retransform → ClassFileLoadHook → our byte hooks patch the classes.
    for cls in [
        LEVEL_CLASS,
        ENTITY_CLASS,
        LOOKUP_CLASS,
        ITEM_ENTITY_CLASS,
        SERVER_LEVEL_CLASS,
        LIVING_ENTITY_CLASS,
        SERVER_ENTITY_CLASS,
        ENTITY_TICK_LIST_CLASS,
    ] {
        let rc = cplug_sdk::retransform_class(cls);
        cplug_sdk::log::info(&format!("[c-entities] retransform {cls}: rc={rc}"));
    }

    // Paper re-loads kernel classes after bootstrap: a retransform that
    // landed on a doomed bootstrap class object never reaches the running
    // code (observed 2026-10-20: ItemEntity rc=0, hook bytes ignored, the
    // live class stayed vanilla). The woven code itself is the proof:
    // the verifier reads the Java counters and re-applies a class until its
    // counter moves (probes fire = the woven call executes). JVMTI hands the
    // hook the ORIGINAL bytes on every pass, so byte-level markers cannot
    // detect "already applied" — only live execution can.
    std::thread::spawn(|| {
        let targets = [
            (LEVEL_CLASS, Counter::Queries),
            (ENTITY_CLASS, Counter::Boxes),
            (LOOKUP_CLASS, Counter::Adds),
            (ITEM_ENTITY_CLASS, Counter::ItemChecks),
            (SERVER_LEVEL_CLASS, Counter::DispChecks),
            (LIVING_ENTITY_CLASS, Counter::PushChecks),
            (SERVER_ENTITY_CLASS, Counter::TrackerChecks),
        ];
        // The L13 weave is optional (-Dcentities.tickListRaw=false refuses it
        // at precompute time); the verifier must not chase a target that was
        // deliberately never installed.
        let tl_woven = TL_WOVEN.load(Ordering::Acquire);
        let mut targets: Vec<(&str, Counter)> = targets.to_vec();
        if tl_woven {
            targets.push((ENTITY_TICK_LIST_CLASS, Counter::TickListChecks));
        }
        let merge_woven = MERGE_WOVEN.load(Ordering::Acquire);
        let mut logged_done = false;
        for attempt in 0..180 {
            std::thread::sleep(std::time::Duration::from_millis(2000));
            // Deferred weaves (L14 ServerEntity / L13 EntityTickList): the
            // classes may load AFTER the arm; once the byte hook has stashed
            // their bytes, redo the precompute OUTSIDE any ClassFileLoadHook
            // callback (the ASM helper's loadClass deadlocks inside one) and
            // retransform to apply.
            let loader = LOADER.load(Ordering::Acquire);
            if loader != 0 {
                if !SE_WOVEN.load(Ordering::Acquire)
                    && STASH
                        .lock()
                        .map(|s| s.contains_key(SERVER_ENTITY_CLASS))
                        .unwrap_or(false)
                {
                    precompute_serverentity(loader as jni::jobject);
                    if SE_WOVEN.load(Ordering::Acquire) {
                        let rc = cplug_sdk::retransform_class(SERVER_ENTITY_CLASS);
                        cplug_sdk::log::info(&format!(
                            "[c-entities] verifier: deferred ServerEntity weave applied, rc={rc}"
                        ));
                    }
                }
                if !TL_WOVEN.load(Ordering::Acquire)
                    && !TL_DECLINED.load(Ordering::Acquire)
                    && STASH
                        .lock()
                        .map(|s| s.contains_key(ENTITY_TICK_LIST_CLASS))
                        .unwrap_or(false)
                {
                    precompute_entityticklist(loader as jni::jobject);
                    if TL_WOVEN.load(Ordering::Acquire) {
                        let rc = cplug_sdk::retransform_class(ENTITY_TICK_LIST_CLASS);
                        cplug_sdk::log::info(&format!(
                            "[c-entities] verifier: deferred EntityTickList weave applied, rc={rc}"
                        ));
                    }
                }
            }
            let snap = read_hook_counters();
            if attempt < 6 || attempt % 25 == 0 {
                let dbg: Vec<(String, i64)> =
                    snap.iter().map(|(k, v)| (format!("{:?}", k), *v)).collect();
                cplug_sdk::log::info(&format!("[c-entities] verifier snapshot: {:?}", dbg));
            }
            let mut pending: Vec<&str> = Vec::new();
            for (cls, c) in targets.iter() {
                if snap.get(c).copied().unwrap_or(0) == 0 {
                    pending.push(cls);
                }
            }
            if merge_woven && snap.get(&Counter::MergeChecks).copied().unwrap_or(0) == 0 {
                pending.push(ITEM_ENTITY_CLASS);
            }
            if pending.is_empty() {
                if !logged_done {
                    cplug_sdk::log::info(&format!(
                        "[c-entities] patch verifier: all woven code live after {attempt} checks"
                    ));
                    logged_done = true;
                }
                continue; // keep watching: a class reload would zero nothing,
                          // but a future re-apply request must still find us
            }
            for cls in pending {
                let rc = cplug_sdk::retransform_class(cls);
                if attempt < 3 {
                    cplug_sdk::log::info(&format!(
                        "[c-entities] patch verifier: re-apply {cls}: rc={rc}"
                    ));
                }
            }
        }
    });

    let ok_n = PATCHES_OK.load(Ordering::Acquire);
    let fail_n = PATCHES_FAIL.load(Ordering::Acquire);
    if ok_n >= 4 && fail_n == 0 {
        // CeHooks.setActive(true) — on the EXACT class object define_class
        // returned (global ref), never via find_class (loader mismatch).
        let hooks_cls = HOOKS_CLASS_GLOBAL.load(Ordering::Acquire);
        if hooks_cls != 0 {
            let armed = cplug_sdk::jni_util::with_attached(|env| {
                let cls = hooks_cls as jni::jclass;
                let mid = env.get_static_method_id(cls, "setActive", "(Z)V")?;
                env.call_static_void_method(cls, mid, &[jni::jvalue { z: 1 }]);
                // call_static_void_method leaves a pending exception
                // (e.g. ExceptionInInitializerError) unchecked — check and
                // clear, otherwise the arm would be reported OK while the
                // flag never flipped.
                if env.exception_check() {
                    env.exception_describe();
                    env.exception_clear();
                    return None::<()>;
                }
                Some(())
            })
            .is_some();
            if armed {
                ARMED.store(true, Ordering::Release);
                cplug_sdk::log::info("[c-entities] ARMED: index fast path active");
                return;
            }
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
        cplug_sdk::log::info(
            "[c-entities] precompute Level.getEntities: original bytes not seen at boot",
        );
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
            cplug_sdk::log::info(
                "[c-entities] FAILED to precompute Level.getEntities — vanilla kept",
            );
        }
    }
}

/// Precompute the ItemEntity.tick skip guard (L2a). The guard's skip path
/// jumps to bytecode offset 432 — the merge-gate `istore_2` of tick()V in
/// Paper 1.21.8-60 (javap-verified): past ALL physics (oldPos, gravity,
/// noCollision, move, friction, landing damping, the block-position-change
/// event) but BEFORE the merge gate, so the vanilla tail (merge scan,
/// age/despawn incl. the Bukkit despawn event) still runs in bytecode with
/// full private access. The skip path initializes local 1 (the pre-move
/// Vec3, read by the hasImpulse check at 517-541) with the FROZEN delta and
/// pushes iconst_0 for i2 (position unchanged → merge interval 40) — the
/// local/stack types at the target then match the vanilla flow exactly.
fn force_init_item_entity_raw(loader: jni::jobject) {
    // The retransform of a PREPARED-but-not-INITIALIZED class (status 0x3)
    // returned rc=0 yet the woven guard never executed (observed 2026-10-20:
    // restChecks stayed 0 while vanilla tick ran). Forcing <clinit> first
    // brings ItemEntity to status 0x7, same as every target that works.
    let _ = cplug_sdk::jni_util::with_attached(|e| {
        let sys = cplug_sdk::classes::find_class("java/lang/Class")?.0;
        if let Some(mid) = e.get_static_method_id(
            sys,
            "forName",
            "(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;",
        ) {
            if let Some(name) = e.new_string_utf("net.minecraft.world.entity.item.ItemEntity") {
                let res = e.call_static_object_method(
                    sys,
                    mid,
                    &[
                        jni::jvalue { l: name },
                        jni::jvalue { z: 1 },
                        jni::jvalue { l: loader },
                    ],
                );
                if e.exception_check() {
                    e.exception_describe();
                    e.exception_clear();
                } else if !res.is_null() {
                    e.delete_local_ref(res);
                }
            }
        }
        Some(())
    });
}

/// Whether the L3 merge fast path was woven into the precomputed ItemEntity
/// bytes. The patch verifier only demands its counter when this is set —
/// a refused merge weave must NOT block arming (the tick guard stays live).
static MERGE_WOVEN: AtomicBool = AtomicBool::new(false);
/// L14: set when the sendChanges early-return guard was woven. The class
/// may load AFTER the arm (it is only referenced by ChunkMap once a level
/// instance exists) — the precompute then defers and the patch verifier
/// completes the weave once the boot hook has stashed the bytes.
static SE_WOVEN: AtomicBool = AtomicBool::new(false);
/// L13: set when the tickListForEach replacement was actually prepared —
/// gates the patch verifier so a -Dcentities.tickListRaw=false refusal does
/// not send it chasing a target that will never be woven.
static TL_WOVEN: AtomicBool = AtomicBool::new(false);
/// L13: set when the weave was deliberately declined (flag off) — the
/// verifier stops retrying the precompute for it.
static TL_DECLINED: AtomicBool = AtomicBool::new(false);

fn precompute_item(loader: jni::jobject) {
    let stash = STASH
        .lock()
        .ok()
        .and_then(|mut s| s.remove(ITEM_ENTITY_CLASS));
    let Some(bytes) = stash else {
        PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
        cplug_sdk::log::info(
            "[c-entities] precompute ItemEntity guard: original bytes not seen at boot",
        );
        return;
    };
    // L6: the tick is COMPUTED natively — a conditional early-return prefix
    // (`if (CeHooks.itemSkip(e)) return;`) suppresses the vanilla body for
    // items the Rust core computes at the flush. The rest-skip era is over
    // (owner directive: compute in Rust, never skip).
    let tick_spec = cplug_sdk::asm::EarlyReturnGuard {
        method_name: "tick",
        method_desc: "()V",
        check_class: HOOKS_CLASS,
        check_name: "itemSkip",
        check_desc: "(Lnet/minecraft/world/entity/item/ItemEntity;)Z",
    };
    let inactive_spec = cplug_sdk::asm::EarlyReturnGuard {
        method_name: "inactiveTick",
        method_desc: "()V",
        check_class: HOOKS_CLASS,
        check_name: "itemInactiveCollect",
        check_desc: "(Lnet/minecraft/world/entity/item/ItemEntity;)Z",
    };
    let patched = cplug_sdk::jni_util::with_attached(|env| {
        let Some(b1) = cplug_sdk::asm::insert_early_return_guard(env, loader, &bytes, &tick_spec)
        else {
            return None;
        };
        cplug_sdk::asm::insert_early_return_guard(env, loader, &b1, &inactive_spec)
    });
    match patched {
        Some(Some(mut b)) if !b.is_empty() => {
            // L3 Rust merge broad-phase (best-effort): weave the early-return
            // guard into mergeWithNeighbours on TOP of the tick guard. A
            // refusal here keeps the module armed with vanilla merges.
            let mspec = cplug_sdk::asm::EarlyReturnGuard {
                method_name: "mergeWithNeighbours",
                method_desc: "()V",
                check_class: HOOKS_CLASS,
                check_name: "mergeScanNative",
                check_desc: "(Lnet/minecraft/world/entity/item/ItemEntity;)Z",
            };
            match cplug_sdk::jni_util::with_attached(|env| {
                cplug_sdk::asm::insert_early_return_guard(env, loader, &b, &mspec)
            }) {
                Some(Some(b2)) if !b2.is_empty() => {
                    b = b2;
                    MERGE_WOVEN.store(true, Ordering::Release);
                    cplug_sdk::log::info(
                        "[c-entities] precomputed ItemEntity skip guard + merge fast path",
                    );
                }
                _ => {
                    cplug_sdk::log::info(
                        "[c-entities] merge fast path refused — vanilla merge kept (arm continues)",
                    );
                }
            }
            if let Ok(mut pc) = PRECOMPUTED.lock() {
                pc.insert(ITEM_ENTITY_CLASS, b);
            }
            if !MERGE_WOVEN.load(Ordering::Acquire) {
                cplug_sdk::log::info("[c-entities] precomputed ItemEntity skip guard (target 432)");
            }
        }
        _ => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(
                "[c-entities] FAILED to precompute ItemEntity guard — vanilla tick kept",
            );
        }
    }
}

/// L7 dispatch takeover: the tick is suppressed ONE LEVEL HIGHER — a
/// conditional early-return prefix on `ServerLevel.tickNonPassenger(Entity)`:
///
///   if (CeHooks.itemDispatch(entity)) return;
///   ...vanilla dispatcher: setOldPosAndRot, tickCount++/totalEntityAge++,
///      profiler, checkIfActive (AABB vs players), tick()/inactiveTick(),
///      postTick (handlePortal), passengers...
///
/// For a dispatch-owned item the Rust core computes the equivalent of the
/// ENTIRE body natively (active: full physics tick; inactive: the
/// inactiveTick semantics), and the applier mirrors the dispatcher's
/// tickCount/totalEntityAge side effects. A suppressed item therefore skips
/// setOldPosAndRot (5% of tick CPU at 100k items), the per-entity profiler
/// push (a lambda alloc per entity per tick), checkIfActive's AABB work and
/// the tick() entry itself — the Java per-item floor collapses to the guard.
///
/// BEST-EFFORT: if this weave is refused, the module still arms at the L6
/// level (the ItemEntity.tick guard computes the tick natively; only the
/// dispatcher floor stays vanilla).
fn precompute_serverlevel(loader: jni::jobject) {
    let stash = STASH
        .lock()
        .ok()
        .and_then(|mut s| s.remove(SERVER_LEVEL_CLASS));
    let Some(bytes) = stash else {
        PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
        cplug_sdk::log::info(
            "[c-entities] precompute ServerLevel dispatch: original bytes not seen at boot",
        );
        return;
    };
    let spec = cplug_sdk::asm::EarlyReturnGuard {
        method_name: "tickNonPassenger",
        method_desc: "(Lnet/minecraft/world/entity/Entity;)V",
        check_class: HOOKS_CLASS,
        check_name: "itemDispatch",
        check_desc: "(Lnet/minecraft/world/entity/Entity;)Z",
    };
    let patched = cplug_sdk::jni_util::with_attached(|env| {
        cplug_sdk::asm::insert_early_return_guard(env, loader, &bytes, &spec)
    });
    match patched {
        Some(Some(b)) if !b.is_empty() => {
            if let Ok(mut pc) = PRECOMPUTED.lock() {
                pc.insert(SERVER_LEVEL_CLASS, b);
            }
            cplug_sdk::log::info("[c-entities] precomputed ServerLevel dispatch guard");
        }
        _ => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(
                "[c-entities] FAILED to precompute ServerLevel dispatch — L6 tick guard kept",
            );
        }
    }
}

fn patch_serverlevel(bytes: &[u8]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| patch_serverlevel_inner(bytes))
        .ok()
        .flatten()
}

fn patch_serverlevel_inner(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        // Boot load: stash the original bytes, stay vanilla. The guard is
        // woven at arm time (precompute_serverlevel) — outside the callback.
        let mut st = STASH.lock().ok()?;
        st.entry(SERVER_LEVEL_CLASS)
            .or_insert_with(|| bytes.to_vec());
        return None;
    }
    let pre = PRECOMPUTED.lock().ok()?.get(SERVER_LEVEL_CLASS).cloned();
    match pre {
        Some(b) if !b.is_empty() => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(
                "[c-entities] patched ServerLevel tickNonPassenger dispatch guard",
            );
            Some(b)
        }
        _ => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(
                "[c-entities] FAILED to patch ServerLevel dispatch — vanilla kept",
            );
            None
        }
    }
}

/// L12 push dispatch takeover: a conditional early-return prefix on
/// `LivingEntity.pushEntities()`:
///
///   if (CeHooks.pushDispatch(this)) return;
///   ...vanilla pushEntities: isPushable gate, team rule, cramming audit,
///      getPushableEntities query, the doPush force loop...
///
/// For a kernel-owned living entity the Rust push core computes the ENTIRE
/// body natively (pushable-only index scan, exact f64 intersects, the
/// Entity.push force math in vanilla candidate order, numCollisions
/// bookkeeping); the Java guard applies the self writeback + cramming hurt.
/// The doc-21 profile put 79.95% of the mixed-150k server thread inside the
/// vanilla body's section scans — this weave removes the query and the
/// per-pair Java floor while the observable push semantics stay byte-faithful
/// (constants, order, RNG consumption all ported from the 26.3 bytecode).
///
/// BEST-EFFORT: a refusal here keeps the module armed with vanilla pushes.
fn precompute_livingentity(loader: jni::jobject) {
    let stash = STASH
        .lock()
        .ok()
        .and_then(|mut s| s.remove(LIVING_ENTITY_CLASS));
    let Some(bytes) = stash else {
        PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
        cplug_sdk::log::info(
            "[c-entities] precompute LivingEntity push guard: original bytes not seen at boot",
        );
        return;
    };
    // NOTE: check_desc MUST equal the Java method's exact descriptor —
    // pushDispatch(Entity) — or the woven call fails to LINK at runtime
    // (NoSuchMethodError per entity tick, and Paper 26.3 DISCARDS entities
    // that throw in their tick; the first CI run lost every zombie to
    // exactly this). Passing the receiver as the base Entity type is the
    // itemDispatch pattern (widening is verifier-clean).
    let spec = cplug_sdk::asm::EarlyReturnGuard {
        method_name: "pushEntities",
        method_desc: "()V",
        check_class: HOOKS_CLASS,
        check_name: "pushDispatch",
        check_desc: "(Lnet/minecraft/world/entity/Entity;)Z",
    };
    let patched = cplug_sdk::jni_util::with_attached(|env| {
        cplug_sdk::asm::insert_early_return_guard(env, loader, &bytes, &spec)
    });
    match patched {
        Some(Some(b)) if !b.is_empty() => {
            if let Ok(mut pc) = PRECOMPUTED.lock() {
                pc.insert(LIVING_ENTITY_CLASS, b);
            }
            cplug_sdk::log::info("[c-entities] precomputed LivingEntity push dispatch guard");
        }
        _ => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(
                "[c-entities] FAILED to precompute LivingEntity push guard — vanilla pushes kept",
            );
        }
    }
}

fn patch_livingentity(bytes: &[u8]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| patch_livingentity_inner(bytes))
        .ok()
        .flatten()
}

/// L14 tracker fast gate: boot-time hook stashes the original bytes; the
/// guard is woven at arm time (precompute_serverentity). The weave is an
/// early-return prefix — the vanilla sendChanges body stays intact behind
/// it, so any refusal is vanilla by construction.
fn patch_serverentity(bytes: &[u8]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| patch_serverentity_inner(bytes))
        .ok()
        .flatten()
}

fn patch_serverentity_inner(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        let mut st = STASH.lock().ok()?;
        st.entry(SERVER_ENTITY_CLASS)
            .or_insert_with(|| bytes.to_vec());
        return None;
    }
    let pre = PRECOMPUTED.lock().ok()?.get(SERVER_ENTITY_CLASS).cloned();
    match pre {
        Some(b) if !b.is_empty() => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] patched ServerEntity.sendChanges fast gate");
            Some(b)
        }
        _ if !SE_WOVEN.load(Ordering::Acquire) => {
            // Not woven YET (the class loaded after the arm — its bytes are
            // stashed here and the patch verifier completes the weave outside
            // any ClassFileLoadHook callback). Vanilla until then.
            let mut st = STASH.lock().ok()?;
            st.entry(SERVER_ENTITY_CLASS)
                .or_insert_with(|| bytes.to_vec());
            cplug_sdk::log::info(
                "[c-entities] ServerEntity bytes stashed post-arm — verifier will weave",
            );
            None
        }
        _ => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(
                "[c-entities] FAILED to patch ServerEntity.sendChanges — vanilla kept",
            );
            None
        }
    }
}

/// L13 tick-list raw walk: the body of EntityTickList.forEach is REPLACED
/// (not prefixed) — the vanilla body cannot stay behind a full-body
/// replacement, so the exactness burden sits entirely on the mirror. The
/// -Dcentities.tickListRaw=false refusal happens at precompute time and
/// leaves the class untouched (no PATCHES_FAIL: a deliberate disable is not
/// a failure).
fn patch_entityticklist(bytes: &[u8]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| patch_entityticklist_inner(bytes))
        .ok()
        .flatten()
}

fn patch_entityticklist_inner(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        let mut st = STASH.lock().ok()?;
        st.entry(ENTITY_TICK_LIST_CLASS)
            .or_insert_with(|| bytes.to_vec());
        return None;
    }
    let pre = PRECOMPUTED.lock().ok()?.get(ENTITY_TICK_LIST_CLASS).cloned();
    match pre {
        Some(b) if !b.is_empty() => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] patched EntityTickList.forEach raw walk");
            Some(b)
        }
        _ if !TL_WOVEN.load(Ordering::Acquire) => {
            // Deliberate disable (-Dcentities.tickListRaw=false), a refused
            // precompute, or a post-arm class load — the class stays vanilla
            // for now; stashed bytes let the verifier complete (or reject)
            // the weave outside any ClassFileLoadHook callback. None of these
            // is a genuine weave error, so the FAIL ledger stays untouched
            // (a FAIL here would block arming of the whole module).
            let mut st = STASH.lock().ok()?;
            st.entry(ENTITY_TICK_LIST_CLASS)
                .or_insert_with(|| bytes.to_vec());
            cplug_sdk::log::info(
                "[c-entities] EntityTickList bytes stashed post-arm — verifier will decide",
            );
            None
        }
        _ => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(
                "[c-entities] FAILED to patch EntityTickList.forEach — vanilla kept",
            );
            None
        }
    }
}

fn serverentity_spec() -> cplug_sdk::asm::EarlyReturnGuard<'static> {
    // NOTE (the L12 lesson): check_desc must equal the Java method's exact
    // receiver type — sendChanges is declared on ServerEntity, so the guard
    // takes the ServerEntity itself byte-for-byte, or the woven call fails
    // to LINK at runtime.
    cplug_sdk::asm::EarlyReturnGuard {
        method_name: "sendChanges",
        method_desc: "()V",
        check_class: HOOKS_CLASS,
        check_name: "trackerDispatch",
        check_desc: "(Lnet/minecraft/server/level/ServerEntity;)Z",
    }
}

fn precompute_serverentity(loader: jni::jobject) {
    let stash = STASH
        .lock()
        .ok()
        .and_then(|mut s| s.remove(SERVER_ENTITY_CLASS));
    let Some(bytes) = stash else {
        // The class has not loaded yet (it is only referenced once a level
        // instance exists). DEFER — the boot/post-arm hook stashes the bytes
        // whenever they appear and the patch verifier calls this again.
        cplug_sdk::log::info(
            "[c-entities] ServerEntity not loaded yet — tracker gate deferred to the verifier",
        );
        return;
    };
    let patched = cplug_sdk::jni_util::with_attached(|env| {
        cplug_sdk::asm::insert_early_return_guard(env, loader, &bytes, &serverentity_spec())
    });
    match patched {
        Some(Some(b)) if !b.is_empty() => {
            if let Ok(mut pc) = PRECOMPUTED.lock() {
                pc.insert(SERVER_ENTITY_CLASS, b);
            }
            SE_WOVEN.store(true, Ordering::Release);
            cplug_sdk::log::info("[c-entities] precomputed ServerEntity.sendChanges fast gate");
        }
        _ => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(
                "[c-entities] FAILED to precompute ServerEntity tracker gate — vanilla kept",
            );
        }
    }
}

fn ticklist_spec() -> cplug_sdk::asm::ReplaceBody<'static> {
    cplug_sdk::asm::ReplaceBody {
        method_name: "forEach",
        method_desc: "(Ljava/util/function/Consumer;)V",
        bridge_owner: HOOKS_CLASS,
        bridge_name: "tickListForEach",
        bridge_desc:
            "(Lnet/minecraft/world/level/entity/EntityTickList;Ljava/util/function/Consumer;)V",
        args: &[
            cplug_sdk::asm::ArgSpec::Local { slot: 0, ty: b'L' },
            cplug_sdk::asm::ArgSpec::Local { slot: 1, ty: b'L' },
        ],
    }
}

fn precompute_entityticklist(loader: jni::jobject) {
    // Honor the kill switch BEFORE any weaving: read CeHooks.TICKLIST_ON
    // (a Java system property folded into a static final at class init).
    let flag_on: Option<bool> =
        cplug_sdk::jni_util::with_attached(|env| -> Option<bool> {
        let hooks_cls = HOOKS_CLASS_GLOBAL.load(Ordering::Acquire);
        if hooks_cls == 0 {
            return None;
        }
        let cls = hooks_cls as jni::jclass;
        let fid = env.get_static_field_id(cls, "TICKLIST_ON", "Z")?;
        let raw = env.raw();
        // SAFETY: `raw` is a live JNI env inside this attach scope; the
        // vtable deref mirrors the counter reads in read_hook_counters.
        let v = unsafe {
            let vt = *raw;
            ((*vt).GetStaticBooleanField)(raw, cls, fid)
        };
        if env.exception_check() {
            env.exception_describe();
            env.exception_clear();
            return None;
        }
        Some(v != 0)
    })
    .flatten();
    if flag_on != Some(true) {
        TL_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] L13 disabled (centities.tickListRaw) — vanilla forEach kept",
        );
        return;
    }
    // PROVE the private-handle set before the body is replaced: the mirror
    // has no vanilla escape, so a runtime handle failure would crash the
    // first tick (the first CI run crashed exactly there — the lazy init was
    // never wired into the mirror). Refusal here keeps vanilla forever.
    let handles_ok: Option<bool> =
        cplug_sdk::jni_util::with_attached(|env| -> Option<bool> {
            let hooks_cls = HOOKS_CLASS_GLOBAL.load(Ordering::Acquire);
            if hooks_cls == 0 {
                return None;
            }
            let cls = hooks_cls as jni::jclass;
            // void call (the variadic boolean-method slot is *mut c_void in
            // these bindings) + the result read back through a static field.
            let mid = env.get_static_method_id(cls, "initTickListHandles", "()V")?;
            env.call_static_void_method(cls, mid, &[]);
            if env.exception_check() {
                env.exception_describe();
                env.exception_clear();
                return None;
            }
            let fid = env.get_static_field_id(cls, "TL_HANDLES_OK", "Z")?;
            let raw = env.raw();
            // SAFETY: `raw` is a live JNI env inside this attach scope; the
            // vtable deref mirrors the counter reads in read_hook_counters.
            let v = unsafe {
                let vt = *raw;
                ((*vt).GetStaticBooleanField)(raw, cls, fid)
            };
            Some(v != 0)
        })
        .flatten();
    if handles_ok != Some(true) {
        TL_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] L13 handle probe failed — vanilla forEach kept (lastErr in stats)",
        );
        return;
    }
    let stash = STASH
        .lock()
        .ok()
        .and_then(|mut s| s.remove(ENTITY_TICK_LIST_CLASS));
    let Some(bytes) = stash else {
        // DEFERRED: the class may load after the arm; the boot/post-arm hook
        // stashes the bytes and the patch verifier retries this precompute.
        cplug_sdk::log::info(
            "[c-entities] EntityTickList not loaded yet — raw walk deferred to the verifier",
        );
        return;
    };
    let patched = cplug_sdk::jni_util::with_attached(|env| {
        cplug_sdk::asm::replace_body(env, loader, &bytes, &ticklist_spec())
    });
    match patched {
        Some(Some(b)) if !b.is_empty() => {
            if let Ok(mut pc) = PRECOMPUTED.lock() {
                pc.insert(ENTITY_TICK_LIST_CLASS, b);
            }
            TL_WOVEN.store(true, Ordering::Release);
            cplug_sdk::log::info("[c-entities] precomputed EntityTickList.forEach raw walk");
        }
        _ => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(
                "[c-entities] FAILED to precompute EntityTickList.forEach — vanilla kept",
            );
        }
    }
}

fn patch_livingentity_inner(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        // Boot load: stash the original bytes, stay vanilla. The guard is
        // woven at arm time (precompute_livingentity).
        let mut st = STASH.lock().ok()?;
        st.entry(LIVING_ENTITY_CLASS)
            .or_insert_with(|| bytes.to_vec());
        return None;
    }
    let pre = PRECOMPUTED.lock().ok()?.get(LIVING_ENTITY_CLASS).cloned();
    match pre {
        Some(b) if !b.is_empty() => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] patched LivingEntity pushEntities dispatch guard");
            Some(b)
        }
        _ => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(
                "[c-entities] FAILED to patch LivingEntity pushEntities — vanilla kept",
            );
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Counter {
    Queries,
    Boxes,
    Adds,
    RestChecks,
    MergeChecks,
    ItemChecks,
    DispChecks,
    PushChecks,
    TrackerChecks,
    TickListChecks,
}

/// Read the Java-side woven-code counters (probes that fire = the patch is
/// live). Zero on any JNI trouble: the verifier then keeps re-applying,
/// which is the safe direction.
fn read_hook_counters() -> std::collections::HashMap<Counter, i64> {
    let mut out = std::collections::HashMap::new();
    let hooks_cls = HOOKS_CLASS_GLOBAL.load(Ordering::Acquire);
    if hooks_cls == 0 {
        return out;
    }
    let _ = cplug_sdk::jni_util::with_attached(|e| {
        let cls = hooks_cls as jni::jclass;
        let mut fids: std::collections::HashMap<Counter, jni::jfieldID> =
            std::collections::HashMap::new();
        for (c, (n, sig)) in [
            (Counter::Queries, ("queriesFast", "J")),
            (Counter::Boxes, ("probesBox", "J")),
            (Counter::Adds, ("probesAdd", "J")),
            (Counter::RestChecks, ("restChecks", "J")),
            (Counter::MergeChecks, ("mergeChecks", "J")),
            (Counter::ItemChecks, ("itemChecks", "J")),
            (Counter::DispChecks, ("dispChecks", "J")),
            (Counter::PushChecks, ("pushChecks", "J")),
            (Counter::TrackerChecks, ("trackerChecks", "J")),
            (Counter::TickListChecks, ("tickListChecks", "J")),
        ] {
            if let Some(fid) = e.get_static_field_id(cls, n, sig) {
                fids.insert(c, fid);
            }
        }
        let raw = e.raw();
        for (c, fid) in fids {
            // SAFETY: `raw` is a live JNI env inside this attach scope; the
            // vtable deref mirrors the other direct vtable calls in this file.
            let v = unsafe {
                let vt = *raw;
                ((*vt).GetStaticLongField)(raw, cls, fid)
            };
            out.insert(c, v);
        }
        if e.exception_check() {
            e.exception_clear();
        }
        Some(())
    });
    out
}

fn define_bridge(env: &JniEnv) -> Option<(jni::jobject, jni::jobject)> {
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
    let Some(mid) = env.get_method_id(class_class, "getClassLoader", "()Ljava/lang/ClassLoader;")
    else {
        cplug_sdk::log::info("[c-entities] bridge: getClassLoader mid not found");
        return None;
    };
    let loader = env.call_object_method(level_cls.0, mid, &[]);
    cplug_sdk::log::info("[c-entities] bridge: step 4 define_class");
    // define nested/helper classes FIRST (the outer class references them)
    for (name, bytes) in hooks_classes() {
        if name == "dev/planeta9091/centities/CeHooks.class" {
            continue;
        }
        let iname = &name[..name.len() - 6]; // strip ".class"
        if env.define_class(iname, loader, bytes).is_none() {
            cplug_sdk::log::info(&format!(
                "[c-entities] bridge define_class failed for {iname}"
            ));
            return None;
        }
    }
    let Some(cls) = env.define_class(HOOKS_CLASS, loader, HOOKS_CLASS_BYTES) else {
        cplug_sdk::log::info("[c-entities] bridge define_class failed");
        return None;
    };
    // Keep the EXACT class object: global ref (this attach scope's local ref
    // dies at scope end) — arm_module uses it for setActive(true).
    let cls_global = env.new_global_ref(cls);
    cplug_sdk::log::info("[c-entities] bridge: step 5 register_natives");
    if !register_natives(env, cls) {
        return None;
    }
    // 4b.2a: wire the Java row table (forces <clinit> so ROWS is allocated,
    // resolves field ids, takes a global ref). Failure only disables the
    // zero-JNI row path — the native fast path and the mirror stay exact.
    if init_publisher(env, cls) {
        cplug_sdk::log::info("[c-entities] bridge: publisher wired (ROWS + seq)");
    } else {
        cplug_sdk::log::info("[c-entities] bridge: publisher NOT wired — row path disabled");
    }
    // Promote the loader to a global ref while we are still attached (the
    // local ref dies with this attach scope).
    Some((env.new_global_ref(loader), cls_global))
}

/// Resolve the CeHooks static row-table surface and stash it for the
/// publisher. Called once, inside define_bridge's attach scope.
fn init_publisher(env: &JniEnv, cls: jni::jclass) -> bool {
    // Force <clinit> (allocates ROWS = new long[1<<20][]): any static method
    // call initializes the class; setActive(false) is a no-op right now.
    if let Some(mid) = env.get_static_method_id(cls, "setActive", "(Z)V") {
        env.call_static_void_method(cls, mid, &[jni::jvalue { z: 0 }]);
        if env.exception_check() {
            env.exception_describe();
            env.exception_clear();
            return false;
        }
    }
    let Some(fid_rows) = env.get_static_field_id(cls, "ROWS", "[[J") else {
        return false;
    };
    let Some(fid_seq) = env.get_static_field_id(cls, "PUBLISH_SEQ", "J") else {
        return false;
    };
    let Some(fid_pop) = env.get_static_field_id(cls, "OVERSIZE_POP", "I") else {
        return false;
    };
    let rows = env.get_static_object_field(cls, fid_rows);
    if rows.is_null() {
        return false;
    }
    let rows_g = env.new_global_ref(rows);
    let cls_g = env.new_global_ref(cls);
    let info = PubInfo {
        rows: rows_g,
        cls: cls_g,
        fid_pop,
    };
    // Align Java's PUBLISH_SEQ with the stamp this publisher instance writes.
    let seq = NEXT_SEQ.load(Ordering::Acquire);
    unsafe { set_static_long(env, cls_g as jni::jclass, fid_seq, seq) };
    if env.exception_check() {
        env.exception_clear();
        return false;
    }
    if let Ok(mut g) = PUB_INFO.lock() {
        *g = Some(info);
        return true;
    }
    false
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
    push!(
        "queryNative",
        "(IDDDDDD[JI[J[D)I",
        n_query
            as extern "system" fn(
                *mut jni::JNIEnv,
                jni::jclass,
                i32,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                jni::jlongArray,
                i32,
                jni::jlongArray,
                jni::jdoubleArray,
            ) -> i32
    );
    push!(
        "queryNative2",
        "(IDDDDDDI[J[D)I",
        n_query2
            as extern "system" fn(
                *mut jni::JNIEnv,
                jni::jclass,
                i32,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
                i32,
                jni::jlongArray,
                jni::jdoubleArray,
            ) -> i32
    );
    push!(
        "nativeSetOutBuffer",
        "(Ljava/nio/ByteBuffer;)I",
        n_set_out_buffer as extern "system" fn(*mut jni::JNIEnv, jni::jclass, jni::jobject) -> i32
    );
    push!(
        "nativeFlush",
        "(I[J[D[I[D[D[II[J[D)V",
        n_flush
            as extern "system" fn(
                *mut jni::JNIEnv,
                jni::jclass,
                i32,
                jni::jlongArray,
                jni::jdoubleArray,
                jni::jintArray,
                jni::jdoubleArray,
                jni::jdoubleArray,
                jni::jintArray,
                i32,
                jni::jlongArray,
                jni::jdoubleArray,
            )
    );
    push!(
        "nativeBoxDirect",
        "(JIDDDDDD)V",
        n_box
            as extern "system" fn(
                *mut jni::JNIEnv,
                jni::jclass,
                i64,
                i32,
                f64,
                f64,
                f64,
                f64,
                f64,
                f64,
            )
    );
    push!(
        "nativeRemoveDirect",
        "(J)V",
        n_remove as extern "system" fn(*mut jni::JNIEnv, jni::jclass, i64)
    );
    push!(
        "nativeReset",
        "()V",
        n_reset as extern "system" fn(*mut jni::JNIEnv, jni::jclass)
    );
    push!(
        "nativeItemTick",
        "(II[J[I[DI[I[D[I[D[I)I",
        itemcore::n_item_tick as *const c_void
    );
    push!(
        "nativeItemInvalidate",
        "(II[I[I[I)V",
        itemcore::n_item_invalidate as *const c_void
    );
    push!(
        "nativePush",
        "(IIIIIIII)J",
        n_push as extern "system" fn(*mut jni::JNIEnv, jni::jclass, i32, i32, i32, i32, i32, i32, i32, i32) -> i64
    );
    push!(
        "nativePushNcol",
        "(I)I",
        n_push_ncol as extern "system" fn(*mut jni::JNIEnv, jni::jclass, i32) -> i32
    );
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

// LAW: a panic unwinding through the extern "C" JVMTI callback aborts the
// JVM (observed: signal 6 during the 4a patcher bring-up). Every hook body
// is wrapped in catch_unwind — a failure degrades to vanilla bytes (class
// untouched), never a crash. Zero cost on the happy path.
fn patch_level(bytes: &[u8]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| patch_level_inner(bytes))
        .ok()
        .flatten()
}

fn patch_level_inner(bytes: &[u8]) -> Option<Vec<u8>> {
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
            // 4b.2a + L2a: the Level hook carries BOTH rewrites — the
            // precomputed getEntities replace_body AND the setBlock wake
            // prefixes (a wake-probe failure must fail the arm: resting
            // items without wakes would go stale).
            let b2 = match patch_level_setblock_wakes(b) {
                Ok(b2) => b2,
                Err(e) => {
                    PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
                    cplug_sdk::log::info(&format!("[c-entities] setBlock wake probe refused: {e}"));
                    return None;
                }
            };
            // L6 flush prefix (best-effort): if refused, the mode table never
            // arms (items stay vanilla) — inherently safe, arm continues.
            let b3 = match patch::insert_static_call_at_start(
                b2.clone(),
                "tickBlockEntities",
                "()V",
                HOOKS_CLASS,
                "itemTickFlush",
                "(Lnet/minecraft/world/level/Level;)V",
                &[(0, b'L')],
            ) {
                Ok(b3) => {
                    cplug_sdk::log::info(
                        "[c-entities] L6 flush prefix woven (Level.tickBlockEntities)",
                    );
                    b3
                }
                Err(e) => {
                    cplug_sdk::log::info(&format!(
                        "[c-entities] L6 flush prefix refused (items stay vanilla): {e}"
                    ));
                    b2.clone()
                }
            };
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] patched Level.getEntities + setBlock wakes");
            Some(b3)
        }
        _ => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] FAILED to patch Level.getEntities — no precomputed rewrite, vanilla kept");
            None
        }
    }
}

fn patch_lookup(bytes: &[u8]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| patch_lookup_inner(bytes))
        .ok()
        .flatten()
}

fn patch_lookup_inner(bytes: &[u8]) -> Option<Vec<u8>> {
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
    std::panic::catch_unwind(|| patch_entity_inner(bytes))
        .ok()
        .flatten()
}

/// L2a resting items: the tick guard is precomputed at arm time
/// (precompute_item); the boot-time hook only stashes the bytes.
fn patch_item(bytes: &[u8]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| patch_item_inner(bytes))
        .ok()
        .flatten()
}

fn patch_item_inner(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        let mut st = STASH.lock().ok()?;
        st.entry(ITEM_ENTITY_CLASS)
            .or_insert_with(|| bytes.to_vec());
        return None;
    }

    let pre = PRECOMPUTED.lock().ok()?.get(ITEM_ENTITY_CLASS).cloned();
    match pre {
        Some(b) if !b.is_empty() => {
            // Wake insurance: an explosion that only knocks an item back (no
            // block broken nearby) would leave it resting while vanilla moves
            // it. hurtServer fires before any knockback — clear the rest state.
            let pre2 = match patch::insert_static_call_at_start(
                b,
                "hurtServer",
                "(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;F)Z",
                HOOKS_CLASS,
                "probeHurt",
                "(Lnet/minecraft/world/entity/Entity;)V",
                &[(0, b'L')],
            ) {
                Ok(b2) => b2,
                Err(e) => {
                    PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
                    cplug_sdk::log::info(&format!(
                        "[c-entities] ItemEntity.hurtServer probe refused: {e}"
                    ));
                    return None;
                }
            };
            // L8.4 empty-stack dispatch cache: EVERY ItemEntity stack write
            // goes through setItem (both ctors, NBT load, merge absorb —
            // javap-verified), so this prefix probe keeps the guard's
            // empty-parity bit fresh and the dispatch off the SynchedEntityData
            // read (6.55% MSPT on items-100k-mod). Pure cache refresh — the
            // woven call must never fail arming (the fallback read is exact).
            let pre3 = match patch::insert_static_call_at_start(
                pre2,
                "setItem",
                "(Lnet/minecraft/world/item/ItemStack;)V",
                HOOKS_CLASS,
                "probeSetItem",
                "(Lnet/minecraft/world/entity/item/ItemEntity;Lnet/minecraft/world/item/ItemStack;)V",
                &[(0, b'L'), (1, b'L')],
            ) {
                Ok(b3) => b3,
                Err(e) => {
                    PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
                    cplug_sdk::log::info(&format!(
                        "[c-entities] ItemEntity.setItem probe refused: {e}"
                    ));
                    return None;
                }
            };
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(
                "[c-entities] patched ItemEntity.tick guard + hurt/setItem probes",
            );
            Some(pre3)
        }
        _ => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] FAILED to patch ItemEntity.tick — vanilla kept");
            None
        }
    }
}

/// L2a wake probes: EVERY gameplay block write funnels through
/// Level.setBlock(pos, state, flags[, recursion]) — Paper made it concrete on
/// Level (javap-verified; ServerLevel does not declare it). Resting items are
/// woken via CeHooks.probeWake so their next tick runs full vanilla physics.
/// A probe failure here MUST disable arming entirely (silent wake loss would
/// make resting items stale — the arm gate counts this class).
fn patch_level_setblock_wakes(bytes: Vec<u8>) -> Result<Vec<u8>, String> {
    const WAKE_DESC: &str = "(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V";
    // 26.3 RETARGET: Level.setBlock is now the 4-arg overload
    // (pos, state, flags, recursion) — the 3-arg overload was REMOVED by
    // Mojang (javap-verified on the shipped 26.3-218 jar; on 1.21.8 both
    // existed and both were probed). At least ONE probe must land or arming
    // is refused (silent wake loss would make resting items stale — the arm
    // gate counts this class). "not found" on one overload is therefore
    // tolerated as long as the other landed; any structural failure aborts.
    let mut out = bytes;
    let mut landed = false;
    for desc in [
        "(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;II)Z",
        "(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z",
    ] {
        match patch::insert_static_call_at_start(
            out.clone(),
            "setBlock",
            desc,
            HOOKS_CLASS,
            "probeWake",
            WAKE_DESC,
            &[(0, b'L'), (1, b'L')],
        ) {
            Ok(next) => {
                out = next;
                landed = true;
            }
            Err(e) => {
                if !e.contains("not found") {
                    return Err(e);
                }
            }
        }
    }
    if !landed {
        return Err("setBlock: no overload found (probe refused)".to_string());
    }
    Ok(out)
}

fn patch_entity_inner(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        return None;
    }

    // setBoundingBox is the SINGLE box-change choke point in 1.21.8
    // (setPos(DDD) -> setPosRaw(DDD,Z) -> makeBoundingBox -> setBoundingBox,
    // javap-verified on the shipped jar). One probe covers every move with
    // the EXACT new box — no separate setPos probe, half the JNI traffic.
    let boxed = match patch::insert_static_call_at_start(
        bytes.to_vec(),
        "setBoundingBox",
        "(Lnet/minecraft/world/phys/AABB;)V",
        HOOKS_CLASS,
        "probeBox",
        "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)V",
        &[(0, b'L'), (1, b'L')],
    ) {
        Ok(b) => b,
        Err(e) => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(&format!("[c-entities] setBoundingBox probe refused: {e}"));
            return None;
        }
    };

    // L3.2 wake-on-impulse: a RESTING item's tick is skipped, so ANY external
    // velocity change (a zombie/boat pushing it, knockback, plugin velocity)
    // would never surface — the item would stay put while vanilla slides it
    // away, and pushers re-query it every tick (measured on CI spread/mixed:
    // pushed-resting items densified their piles, 2.27x AABB.intersects CPU
    // across the 19M push queries, +32..70% p50). setDeltaMovement is the
    // complete choke point for velocity writes (Entity.push(*) lands here;
    // both overloads write the field directly). Waking a NON-resting entity
    // is a no-op (one array read + branch, ~3ns — the common case for every
    // mob/stacked item), so the probe is free outside the rest guard.
    let d1 = match patch::insert_static_call_at_start(
        boxed,
        "setDeltaMovement",
        "(Lnet/minecraft/world/phys/Vec3;)V",
        HOOKS_CLASS,
        "probeDelta",
        "(Lnet/minecraft/world/entity/Entity;)V",
        &[(0, b'L')],
    ) {
        Ok(b) => b,
        Err(e) => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(&format!(
                "[c-entities] setDeltaMovement(Vec3) probe refused: {e}"
            ));
            return None;
        }
    };
    match patch::insert_static_call_at_start(
        d1,
        "setDeltaMovement",
        "(DDD)V",
        HOOKS_CLASS,
        "probeDelta",
        "(Lnet/minecraft/world/entity/Entity;)V",
        &[(0, b'L')],
    ) {
        Ok(b) => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] patched Entity box probe + wake-on-impulse");
            Some(b)
        }
        Err(e) => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(&format!(
                "[c-entities] setDeltaMovement(DDD) probe refused: {e}"
            ));
            None
        }
    }
}

// ---------------------------------------------------------------------------
// natives — all AABB doubles arrive as arguments (no JNI field reads).
// Iteration 2b: queries write into a caller-provided buffer (no per-query
// jlongArray alloc); probe changes arrive as ONE packed batch per flush.
// ---------------------------------------------------------------------------

thread_local! {
    /// Scratch for query results (native side); reused across calls.
    static QSCRATCH: std::cell::RefCell<Vec<jni::jlong>> = const { std::cell::RefCell::new(Vec::new()) };
    /// Batch-array read buffers (grow-only, one JNI region copy per apply).
    static BIDBUF: std::cell::RefCell<Vec<jni::jlong>> = const { std::cell::RefCell::new(Vec::new()) };
    static BBOXBUF: std::cell::RefCell<Vec<f64>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Shared core of n_query / n_query2 (doc 21): applies the direct queue and
/// the caller's pending probe batch under ONE lock acquisition, publishes
/// dirty rows, runs the exact query into `out`. Returns false when the index
/// lock is unavailable (caller maps that to its own not-ready contract).
///
/// SAFETY: `e` must be a live env on the calling thread; batch arrays are
/// read via JNI region ops inside the lock.
unsafe fn locked_flush_query(
    e: &JniEnv,
    world: i32,
    x0: f64,
    y0: f64,
    z0: f64,
    x1: f64,
    y1: f64,
    z1: f64,
    bn: usize,
    batch_ids: jni::jlongArray,
    batch_boxes: jni::jdoubleArray,
    out: &mut Vec<i64>,
) -> bool {
    let mut g = match INDEX.lock() {
        Ok(g) => g,
        Err(_) => return false,
    };
    let st = g.get_or_insert_with(new_state);
    drain_direct(st);
    if bn > 0 {
        BIDBUF.with(|b| {
            let mut b = b.borrow_mut();
            b.clear();
            b.resize(bn, 0);
            e.get_long_array_region(batch_ids, 0, bn as i32, &mut b);
            for l in b.iter() {
                if *l < 0 {
                    let ev = st.idx.remove(*l & 0xFFFF_FFFF);
                    st.note_remove(ev);
                }
            }
            BBOXBUF.with(|bx| {
                let mut bx = bx.borrow_mut();
                bx.clear();
                bx.resize(bn * 6, 0.0);
                jni_get_double_array_region(e, batch_boxes, 0, (bn * 6) as i32, &mut bx);
                for (i, l) in b.iter().enumerate() {
                    if *l >= 0 {
                        let wid = ((*l >> 32) & 0xFFFF) as u16;
                        let o = i * 6;
                        let ev = st.idx.upsert(
                            *l & 0xFFFF_FFFF,
                            wid,
                            &centities_core::Aabb::new(
                                bx[o],
                                bx[o + 1],
                                bx[o + 2],
                                bx[o + 3],
                                bx[o + 4],
                                bx[o + 5],
                            ),
                        );
                        st.note_upsert(ev);
                    }
                }
            });
        });
    }
    publish(e, st);
    st.idx.query(
        world as u16,
        &centities_core::Aabb::new(x0, y0, z0, x1, y1, z1),
        out,
    );
    true
}

/// Returns: n >= 0 — count written into `out`; n > out.len — required
/// capacity (caller regrows and retries); -1 — index not ready.
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
    out: jni::jlongArray,
    batch_n: i32,
    batch_ids: jni::jlongArray,
    batch_boxes: jni::jdoubleArray,
) -> i32 {
    // FLUSH-MERGED QUERY (iteration 3): the caller's pending probe batch is
    // applied under the SAME lock acquisition right before the query — one
    // JNI crossing and one lock per query (2b paid a separate flush call).
    // 4b.2a: after the apply, dirty cells' id rows are published into the
    // Java table (same lock, same thread) — Java then serves small queries
    // without ever entering this function.
    let bn = if batch_n > 0 { batch_n as usize } else { 0 };
    let n = unsafe {
        let e = JniEnv::from_raw(env);
        QSCRATCH
            .with(|q| {
                let mut q = q.borrow_mut();
                if locked_flush_query(
                    &e,
                    world,
                    x0,
                    y0,
                    z0,
                    x1,
                    y1,
                    z1,
                    bn,
                    batch_ids,
                    batch_boxes,
                    &mut q,
                ) {
                    Some(q.len() as i32)
                } else {
                    None
                }
            })
            .unwrap_or(-1)
    };
    if n < 0 {
        return -1;
    }
    if n > 0 {
        unsafe {
            let e = JniEnv::from_raw(env);
            let cap = e.get_array_length(out as jni::jarray);
            if cap < n {
                return n; // caller regrows and retries
            }
            QSCRATCH.with(|q| {
                let q = q.borrow();
                // jlong == i64 — write straight from the scratch buffer.
                e.set_long_array_region(out, 0, n, &q[..n as usize]);
            });
        }
    }
    n
}

// ---------------------------------------------------------------------------
// doc 21 — DIRECT OUT BUFFER. The query result ids are written with raw
// pointer stores into a pinned direct ByteBuffer (registered once at arm,
// regrown only when a query outgrows it) — the per-query JNI array ops
// (get_array_length + set_long_array_region) disappear from the hot path.
// A NewGlobalRef keeps the buffer object alive for as long as it is
// registered; re-registration releases the previous one.
// ---------------------------------------------------------------------------

static OUT_ADDR: AtomicU64 = AtomicU64::new(0);
static OUT_CAP: AtomicI64 = AtomicI64::new(0);
static OUT_GREF: AtomicUsize = AtomicUsize::new(0);

pub extern "system" fn n_set_out_buffer(
    env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    buf: jni::jobject,
) -> i32 {
    unsafe {
        let e = JniEnv::from_raw(env);
        let raw = e.raw();
        let vt = *raw;
        // release the previous registration first (if any)
        let prev = OUT_GREF.swap(0, Ordering::AcqRel);
        OUT_ADDR.store(0, Ordering::Release);
        if prev != 0 {
            ((*vt).DeleteGlobalRef)(raw, prev as jni::jobject);
        }
        if buf.is_null() {
            OUT_CAP.store(0, Ordering::Release);
            return 0;
        }
        let addr = ((*vt).GetDirectBufferAddress)(raw, buf);
        if addr.is_null() {
            OUT_CAP.store(0, Ordering::Release);
            return 0;
        }
        let cap_bytes = ((*vt).GetDirectBufferCapacity)(raw, buf);
        if cap_bytes < 8 {
            OUT_CAP.store(0, Ordering::Release);
            return 0;
        }
        let g = ((*vt).NewGlobalRef)(raw, buf);
        if g.is_null() {
            OUT_CAP.store(0, Ordering::Release);
            return 0;
        }
        OUT_GREF.store(g as usize, Ordering::Release);
        OUT_CAP.store(cap_bytes / 8, Ordering::Release);
        OUT_ADDR.store(addr as u64, Ordering::Release); // published LAST
        (cap_bytes / 8).min(i32::MAX as i64) as i32
    }
}

/// queryNative2: identical semantics to n_query, but results land in the
/// registered direct buffer instead of a JNI long array.
/// Returns: n >= 0 — count in the buffer; n <= -2 — required capacity (-n),
/// the batch was already consumed; -1 — not ready / no buffer.
pub extern "system" fn n_query2(
    env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    world: i32,
    x0: f64,
    y0: f64,
    z0: f64,
    x1: f64,
    y1: f64,
    z1: f64,
    batch_n: i32,
    batch_ids: jni::jlongArray,
    batch_boxes: jni::jdoubleArray,
) -> i32 {
    let bn = if batch_n > 0 { batch_n as usize } else { 0 };
    let n = unsafe {
        let e = JniEnv::from_raw(env);
        QSCRATCH
            .with(|q| {
                let mut q = q.borrow_mut();
                if locked_flush_query(
                    &e,
                    world,
                    x0,
                    y0,
                    z0,
                    x1,
                    y1,
                    z1,
                    bn,
                    batch_ids,
                    batch_boxes,
                    &mut q,
                ) {
                    Some(q.len() as i32)
                } else {
                    None
                }
            })
            .unwrap_or(-1)
    };
    if n < 0 {
        return -1;
    }
    let addr = OUT_ADDR.load(Ordering::Acquire) as *mut i64;
    if addr.is_null() {
        return -1; // buffer vanished — caller falls back to the legacy path
    }
    let cap = OUT_CAP.load(Ordering::Acquire);
    if n as i64 > cap {
        // regrow hint. cap >= 1024 at arm, so n >= 1025 -> -n <= -1025,
        // safely distinct from -1 (not ready).
        return -n;
    }
    if n > 0 {
        unsafe {
            QSCRATCH.with(|q| {
                let q = q.borrow();
                // jlong == i64 — raw stores into the pinned direct buffer.
                std::ptr::copy_nonoverlapping(q.as_ptr(), addr, n as usize);
            });
        }
    }
    n
}

/// jvmti-bindings has no double-array wrapper — call the JNI table directly.
unsafe fn jni_get_double_array_region(
    e: &JniEnv,
    array: jni::jdoubleArray,
    start: i32,
    len: i32,
    buf: &mut [f64],
) {
    let env = e.raw();
    let vtable = *env;
    ((*vtable).GetDoubleArrayRegion)(env, array, start, len, buf.as_mut_ptr());
}

/// Apply one packed batch: entries with bit63 set are removals
/// ([remove:1][world:16][id:32]), the rest are upserts with 6 doubles each.
/// L12: the same call applies the push-state extension (flags/pxz/delta/vid
/// per entry) and the delta-only batch into the push mirror + the
/// pushable-only index.
pub extern "system" fn n_flush(
    env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    n: i32,
    ids: jni::jlongArray,
    boxes: jni::jdoubleArray,
    pf: jni::jintArray,
    pxz: jni::jdoubleArray,
    dv: jni::jdoubleArray,
    vid: jni::jintArray,
    dbn: i32,
    db_ids: jni::jlongArray,
    db_dv: jni::jdoubleArray,
) {
    if n <= 0 && dbn <= 0 {
        return;
    }
    unsafe {
        let e = JniEnv::from_raw(env);
        let mut id_buf: Vec<i64> = vec![0; n.max(0) as usize];
        let mut box_buf: Vec<f64> = vec![0.0; n.max(0) as usize * 6];
        let mut pf_buf: Vec<i32> = vec![0; n.max(0) as usize];
        let mut pxz_buf: Vec<f64> = vec![0.0; n.max(0) as usize * 2];
        let mut dv_buf: Vec<f64> = vec![0.0; n.max(0) as usize * 3];
        let mut vid_buf: Vec<i32> = vec![0; n.max(0) as usize];
        if n > 0 {
            e.get_long_array_region(ids, 0, n, &mut id_buf);
            jni_get_double_array_region(&e, boxes, 0, n * 6, &mut box_buf);
            e.get_int_array_region(pf, 0, n, &mut pf_buf);
            jni_get_double_array_region(&e, pxz, 0, n * 2, &mut pxz_buf);
            jni_get_double_array_region(&e, dv, 0, n * 3, &mut dv_buf);
            e.get_int_array_region(vid, 0, n, &mut vid_buf);
        }
        let mut db_buf: Vec<i64> = vec![0; dbn.max(0) as usize];
        let mut dbdv_buf: Vec<f64> = vec![0.0; dbn.max(0) as usize * 3];
        if dbn > 0 {
            e.get_long_array_region(db_ids, 0, dbn, &mut db_buf);
            jni_get_double_array_region(&e, db_dv, 0, dbn * 3, &mut dbdv_buf);
        }
        let _ = with_state(|st| {
            drain_direct(st);
            for (i, l) in id_buf.iter().enumerate() {
                let id = *l & 0xFFFF_FFFF;
                if *l < 0 {
                    let ev = st.idx.remove(id);
                    st.note_remove(ev);
                    // the push view drops removals unconditionally (a no-op
                    // for entities that were never pushable)
                    st.push.remove(id as i64);
                    let ev2 = st.pidx.remove(id as i64);
                    let _ = ev2;
                } else {
                    let wid = ((*l >> 32) & 0xFFFF) as u16;
                    let o = i * 6;
                    let b = centities_core::Aabb::new(
                        box_buf[o],
                        box_buf[o + 1],
                        box_buf[o + 2],
                        box_buf[o + 3],
                        box_buf[o + 4],
                        box_buf[o + 5],
                    );
                    let ev = st.idx.upsert(id, wid, &b);
                    st.note_upsert(ev);
                    if pf_buf[i] & (pushcore::PF_PUSHABLE as i32) != 0 {
                        let evicted = st.push.upsert(
                            id as i64,
                            &b,
                            pxz_buf[i * 2],
                            pxz_buf[i * 2 + 1],
                            dv_buf[i * 3],
                            dv_buf[i * 3 + 1],
                            dv_buf[i * 3 + 2],
                            pf_buf[i] as u8,
                            vid_buf[i] as i64,
                        );
                        if let Some(old) = evicted {
                            // arena slot collision: the previous owner lost
                            // its mirror slot — it must leave the push index
                            // too (it routes vanilla via the guard's mirror
                            // miss from now on)
                            st.pidx.remove(old);
                        }
                        let ev2 = st.pidx.upsert(id as i64, wid, &b);
                        let _ = ev2;
                    } else {
                        // reclassified non-pushable (or a plain living): drop
                        // it from the push view (no-ops when absent)
                        st.push.remove(id as i64);
                        st.pidx.remove(id as i64);
                    }
                }
            }
            for (j, l) in db_buf.iter().enumerate() {
                let id = (*l & 0xFFFF_FFFF) as i64;
                st.push
                    .set_delta(id, dbdv_buf[j * 3], dbdv_buf[j * 3 + 1], dbdv_buf[j * 3 + 2]);
            }
            // Mid-tick cap flush: publish now, or the rows would be
            // stale-with-valid-stamp until the next native query.
            publish(&e, st);
        });
    }
}

// ---------------------------------------------------------------------------
// L12 — the native push kernel.
// ---------------------------------------------------------------------------

thread_local! {
    static PUSH_OUT: std::cell::RefCell<pushcore::PushOut> =
        std::cell::RefCell::new(pushcore::PushOut::default());
}

/// Compute one `LivingEntity.pushEntities()` natively. Returns the packed
/// verdict (bit0 ok, bit1 cramming, bits 2..21 non-passenger hits,
/// bits 22..53 numCollisions); the full post-tick delta is written into the
/// PUSH_SDX/SDY/SDZ statics. The caller's pending probe batches were flushed
/// by the Java guard (flushIfPending) BEFORE this call, so the mirror is
/// current.
pub extern "system" fn n_push(
    env: *mut jni::JNIEnv,
    cls: jni::jclass,
    world: i32,
    self_id: i32,
    self_vehicle: i32,
    self_player: i32,
    only_players: i32,
    max_ec: i32,
    cramming: i32,
    cur_tick: i32,
) -> i64 {
    let res = with_state(|st| {
        drain_direct(st);
        let slot = match st.push.slot_of(self_id as i64) {
            Some(s) if st.push.flags[s] & pushcore::PF_PUSHABLE != 0 => s,
            _ => return 0i64, // mirror: vanilla body runs
        };
        PUSH_OUT.with(|o| {
            let mut o = o.borrow_mut();
            pushcore::push_tick(
                &mut st.push,
                &mut st.pidx,
                world as u16,
                self_id as i64,
                slot,
                self_vehicle != 0,
                self_player != 0, // now carries the enablePlayerCollisions config
                only_players != 0,
                self_player != 0,
                max_ec,
                cramming,
                pushcore::GATE_MIN,
                cur_tick as u64,
                &mut o,
            );
            if o.ok {
                (1i64)
                    | ((o.cramming as i64) << 1)
                    | (((o.non_passengers.max(0) as i64) & 0xF_FFFF) << 2)
                    | (((o.ncol.max(0) as i64) & 0xFFFF_FFFF) << 22)
            } else {
                0i64
            }
        })
    })
    .unwrap_or(0i64);
    if res & 1 == 1 {
        // write the delta statics OUTSIDE the state borrow (server-thread
        // confined statics, no race)
        let (dx, dy, dz) = PUSH_OUT.with(|o| {
            let o = o.borrow();
            (o.dx, o.dy, o.dz)
        });
        unsafe {
            let e = JniEnv::from_raw(env);
            for (name, v) in [
                ("PUSH_SDX", dx),
                ("PUSH_SDY", dy),
                ("PUSH_SDZ", dz),
            ] {
                if let Some(fid) = e.get_static_field_id(cls, name, "D") {
                    let raw = e.raw();
                    let vt = *raw;
                    ((*vt).SetStaticDoubleField)(raw, cls, fid, v);
                }
            }
            if e.exception_check() {
                e.exception_clear();
            }
        }
    }
    res
}

/// The numCollisions mirror value for `id`, or -1 (mirror miss). The Java
/// decline path uses this to re-sync the field before running vanilla.
pub extern "system" fn n_push_ncol(
    _env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    id: i32,
) -> i32 {
    with_state(|st| match st.push.slot_of(id as i64) {
        Some(s) => st.push.ncol[s],
        None => -1,
    })
    .unwrap_or(-1)
}

// bridge class bytes (compiled by build.rs against the paper jar)
static HOOKS_CLASS_BYTES: &[u8] = include_bytes!(concat!(env!("CE_HOOKS_CLASS")));
static HOOKS_BLOB: &[u8] = include_bytes!(concat!(env!("CE_HOOKS_BLOB")));

/// All compiled bridge classes: (internal name, bytes). The bridge may
/// declare nested classes — each must be defined into the SAME loader.
fn hooks_classes() -> Vec<(&'static str, &'static [u8])> {
    let mut out = Vec::new();
    let b = HOOKS_BLOB;
    let mut i = 0usize;
    let count = u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as usize;
    i = 4;
    for _ in 0..count {
        let nl = u16::from_be_bytes([b[i], b[i + 1]]) as usize;
        i += 2;
        let name = std::str::from_utf8(&b[i..i + nl]).expect("utf8 name");
        i += nl;
        let cl = u32::from_be_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]) as usize;
        i += 4;
        out.push((name, &b[i..i + cl]));
        i += cl;
    }
    out
}

/// Off-thread upsert (worldgen workers): queued, drained by the next
/// server-thread apply so the index and the published rows keep a single
/// writer thread.
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
    push_direct(DirEv::Upsert {
        id,
        wid: world as u16,
        b,
    });
}

/// Off-thread remove (worldgen workers): queued like n_box.
pub extern "system" fn n_remove(_env: *mut jni::JNIEnv, _cls: jni::jclass, id: i64) {
    push_direct(DirEv::Remove { id });
}

/// Full index drop (parity-gate suspend/resume, iteration 4): the Java gate
/// suspends indexing when the fast path is measurably useless for the current
/// traffic; on resume it fully re-syncs the index from the live lookups, so
/// the native side only needs to forget everything it had.
pub extern "system" fn n_reset(env: *mut jni::JNIEnv, cls: jni::jclass) {
    if let Ok(mut g) = INDEX.lock() {
        *g = None; // lazily re-created by the next apply/query
    }
    // Invalidate EVERY published row: PUBLISH_SEQ jumps past all stamps ever
    // written (rows carry seq << 11, so +2^40 can never collide with an old
    // stamp). The next publisher instance continues from NEXT_SEQ. Rare path
    // (suspend/resync) — resolve the field locally.
    unsafe {
        let e = JniEnv::from_raw(env);
        if let Some(fid) = e.get_static_field_id(cls, "PUBLISH_SEQ", "J") {
            let raw = e.raw();
            let vt = *raw;
            let cur = ((*vt).GetStaticLongField)(raw, cls, fid);
            let next = cur.wrapping_add(1 << 40);
            ((*vt).SetStaticLongField)(raw, cls, fid, next);
            NEXT_SEQ.store(next, Ordering::Release);
        }
    }
}
