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
#[doc(hidden)]
pub mod patch;
pub mod pushcore;

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
const GOAL_SELECTOR_CLASS: &str = "net/minecraft/world/entity/ai/goal/GoalSelector";
const MOB_CLASS: &str = "net/minecraft/world/entity/Mob";
const CONVERSION_TRACKER_CLASS: &str = "net/minecraft/world/entity/ConversionTracker";
const SENSING_CLASS: &str = "net/minecraft/world/entity/ai/sensing/Sensing";
const TRACKED_ENTITY_CLASS: &str = "net/minecraft/server/level/ChunkMap$TrackedEntity";
/// L30: the tracker-loop floor target — ChunkMap.newTrackerTick replace_body.
const CHUNKMAP_CLASS: &str = "net/minecraft/server/level/ChunkMap";
// L36: the idle AI-control classes (nav/move/look early-out guards).
const NAVIGATION_CLASS: &str = "net/minecraft/world/entity/ai/navigation/PathNavigation";
const MOVE_CONTROL_CLASS: &str = "net/minecraft/world/entity/ai/control/MoveControl";
const LOOK_CONTROL_CLASS: &str = "net/minecraft/world/entity/ai/control/LookControl";
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
    cplug_sdk::hooks::register_bytes(GOAL_SELECTOR_CLASS, |_n, b| patch_goalselector(b));
    cplug_sdk::hooks::register_bytes(MOB_CLASS, |_n, b| patch_mob(b));
    cplug_sdk::hooks::register_bytes(CONVERSION_TRACKER_CLASS, |_n, b| patch_conversion(b));
    cplug_sdk::hooks::register_bytes(TRACKED_ENTITY_CLASS, |_n, b| patch_trackedentity(b));
    cplug_sdk::hooks::register_bytes(CHUNKMAP_CLASS, |_n, b| patch_chunkmap(b));
    cplug_sdk::hooks::register_bytes(SENSING_CLASS, |_n, b| patch_sensing(b));
    cplug_sdk::hooks::register_bytes(NAVIGATION_CLASS, |_n, b| patch_navigation(b));
    cplug_sdk::hooks::register_bytes(MOVE_CONTROL_CLASS, |_n, b| patch_movecontrol(b));
    cplug_sdk::hooks::register_bytes(LOOK_CONTROL_CLASS, |_n, b| patch_lookcontrol(b));

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

// ---------------------------------------------------------------------------
// L36 — idle AI-control guards (nav/move/look early-outs).
//
// Three INDEPENDENT no-arg-instance-method guards (op-4, receiver-as-local-0
// — the tickEffects lesson): PathNavigation.tick / MoveControl.tick /
// LookControl.tick. The Java checks (navIdle$/moveIdle$/lookIdle$) return
// true only when the javap-proven no-op conditions hold; any doubt routes
// the exact vanilla body. A refusal of one guard keeps the other two.
// ---------------------------------------------------------------------------

static NAV_WOVEN: AtomicBool = AtomicBool::new(false);
static NAV_DECLINED: AtomicBool = AtomicBool::new(false);
static MOVE_WOVEN: AtomicBool = AtomicBool::new(false);
static MOVE_DECLINED: AtomicBool = AtomicBool::new(false);
static LOOK_WOVEN: AtomicBool = AtomicBool::new(false);
static LOOK_DECLINED: AtomicBool = AtomicBool::new(false);

fn patch_navigation(bytes: &[u8]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| patch_idle_inner(bytes, NAVIGATION_CLASS, &NAV_WOVEN))
        .ok()
        .flatten()
}

fn patch_movecontrol(bytes: &[u8]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| patch_idle_inner(bytes, MOVE_CONTROL_CLASS, &MOVE_WOVEN))
        .ok()
        .flatten()
}

fn patch_lookcontrol(bytes: &[u8]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| patch_idle_inner(bytes, LOOK_CONTROL_CLASS, &LOOK_WOVEN))
        .ok()
        .flatten()
}

fn patch_idle_inner(bytes: &[u8], cls: &'static str, woven: &AtomicBool) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        let mut st = STASH.lock().ok()?;
        st.entry(cls).or_insert_with(|| bytes.to_vec());
        return None;
    }
    let pre = PRECOMPUTED.lock().ok()?.get(cls).cloned();
    match pre {
        Some(b) if !b.is_empty() => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            Some(b)
        }
        _ if !woven.load(Ordering::Acquire) => {
            // post-arm load (or a pending precompute) — stash for the
            // verifier's deferred weave, keep the class vanilla meanwhile.
            let mut st = STASH.lock().ok()?;
            st.entry(cls).or_insert_with(|| bytes.to_vec());
            None
        }
        _ => None,
    }
}

fn precompute_idle_controls(loader: jni::jobject) {
    // (class, woven, declined, java check name, java check desc, init, ok flag,
    //  log label). Each entry is fully independent.
    type IdleSpec = (
        &'static str,
        &'static AtomicBool,
        &'static AtomicBool,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    );
    let specs: [IdleSpec; 3] = [
        (
            NAVIGATION_CLASS,
            &NAV_WOVEN,
            &NAV_DECLINED,
            "navIdle$",
            "(Lnet/minecraft/world/entity/ai/navigation/PathNavigation;)Z",
            "initNavHandles",
            "NAV_HANDLES_OK",
            "navIdle",
        ),
        (
            MOVE_CONTROL_CLASS,
            &MOVE_WOVEN,
            &MOVE_DECLINED,
            "moveIdle$",
            "(Lnet/minecraft/world/entity/ai/control/MoveControl;)Z",
            "initMoveHandles",
            "MOVE_HANDLES_OK",
            "moveIdle",
        ),
        (
            LOOK_CONTROL_CLASS,
            &LOOK_WOVEN,
            &LOOK_DECLINED,
            "lookIdle$",
            "(Lnet/minecraft/world/entity/ai/control/LookControl;)Z",
            "initLookHandles",
            "LOOK_HANDLES_OK",
            "lookIdle",
        ),
    ];
    for (cls, woven, declined, check_name, check_desc, init, ok_flag, label) in specs {
        if woven.load(Ordering::Acquire) || declined.load(Ordering::Acquire) {
            continue;
        }
        // Property kill switch (the CeHooks FLAG_ON static is folded at class
        // init from the same property — this read just mirrors the decision
        // in the rust log).
        let flag_field = match label {
            "navIdle" => "NAV_FLAG_ON",
            "moveIdle" => "MOVE_FLAG_ON",
            _ => "LOOK_FLAG_ON",
        };
        if read_java_flag(flag_field) != Some(true) {
            declined.store(true, Ordering::Release);
            cplug_sdk::log::info(&format!(
                "[c-entities] {label} guard disabled (property) — vanilla kept"
            ));
            continue;
        }
        if prove_handles(init, ok_flag) != Some(true) {
            declined.store(true, Ordering::Release);
            cplug_sdk::log::info(&format!(
                "[c-entities] {label} guard handle probe failed — vanilla kept"
            ));
            continue;
        }
        let Some(bytes) = STASH.lock().ok().and_then(|mut s| s.remove(cls)) else {
            // not loaded yet — leave UNDECIDED (the verifier retries when
            // the class loads); never decline on a missing stash.
            continue;
        };
        let spec = cplug_sdk::asm::EarlyReturnGuard {
            method_name: "tick",
            method_desc: "()V",
            check_class: HOOKS_CLASS,
            check_name,
            check_desc,
        };
        let patched = cplug_sdk::jni_util::with_attached(|env| {
            cplug_sdk::asm::insert_early_return_guard(env, loader, &bytes, &spec)
        });
        match patched {
            Some(Some(b)) if !b.is_empty() => {
                if let Ok(mut pc) = PRECOMPUTED.lock() {
                    pc.insert(cls, b);
                }
                woven.store(true, Ordering::Release);
                cplug_sdk::log::info(&format!(
                    "[c-entities] precomputed {label} idle guard"
                ));
            }
            _ => {
                declined.store(true, Ordering::Release);
                cplug_sdk::log::info(&format!(
                    "[c-entities] FAILED to precompute {label} guard — vanilla kept"
                ));
            }
        }
    }
}

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
    precompute_entity(loader);
    precompute_goalselector(loader);
    precompute_mob(loader);
    precompute_conversion(loader);
    precompute_sensing(loader);
    precompute_idle_controls(loader);
    precompute_trackedentity(loader);
    precompute_chunkmap(loader);

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
        GOAL_SELECTOR_CLASS,
        MOB_CLASS,
        CONVERSION_TRACKER_CLASS,
        TRACKED_ENTITY_CLASS,
        CHUNKMAP_CLASS,
        SENSING_CLASS,
        NAVIGATION_CLASS,
        MOVE_CONTROL_CLASS,
        LOOK_CONTROL_CLASS,
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
                // L15 deferred weaves: the GoalSelector/ConversionTracker
                // classes load with the first mobs — usually AFTER the arm.
                if !GOAL_WOVEN.load(Ordering::Acquire)
                    && !GOAL_DECLINED.load(Ordering::Acquire)
                    && STASH
                        .lock()
                        .map(|s| s.contains_key(GOAL_SELECTOR_CLASS))
                        .unwrap_or(false)
                {
                    precompute_goalselector(loader as jni::jobject);
                    if GOAL_WOVEN.load(Ordering::Acquire) {
                        let rc = cplug_sdk::retransform_class(GOAL_SELECTOR_CLASS);
                        cplug_sdk::log::info(&format!(
                            "[c-entities] verifier: deferred GoalSelector floor applied, rc={rc}"
                        ));
                    }
                }
                // L22a deferred weave: Mob loads with the first mobs too.
                if !MOB_WOVEN.load(Ordering::Acquire)
                    && !MOB_DECLINED.load(Ordering::Acquire)
                    && STASH
                        .lock()
                        .map(|s| s.contains_key(MOB_CLASS))
                        .unwrap_or(false)
                {
                    precompute_mob(loader as jni::jobject);
                    if MOB_WOVEN.load(Ordering::Acquire) {
                        let rc = cplug_sdk::retransform_class(MOB_CLASS);
                        cplug_sdk::log::info(&format!(
                            "[c-entities] verifier: deferred Mob AI floor applied, rc={rc}"
                        ));
                    }
                }
                if !FROZEN_WOVEN.load(Ordering::Acquire)
                    && !FROZEN_DECLINED.load(Ordering::Acquire)
                    && STASH
                        .lock()
                        .map(|s| s.contains_key(ENTITY_CLASS))
                        .unwrap_or(false)
                {
                    precompute_entity(loader as jni::jobject);
                    // retransform is idempotent and safe even when only the
                    // probes were re-prepared (no gate) — apply unconditionally
                    let rc = cplug_sdk::retransform_class(ENTITY_CLASS);
                    cplug_sdk::log::info(&format!(
                        "[c-entities] verifier: deferred Entity weave re-applied, rc={rc}"
                    ));
                }
                if !CONV_WOVEN.load(Ordering::Acquire)
                    && !CONV_DECLINED.load(Ordering::Acquire)
                    && STASH
                        .lock()
                        .map(|s| s.contains_key(CONVERSION_TRACKER_CLASS))
                        .unwrap_or(false)
                {
                    precompute_conversion(loader as jni::jobject);
                    if CONV_WOVEN.load(Ordering::Acquire) {
                        let rc = cplug_sdk::retransform_class(CONVERSION_TRACKER_CLASS);
                        cplug_sdk::log::info(&format!(
                            "[c-entities] verifier: deferred ConversionTracker gate applied, rc={rc}"
                        ));
                    }
                }
                // L26 deferred weave: Sensing loads with the first mobs.
                if !SENSING_WOVEN.load(Ordering::Acquire)
                    && !SENSING_DECLINED.load(Ordering::Acquire)
                    && STASH
                        .lock()
                        .map(|s| s.contains_key(SENSING_CLASS))
                        .unwrap_or(false)
                {
                    precompute_sensing(loader as jni::jobject);
                    if SENSING_WOVEN.load(Ordering::Acquire) {
                        let rc = cplug_sdk::retransform_class(SENSING_CLASS);
                        cplug_sdk::log::info(&format!(
                            "[c-entities] verifier: deferred Sensing floor applied, rc={rc}"
                        ));
                    }
                }
                if !TT_WOVEN.load(Ordering::Acquire)
                    && !TT_DECLINED.load(Ordering::Acquire)
                    && STASH
                        .lock()
                        .map(|s| s.contains_key(TRACKED_ENTITY_CLASS))
                        .unwrap_or(false)
                {
                    precompute_trackedentity(loader as jni::jobject);
                    if TT_WOVEN.load(Ordering::Acquire) {
                        let rc = cplug_sdk::retransform_class(TRACKED_ENTITY_CLASS);
                        cplug_sdk::log::info(&format!(
                            "[c-entities] verifier: deferred TrackedEntity gate applied, rc={rc}"
                        ));
                    }
                }
                // L30 deferred weave: ChunkMap may load post-arm on some
                // boot orders — same stash/retry protocol as TrackedEntity.
                if !TTL_WOVEN.load(Ordering::Acquire)
                    && !TTL_DECLINED.load(Ordering::Acquire)
                    && STASH
                        .lock()
                        .map(|s| s.contains_key(CHUNKMAP_CLASS))
                        .unwrap_or(false)
                {
                    precompute_chunkmap(loader as jni::jobject);
                    if TTL_WOVEN.load(Ordering::Acquire) {
                        let rc = cplug_sdk::retransform_class(CHUNKMAP_CLASS);
                        cplug_sdk::log::info(&format!(
                            "[c-entities] verifier: deferred tracker-loop floor applied, rc={rc}"
                        ));
                    }
                }
                // L36 deferred weave: the AI-control classes load with the
                // first mobs (same wave as Sensing) — same stash/retry
                // protocol. Each guard is independent; retransform only the
                // classes whose weave actually landed.
                precompute_idle_controls(loader as jni::jobject);
                for (cls, woven) in [
                    (NAVIGATION_CLASS, &NAV_WOVEN),
                    (MOVE_CONTROL_CLASS, &MOVE_WOVEN),
                    (LOOK_CONTROL_CLASS, &LOOK_WOVEN),
                ] {
                    if woven.load(Ordering::Acquire) {
                        let rc = cplug_sdk::retransform_class(cls);
                        cplug_sdk::log::info(&format!(
                            "[c-entities] verifier: deferred idle guard applied to {cls}, rc={rc}"
                        ));
                    }
                }
            }
            let snap = read_hook_counters();
            // L15 arming: flip a gate ON only when its replaced body is
            // PROVEN live (its liveness counter moved — the GoalSelector
            // bodies bump gsMirrorTicks while disarmed; frozen/conv bump
            // frozenCalls/convCalls on every entry). PLAIN longs may lag a
            // moment behind on another core — the verifier retries anyway.
            if GOAL_WOVEN.load(Ordering::Acquire)
                && !AI_ARMED.load(Ordering::Acquire)
                && snap.get(&Counter::GoalChecks).copied().unwrap_or(0) > 0
            {
                if set_java_bool("setAiFloorOn", true) {
                    AI_ARMED.store(true, Ordering::Release);
                    cplug_sdk::log::info("[c-entities] AI floor ARMED (verified live)");
                }
            }
            if FROZEN_WOVEN.load(Ordering::Acquire)
                && !FROZEN_ARMED.load(Ordering::Acquire)
                && snap.get(&Counter::FrozenChecks).copied().unwrap_or(0) > 0
            {
                if set_java_bool("setFrozenGateOn", true) {
                    FROZEN_ARMED.store(true, Ordering::Release);
                    cplug_sdk::log::info("[c-entities] frozen gate ARMED (verified live)");
                }
            }
            if CONV_WOVEN.load(Ordering::Acquire)
                && !CONV_ARMED.load(Ordering::Acquire)
                && snap.get(&Counter::ConvChecks).copied().unwrap_or(0) > 0
            {
                if set_java_bool("setConvGateOn", true) {
                    CONV_ARMED.store(true, Ordering::Release);
                    cplug_sdk::log::info("[c-entities] conv gate ARMED (verified live)");
                }
            }
            if TT_WOVEN.load(Ordering::Acquire)
                && !TT_ARMED.load(Ordering::Acquire)
                && snap.get(&Counter::TTChecks).copied().unwrap_or(0) > 0
            {
                if set_java_bool("setTrackerTickGateOn", true) {
                    TT_ARMED.store(true, Ordering::Release);
                    cplug_sdk::log::info("[c-entities] tracker-tick gate ARMED (verified live)");
                }
            }
            // L30 arming: ttlCalls bumps on every newTrackerTick$ entry even
            // with the fast path disarmed (the mirror IS the body) — the
            // counter proves the replaced loop is live before the fast path
            // is allowed to fire.
            if TTL_WOVEN.load(Ordering::Acquire)
                && !TTL_ARMED.load(Ordering::Acquire)
                && snap.get(&Counter::TTLoopChecks).copied().unwrap_or(0) > 0
            {
                if set_java_bool("setTrackerLoopOn", true) {
                    TTL_ARMED.store(true, Ordering::Release);
                    cplug_sdk::log::info("[c-entities] tracker-loop floor ARMED (verified live)");
                }
            }
            // L33 arming: effCalls bumps on every tickEffectsNoOp entry even
            // disarmed (the gate declines and the vanilla body runs) — the
            // counter proves the woven call is live.
            if EFF_WOVEN.load(Ordering::Acquire)
                && !EFF_ARMED.load(Ordering::Acquire)
                && snap.get(&Counter::EffChecks).copied().unwrap_or(0) > 0
            {
                if set_java_bool("setTickEffectsGateOn", true) {
                    EFF_ARMED.store(true, Ordering::Release);
                    cplug_sdk::log::info("[c-entities] tickEffects gate ARMED (verified live)");
                }
            }
            if FLAGG_WOVEN.load(Ordering::Acquire)
                && !FLAGG_ARMED.load(Ordering::Acquire)
                && snap.get(&Counter::FlagChecks).copied().unwrap_or(0) > 0
            {
                if set_java_bool("setFlagGateOn", true) {
                    FLAGG_ARMED.store(true, Ordering::Release);
                    cplug_sdk::log::info("[c-entities] flag gate ARMED (verified live)");
                }
            }
            if DELTA_WOVEN.load(Ordering::Acquire)
                && !DELTA_ARMED.load(Ordering::Acquire)
                && snap.get(&Counter::DeltaChecks).copied().unwrap_or(0) > 0
            {
                if set_java_bool("setDeltaGateOn", true) {
                    DELTA_ARMED.store(true, Ordering::Release);
                    cplug_sdk::log::info("[c-entities] delta gate ARMED (verified live)");
                }
            }
            if SENSING_WOVEN.load(Ordering::Acquire)
                && !SENSING_ARMED.load(Ordering::Acquire)
                && snap.get(&Counter::SenseChecks).copied().unwrap_or(0) > 0
            {
                if set_java_bool("setSenseOn", true) {
                    SENSING_ARMED.store(true, Ordering::Release);
                    cplug_sdk::log::info("[c-entities] sensing floor ARMED (verified live)");
                }
            }
            if FLAGR_WOVEN.load(Ordering::Acquire)
                && !FLAGR_ARMED.load(Ordering::Acquire)
                && snap.get(&Counter::FlagRChecks).copied().unwrap_or(0) > 0
            {
                if set_java_bool("setFlagROn", true) {
                    FLAGR_ARMED.store(true, Ordering::Release);
                    cplug_sdk::log::info("[c-entities] flag-read floor ARMED (verified live)");
                }
            }
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
/// L15: the AI floor (GoalSelector microkernel) weave is prepared. The floor
/// itself arms LATER — the verifier flips CeHooks.setAiFloorOn(true) only
/// after the replaced bodies are proven live (gsMirrorTicks moves).
static GOAL_WOVEN: AtomicBool = AtomicBool::new(false);
static GOAL_DECLINED: AtomicBool = AtomicBool::new(false);
static AI_ARMED: AtomicBool = AtomicBool::new(false);
/// L15: the Entity.setTicksFrozen no-op gate weave is prepared.
static FROZEN_WOVEN: AtomicBool = AtomicBool::new(false);
static FROZEN_DECLINED: AtomicBool = AtomicBool::new(false);
static FROZEN_ARMED: AtomicBool = AtomicBool::new(false);
/// L15: the ConversionTracker.setConverting no-op gate weave is prepared.
static CONV_WOVEN: AtomicBool = AtomicBool::new(false);
static CONV_DECLINED: AtomicBool = AtomicBool::new(false);
static CONV_ARMED: AtomicBool = AtomicBool::new(false);
/// L15c: the TrackedEntity.moonrise$tick no-op gate weave is prepared.
static TT_WOVEN: AtomicBool = AtomicBool::new(false);
static TT_DECLINED: AtomicBool = AtomicBool::new(false);
static TT_ARMED: AtomicBool = AtomicBool::new(false);
/// L30: the ChunkMap.newTrackerTick tracker-loop floor weave state.
static TTL_WOVEN: AtomicBool = AtomicBool::new(false);
static TTL_DECLINED: AtomicBool = AtomicBool::new(false);
static TTL_ARMED: AtomicBool = AtomicBool::new(false);
/// L33: the LivingEntity.tickEffects empty-map early-out weave state.
static EFF_WOVEN: AtomicBool = AtomicBool::new(false);
static EFF_DECLINED: AtomicBool = AtomicBool::new(false);
static EFF_ARMED: AtomicBool = AtomicBool::new(false);
/// L18: the Entity.setSharedFlag no-op-write gate weave is prepared.
static FLAGG_WOVEN: AtomicBool = AtomicBool::new(false);
static FLAGG_DECLINED: AtomicBool = AtomicBool::new(false);
static FLAGG_ARMED: AtomicBool = AtomicBool::new(false);
/// L20b: the Entity.setDeltaMovement same-value no-op gate weave is prepared.
static DELTA_WOVEN: AtomicBool = AtomicBool::new(false);
static DELTA_DECLINED: AtomicBool = AtomicBool::new(false);
static BLOCKFX_WOVEN: AtomicBool = AtomicBool::new(false);
static BLOCKFX_DECLINED: AtomicBool = AtomicBool::new(false);
static DELTA_ARMED: AtomicBool = AtomicBool::new(false);
/// L22a: the Mob.serverAiStep native AI floor weave state.
static MOB_WOVEN: AtomicBool = AtomicBool::new(false);
static MOB_DECLINED: AtomicBool = AtomicBool::new(false);
static MOB_ARMED: AtomicBool = AtomicBool::new(false);
/// L26: the Sensing.tick O(1)-clear floor weave state.
static SENSING_WOVEN: AtomicBool = AtomicBool::new(false);
static SENSING_DECLINED: AtomicBool = AtomicBool::new(false);
static SENSING_ARMED: AtomicBool = AtomicBool::new(false);
/// L26: the Entity.getSharedFlag direct-read floor weave state.
static FLAGR_WOVEN: AtomicBool = AtomicBool::new(false);
static FLAGR_DECLINED: AtomicBool = AtomicBool::new(false);
static FLAGR_ARMED: AtomicBool = AtomicBool::new(false);

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
    let push_spec = cplug_sdk::asm::EarlyReturnGuard {
        method_name: "pushEntities",
        method_desc: "()V",
        check_class: HOOKS_CLASS,
        check_name: "pushDispatch",
        check_desc: "(Lnet/minecraft/world/entity/Entity;)Z",
    };
    // L20 quiet-travel floor: the same class carries the second guard.
    // Op 5 (EARLY_RETURN_GUARD2) is REQUIRED here: the op-4 helper guesses
    // the receiver slot from the descriptor (a reference first-param reads
    // as local 1), which is correct only for no-arg instance methods and
    // statics — travel(Vec3) loaded the VEC3 as the receiver and the JVM
    // rejected the class (FAILS_VERIFICATION, "Vec3 is not assignable to
    // LivingEntity", javap-confirmed bci@1). Op 5 loads local 0 explicitly.
    let travel_spec = cplug_sdk::asm::EarlyReturnGuard2 {
        method_name: "travel",
        method_desc: "(Lnet/minecraft/world/phys/Vec3;)V",
        check_class: HOOKS_CLASS,
        check_name: "mobTravelNative",
        check_desc: "(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/phys/Vec3;)Z",
    };
    // L33: the tickEffects empty-map early-out — a THIRD independent guard
    // on the same class, chained after whatever the mode selection kept.
    // tickEffects()V is a NO-ARG instance method — op-4 (receiver-only,
    // the descriptor's reference first-param reads as local 0) is the
    // correct op; op-5 (guard2) refuses it ("bad first param") because it
    // demands a real first argument (the same lesson as the travel note
    // below, mirrored).
    let eff_ok = read_java_flag("EFF_FLAG_ON") == Some(true)
        && prove_handles("initEffHandles", "EFF_HANDLES_OK") == Some(true);
    if !eff_ok {
        EFF_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] tickEffects gate declined (flag/handles) — vanilla kept",
        );
    }
    let eff_spec = cplug_sdk::asm::EarlyReturnGuard {
        method_name: "tickEffects",
        method_desc: "()V",
        check_class: HOOKS_CLASS,
        check_name: "tickEffectsNoOp",
        check_desc: "(Lnet/minecraft/world/entity/LivingEntity;)Z",
    };
    let patched = cplug_sdk::jni_util::with_attached(|env| {
        // CE_WEAVE_MODE=push|travel|both (default both) — the bisect lever
        // for the two-guard FAILS_VERIFICATION hunt
        let mode = std::env::var("CE_WEAVE_MODE").unwrap_or_else(|_| "both".into());
        let base = match mode.as_str() {
            "push" => cplug_sdk::asm::insert_early_return_guard(env, loader, &bytes, &push_spec),
            "travel" => {
                cplug_sdk::asm::insert_early_return_guard2(env, loader, &bytes, &travel_spec)
            }
            _ => {
                // The two guards are INDEPENDENT prefixes on the same class:
                // a refusal of one keeps the other (both are optional weaves).
                let Some(b1) =
                    cplug_sdk::asm::insert_early_return_guard(env, loader, &bytes, &push_spec)
                else {
                    return cplug_sdk::asm::insert_early_return_guard2(
                        env,
                        loader,
                        &bytes,
                        &travel_spec,
                    );
                };
                let Some(b2) =
                    cplug_sdk::asm::insert_early_return_guard2(env, loader, &b1, &travel_spec)
                else {
                    return Some(b1); // the push guard alone
                };
                Some(b2)
            }
        };
        // L33 chain: a refusal of the tickEffects guard keeps the base
        // guards (all three are optional weaves).
        if !eff_ok {
            return base;
        }
        match base {
            Some(b) => match cplug_sdk::asm::insert_early_return_guard(
                env,
                loader,
                &b,
                &eff_spec,
            ) {
                Some(b2) => {
                    EFF_WOVEN.store(true, Ordering::Release);
                    Some(b2)
                }
                None => Some(b),
            },
            None => {
                // no base guard survived; the eff guard alone is still worth
                // applying — vanilla keeps the other two bodies.
                match cplug_sdk::asm::insert_early_return_guard(
                    env,
                    loader,
                    &bytes,
                    &eff_spec,
                ) {
                    Some(b2) => {
                        EFF_WOVEN.store(true, Ordering::Release);
                        Some(b2)
                    }
                    None => None,
                }
            }
        }
    });
    match patched {
        Some(Some(b)) if !b.is_empty() => {
            if let Ok(mut pc) = PRECOMPUTED.lock() {
                pc.insert(LIVING_ENTITY_CLASS, b.clone());
            }
            // CE_DUMP_CLASSES=/dir: dump the patched bytes for standalone
            // verifier diagnosis (the two-guard FAILS_VERIFICATION hunt)
            if let Ok(dir) = std::env::var("CE_DUMP_CLASSES") {
                let _ = std::fs::write(format!("{}/LivingEntity.patched.class", dir), &b);
            }
            if EFF_WOVEN.load(Ordering::Acquire) {
                cplug_sdk::log::info(
                    "[c-entities] precomputed LivingEntity push + quiet-travel + tickEffects guards",
                );
            } else {
                cplug_sdk::log::info(
                    "[c-entities] precomputed LivingEntity push + quiet-travel guards",
                );
            }
        }
        _ => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(
                "[c-entities] FAILED to precompute LivingEntity guards — vanilla kept",
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

/// L15: GoalSelector byte hook — stash before the arm / serve precomputed
/// after the retransform. The bodies themselves carry the pre-arm mirror.
fn patch_goalselector(bytes: &[u8]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| patch_goalselector_inner(bytes))
        .ok()
        .flatten()
}

/// L22a: Mob byte hook — same stash/serve pattern as GoalSelector.
fn patch_mob(bytes: &[u8]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| patch_mob_inner(bytes))
        .ok()
        .flatten()
}

fn patch_mob_inner(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        let mut st = STASH.lock().ok()?;
        st.entry(MOB_CLASS).or_insert_with(|| bytes.to_vec());
        return None;
    }
    let pre = PRECOMPUTED.lock().ok()?.get(MOB_CLASS).cloned();
    match pre {
        Some(b) if !b.is_empty() => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] patched Mob AI floor guard");
            Some(b)
        }
        _ if !MOB_WOVEN.load(Ordering::Acquire) && !MOB_DECLINED.load(Ordering::Acquire) => {
            let mut st = STASH.lock().ok()?;
            st.entry(MOB_CLASS).or_insert_with(|| bytes.to_vec());
            cplug_sdk::log::info("[c-entities] Mob bytes stashed post-arm — verifier will decide");
            None
        }
        _ => None,
    }
}

fn patch_goalselector_inner(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        let mut st = STASH.lock().ok()?;
        st.entry(GOAL_SELECTOR_CLASS)
            .or_insert_with(|| bytes.to_vec());
        return None;
    }
    let pre = PRECOMPUTED.lock().ok()?.get(GOAL_SELECTOR_CLASS).cloned();
    match pre {
        Some(b) if !b.is_empty() => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] patched GoalSelector microkernel");
            Some(b)
        }
        _ if !GOAL_WOVEN.load(Ordering::Acquire) && !GOAL_DECLINED.load(Ordering::Acquire) => {
            let mut st = STASH.lock().ok()?;
            st.entry(GOAL_SELECTOR_CLASS)
                .or_insert_with(|| bytes.to_vec());
            cplug_sdk::log::info(
                "[c-entities] GoalSelector bytes stashed post-arm — verifier will decide",
            );
            None
        }
        _ => None,
    }
}

/// L15: ConversionTracker byte hook — same stash/serve pattern.
fn patch_conversion(bytes: &[u8]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| patch_conversion_inner(bytes))
        .ok()
        .flatten()
}

fn patch_conversion_inner(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        let mut st = STASH.lock().ok()?;
        st.entry(CONVERSION_TRACKER_CLASS)
            .or_insert_with(|| bytes.to_vec());
        return None;
    }
    let pre = PRECOMPUTED
        .lock()
        .ok()?
        .get(CONVERSION_TRACKER_CLASS)
        .cloned();
    match pre {
        Some(b) if !b.is_empty() => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] patched ConversionTracker conv gate");
            Some(b)
        }
        _ if !CONV_WOVEN.load(Ordering::Acquire) && !CONV_DECLINED.load(Ordering::Acquire) => {
            let mut st = STASH.lock().ok()?;
            st.entry(CONVERSION_TRACKER_CLASS)
                .or_insert_with(|| bytes.to_vec());
            cplug_sdk::log::info(
                "[c-entities] ConversionTracker bytes stashed post-arm — verifier will decide",
            );
            None
        }
        _ => None,
    }
}

/// L26: Sensing byte hook — same stash/serve pattern as GoalSelector.
fn patch_sensing(bytes: &[u8]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| patch_sensing_inner(bytes))
        .ok()
        .flatten()
}

fn patch_sensing_inner(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        let mut st = STASH.lock().ok()?;
        st.entry(SENSING_CLASS).or_insert_with(|| bytes.to_vec());
        return None;
    }
    let pre = PRECOMPUTED.lock().ok()?.get(SENSING_CLASS).cloned();
    match pre {
        Some(b) if !b.is_empty() => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] patched Sensing O(1)-clear floor");
            Some(b)
        }
        _ if !SENSING_WOVEN.load(Ordering::Acquire)
            && !SENSING_DECLINED.load(Ordering::Acquire) =>
        {
            let mut st = STASH.lock().ok()?;
            st.entry(SENSING_CLASS).or_insert_with(|| bytes.to_vec());
            cplug_sdk::log::info(
                "[c-entities] Sensing bytes stashed post-arm — verifier will decide",
            );
            None
        }
        _ => None,
    }
}

fn patch_entityticklist_inner(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        let mut st = STASH.lock().ok()?;
        st.entry(ENTITY_TICK_LIST_CLASS)
            .or_insert_with(|| bytes.to_vec());
        return None;
    }
    let pre = PRECOMPUTED
        .lock()
        .ok()?
        .get(ENTITY_TICK_LIST_CLASS)
        .cloned();
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
    let flag_on: Option<bool> = cplug_sdk::jni_util::with_attached(|env| -> Option<bool> {
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
    let handles_ok: Option<bool> = cplug_sdk::jni_util::with_attached(|env| -> Option<bool> {
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

// ---------------------------------------------------------------------------
// L15 — the AI floor + the no-op write gates (doc 26).
//
// Three optional weaves, each with its own kill switch, handle-proof and
// verifier counter:
//   * GoalSelector.tick / tickRunningGoals -> CeHooks.goalSelectorTick{,Running}
//     (the scheduler microkernel; the bodies carry an internal mirror for the
//     pre-arm state, so the weave is provably live BEFORE the floor arms —
//     gsMirrorTicks moves first);
//   * Entity.setTicksFrozen -> CeHooks.setTicksFrozen$ (chained onto the
//     existing Entity probe surgery — one precompute for both);
//   * ConversionTracker.setConverting -> CeHooks.setConverting$.
// Failure policy: OPTIONAL features — a refusal logs + declines and the rest
// of the module arms normally (a failed AI floor must never disable the push
// kernel). PATCHES_FAIL is left untouched here on purpose.
// ---------------------------------------------------------------------------

fn read_java_flag(field: &str) -> Option<bool> {
    cplug_sdk::jni_util::with_attached(|env| -> Option<bool> {
        let hooks_cls = HOOKS_CLASS_GLOBAL.load(Ordering::Acquire);
        if hooks_cls == 0 {
            return None;
        }
        let cls = hooks_cls as jni::jclass;
        let fid = env.get_static_field_id(cls, field, "Z")?;
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
    .flatten()
}

/// Call a no-arg void static on CeHooks (the handle-proof inits) and report
/// whether the referenced *HANDLES_OK flag flipped true.
fn prove_handles(init_method: &str, ok_field: &str) -> Option<bool> {
    cplug_sdk::jni_util::with_attached(|env| -> Option<bool> {
        let hooks_cls = HOOKS_CLASS_GLOBAL.load(Ordering::Acquire);
        if hooks_cls == 0 {
            return None;
        }
        let cls = hooks_cls as jni::jclass;
        let mid = env.get_static_method_id(cls, init_method, "()V")?;
        env.call_static_void_method(cls, mid, &[]);
        if env.exception_check() {
            env.exception_describe();
            env.exception_clear();
            return None;
        }
        let fid = env.get_static_field_id(cls, ok_field, "Z")?;
        let raw = env.raw();
        // SAFETY: `raw` is a live JNI env inside this attach scope; the
        // vtable deref mirrors the counter reads in read_hook_counters.
        let v = unsafe {
            let vt = *raw;
            ((*vt).GetStaticBooleanField)(raw, cls, fid)
        };
        Some(v != 0)
    })
    .flatten()
}

/// Flip a boolean static setter on CeHooks (the L15 arm switches). Returns
/// false on any JNI trouble — the caller then retries on a later pass.
fn set_java_bool(setter: &str, value: bool) -> bool {
    cplug_sdk::jni_util::with_attached(|env| -> bool {
        let hooks_cls = HOOKS_CLASS_GLOBAL.load(Ordering::Acquire);
        if hooks_cls == 0 {
            return false;
        }
        let cls = hooks_cls as jni::jclass;
        let Some(mid) = env.get_static_method_id(cls, setter, "(Z)V") else {
            return false;
        };
        env.call_static_void_method(cls, mid, &[jni::jvalue { z: value as u8 }]);
        if env.exception_check() {
            env.exception_describe();
            env.exception_clear();
            return false;
        }
        true
    })
    .unwrap_or(false)
}

fn goal_tick_spec() -> cplug_sdk::asm::ReplaceBody<'static> {
    cplug_sdk::asm::ReplaceBody {
        method_name: "tick",
        method_desc: "()V",
        bridge_owner: HOOKS_CLASS,
        bridge_name: "goalSelectorTick",
        bridge_desc: "(Lnet/minecraft/world/entity/ai/goal/GoalSelector;)V",
        args: &[cplug_sdk::asm::ArgSpec::Local { slot: 0, ty: b'L' }],
    }
}

fn goal_tick_running_spec() -> cplug_sdk::asm::ReplaceBody<'static> {
    cplug_sdk::asm::ReplaceBody {
        method_name: "tickRunningGoals",
        method_desc: "(Z)V",
        bridge_owner: HOOKS_CLASS,
        bridge_name: "goalSelectorTickRunning",
        bridge_desc: "(Lnet/minecraft/world/entity/ai/goal/GoalSelector;Z)V",
        args: &[
            cplug_sdk::asm::ArgSpec::Local { slot: 0, ty: b'L' },
            cplug_sdk::asm::ArgSpec::Local { slot: 1, ty: b'Z' },
        ],
    }
}

fn frozen_spec() -> cplug_sdk::asm::ReplaceBody<'static> {
    cplug_sdk::asm::ReplaceBody {
        method_name: "setTicksFrozen",
        method_desc: "(I)V",
        bridge_owner: HOOKS_CLASS,
        bridge_name: "setTicksFrozen$",
        bridge_desc: "(Lnet/minecraft/world/entity/Entity;I)V",
        args: &[
            cplug_sdk::asm::ArgSpec::Local { slot: 0, ty: b'L' },
            cplug_sdk::asm::ArgSpec::Local { slot: 1, ty: b'I' },
        ],
    }
}

fn conv_spec() -> cplug_sdk::asm::ReplaceBody<'static> {
    cplug_sdk::asm::ReplaceBody {
        method_name: "setConverting",
        method_desc: "(Z)V",
        bridge_owner: HOOKS_CLASS,
        bridge_name: "setConverting$",
        bridge_desc: "(Ljava/lang/Object;Z)V",
        args: &[
            cplug_sdk::asm::ArgSpec::Local { slot: 0, ty: b'L' },
            cplug_sdk::asm::ArgSpec::Local { slot: 1, ty: b'Z' },
        ],
    }
}

/// L26: the Sensing.tick floor — the body becomes `CeHooks.sensingTick$`;
/// the first call per Sensing swaps the seen/unseen fields to GenIntSet
/// (O(1) generation-stamped clear), every call then clears in O(1).
fn sensing_spec() -> cplug_sdk::asm::ReplaceBody<'static> {
    cplug_sdk::asm::ReplaceBody {
        method_name: "tick",
        method_desc: "()V",
        bridge_owner: HOOKS_CLASS,
        bridge_name: "sensingTick$",
        bridge_desc: "(Lnet/minecraft/world/entity/ai/sensing/Sensing;)V",
        args: &[cplug_sdk::asm::ArgSpec::Local { slot: 0, ty: b'L' }],
    }
}

/// L26: the Entity.getSharedFlag direct-read floor — the body becomes
/// `CeHooks.getSharedFlag$`; the mirror is the byte-exact vanilla chain.
fn flagr_spec() -> cplug_sdk::asm::ReplaceBody<'static> {
    cplug_sdk::asm::ReplaceBody {
        method_name: "getSharedFlag",
        method_desc: "(I)Z",
        bridge_owner: HOOKS_CLASS,
        bridge_name: "getSharedFlag$",
        bridge_desc: "(Lnet/minecraft/world/entity/Entity;I)Z",
        args: &[
            cplug_sdk::asm::ArgSpec::Local { slot: 0, ty: b'L' },
            cplug_sdk::asm::ArgSpec::Local { slot: 1, ty: b'I' },
        ],
    }
}

/// L18: the shared-flags write gate. `(int, boolean)` — the boolean arg
/// occupies one int slot (slot 2) like every other boolean in the ASM
/// bridge builder.
fn flag_spec() -> cplug_sdk::asm::ReplaceBody<'static> {
    cplug_sdk::asm::ReplaceBody {
        method_name: "setSharedFlag",
        method_desc: "(IZ)V",
        bridge_owner: HOOKS_CLASS,
        bridge_name: "setSharedFlag$",
        bridge_desc: "(Lnet/minecraft/world/entity/Entity;IZ)V",
        args: &[
            cplug_sdk::asm::ArgSpec::Local { slot: 0, ty: b'L' },
            cplug_sdk::asm::ArgSpec::Local { slot: 1, ty: b'I' },
            cplug_sdk::asm::ArgSpec::Local { slot: 2, ty: b'I' },
        ],
    }
}

/// L20b: the same-value delta gate — op 5 (receiver + the Vec3 argument;
/// op 4 would load the Vec3 as the receiver — the LivingEntity.travel
/// lesson, javap-verified).
fn delta_spec() -> cplug_sdk::asm::EarlyReturnGuard2<'static> {
    cplug_sdk::asm::EarlyReturnGuard2 {
        method_name: "setDeltaMovement",
        method_desc: "(Lnet/minecraft/world/phys/Vec3;)V",
        check_class: HOOKS_CLASS,
        check_name: "deltaNoOp",
        check_desc: "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;)Z",
    }
}

/// L22a: the Mob.serverAiStep native AI floor weave. The guard is the
/// read-only-safe mobAiFloor — a refusal keeps vanilla; the internal
/// mirror/fallback paths keep exactness, so arming rides the weave + the
/// handle proof (no separate liveness gate). The engine (Rust) and the
/// L15 GoalSelector floor do not overlap: a floored mob never reaches the
/// GoalSelector bodies.
fn precompute_mob(loader: jni::jobject) {
    if read_java_flag("MOB_AI_ON_INIT") != Some(true) {
        MOB_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] mob AI floor disabled (centities.mobAi) — vanilla serverAiStep kept",
        );
        return;
    }
    if prove_handles("initMobAiHandles", "MOB_AI_HANDLES_OK") != Some(true) {
        MOB_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] mob AI floor handle probe failed — vanilla serverAiStep kept",
        );
        return;
    }
    let Some(bytes) = STASH.lock().ok().and_then(|mut s| s.remove(MOB_CLASS)) else {
        cplug_sdk::log::info(
            "[c-entities] Mob not stashed yet — AI floor deferred to the verifier",
        );
        return;
    };
    let spec = cplug_sdk::asm::EarlyReturnGuard {
        method_name: "serverAiStep",
        method_desc: "()V",
        check_class: HOOKS_CLASS,
        check_name: "mobAiFloor",
        check_desc: "(Lnet/minecraft/world/entity/Mob;)Z",
    };
    let patched = cplug_sdk::jni_util::with_attached(|env| {
        cplug_sdk::asm::insert_early_return_guard(env, loader, &bytes, &spec)
    });
    match patched {
        Some(Some(b)) if !b.is_empty() => {
            if let Ok(mut pc) = PRECOMPUTED.lock() {
                pc.insert(MOB_CLASS, b);
            }
            MOB_WOVEN.store(true, Ordering::Release);
            if set_java_bool("setMobAiOn", true) {
                MOB_ARMED.store(true, Ordering::Release);
                cplug_sdk::log::info("[c-entities] precomputed + armed the Mob AI floor");
            }
        }
        _ => {
            cplug_sdk::log::info(
                "[c-entities] FAILED to precompute the Mob AI floor — vanilla kept",
            );
        }
    }
}

fn precompute_goalselector(loader: jni::jobject) {
    if read_java_flag("AI_FLAG_ON") != Some(true) {
        GOAL_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] AI floor disabled (centities.aiFloor) — vanilla GoalSelector kept",
        );
        return;
    }
    if prove_handles("initAIHandles", "AI_HANDLES_OK") != Some(true) {
        GOAL_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] AI floor handle probe failed — vanilla GoalSelector kept",
        );
        return;
    }
    let Some(bytes) = STASH
        .lock()
        .ok()
        .and_then(|mut s| s.remove(GOAL_SELECTOR_CLASS))
    else {
        // GoalSelector loads with the first mob — frequently AFTER the arm;
        // the boot/post-arm hook stashes the bytes and the verifier retries.
        cplug_sdk::log::info(
            "[c-entities] GoalSelector not loaded yet — AI floor deferred to the verifier",
        );
        return;
    };
    // Two bodies, chained: tick() first, then tickRunningGoals on its output.
    let patched = cplug_sdk::jni_util::with_attached(|env| {
        let one = cplug_sdk::asm::replace_body(env, loader, &bytes, &goal_tick_spec());
        match one {
            Some(b) if !b.is_empty() => {
                cplug_sdk::asm::replace_body(env, loader, &b, &goal_tick_running_spec())
            }
            other => other,
        }
    });
    match patched {
        Some(Some(b)) if !b.is_empty() => {
            if let Ok(mut pc) = PRECOMPUTED.lock() {
                pc.insert(GOAL_SELECTOR_CLASS, b);
            }
            GOAL_WOVEN.store(true, Ordering::Release);
            // L22: arm immediately — the GoalChecks liveness gate is DEAD
            // now that the Mob AI floor owns the zombie serverAiStep (a
            // classified zombie never reaches the GoalSelector bodies, so
            // the counter cannot move). The floor's own mirrors carry the
            // pre-arm exactness; the mob-floor refusal paths keep the L15
            // floor live for every NON-Zombie mob species.
            if set_java_bool("setAiFloorOn", true) {
                AI_ARMED.store(true, Ordering::Release);
                cplug_sdk::log::info("[c-entities] AI floor ARMED (weave verified)");
            }
            cplug_sdk::log::info(
                "[c-entities] precomputed GoalSelector microkernel (tick + tickRunningGoals)",
            );
        }
        _ => {
            cplug_sdk::log::info(
                "[c-entities] FAILED to precompute GoalSelector floor — vanilla kept",
            );
        }
    }
}

/// The Entity.setTicksFrozen gate rides the SAME precomputed Entity bytes as
/// the probe surgery: replace_body first (JNI helper), then the rust probe
/// prefixes on top. `bytes` = the stashed ORIGINAL Entity class file.
fn apply_entity_frozen_and_probes(loader: jni::jobject, bytes: &[u8]) -> Option<Vec<u8>> {
    let gated = if read_java_flag("FROZEN_FLAG_ON") == Some(true)
        && prove_handles("initFrozenHandles", "FROZEN_HANDLES_OK") == Some(true)
    {
        match cplug_sdk::jni_util::with_attached(|env| {
            cplug_sdk::asm::replace_body(env, loader, bytes, &frozen_spec())
        }) {
            Some(Some(b)) if !b.is_empty() => b,
            _ => {
                cplug_sdk::log::info(
                    "[c-entities] setTicksFrozen replace refused — Entity keeps probes only",
                );
                bytes.to_vec()
            }
        }
    } else {
        FROZEN_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] frozen gate disabled/refused (centities.frozenGate) — vanilla write kept",
        );
        bytes.to_vec()
    };
    // L18: the shared-flags gate chains onto the same Entity bytes. The
    // flag gate is OPTIONAL like the frozen one — a refusal keeps vanilla
    // flags writes (and the frozen gate if it already landed).
    let gated = if read_java_flag("FLAGG_FLAG_ON") == Some(true)
        && prove_handles("initFlagHandles", "FLAGG_HANDLES_OK") == Some(true)
    {
        match cplug_sdk::jni_util::with_attached(|env| {
            cplug_sdk::asm::replace_body(env, loader, &gated, &flag_spec())
        }) {
            Some(Some(b)) if !b.is_empty() => {
                FLAGG_WOVEN.store(true, Ordering::Release);
                cplug_sdk::log::info("[c-entities] precomputed setSharedFlag gate");
                b
            }
            _ => {
                FLAGG_DECLINED.store(true, Ordering::Release);
                cplug_sdk::log::info(
                    "[c-entities] setSharedFlag replace refused — vanilla flags writes kept",
                );
                gated
            }
        }
    } else {
        FLAGG_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] flag gate disabled/refused (centities.flagGate) — vanilla writes kept",
        );
        gated
    };
    // L20b: the same-value delta gate chains onto the same Entity bytes
    // (before the probe prefixes — the gate must run FIRST so a skipped
    // write also skips the L12 probe mirror and the worldId tax).
    let gated = if read_java_flag("DELTA_FLAG_ON") == Some(true) {
        match cplug_sdk::jni_util::with_attached(|env| {
            cplug_sdk::asm::insert_early_return_guard2(env, loader, &gated, &delta_spec())
        }) {
            Some(Some(b)) if !b.is_empty() => {
                DELTA_WOVEN.store(true, Ordering::Release);
                cplug_sdk::log::info("[c-entities] precomputed setDeltaMovement no-op gate");
                b
            }
            _ => {
                DELTA_DECLINED.store(true, Ordering::Release);
                cplug_sdk::log::info(
                    "[c-entities] setDeltaMovement gate refused — vanilla writes kept",
                );
                gated
            }
        }
    } else {
        DELTA_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] delta gate disabled (centities.deltaGate) — vanilla writes kept",
        );
        gated
    };
    // L22b: the inside-block-effect no-op gate chains onto the same Entity
    // bytes (op 4 — the no-arg applyEffectsFromBlocks() body; the receiver
    // load is the op-4 default). The gate must run before the probes: the
    // skipped work includes the applyEffectsFromBlocks → movement list
    // maintenance the probes would otherwise observe.
    let gated = if read_java_flag("BLOCK_FX_ON") == Some(true) {
        let blockfx = cplug_sdk::asm::EarlyReturnGuard {
            method_name: "applyEffectsFromBlocks",
            method_desc: "()V",
            check_class: HOOKS_CLASS,
            check_name: "mobBlockEffectsNoOp",
            check_desc: "(Lnet/minecraft/world/entity/Entity;)Z",
        };
        match cplug_sdk::jni_util::with_attached(|env| {
            cplug_sdk::asm::insert_early_return_guard(env, loader, &gated, &blockfx)
        }) {
            Some(Some(b)) if !b.is_empty() => {
                BLOCKFX_WOVEN.store(true, Ordering::Release);
                cplug_sdk::log::info("[c-entities] precomputed applyEffectsFromBlocks no-op gate");
                b
            }
            _ => {
                BLOCKFX_DECLINED.store(true, Ordering::Release);
                cplug_sdk::log::info(
                    "[c-entities] applyEffectsFromBlocks gate refused — vanilla block effects kept",
                );
                gated
            }
        }
    } else {
        BLOCKFX_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] block-fx gate disabled (CE_BLOCK_FX=0) — vanilla effects kept",
        );
        gated
    };
    // L26: the getSharedFlag direct-read floor chains onto the same Entity
    // bytes (after the block-fx gate — a different method, order is free).
    let gated = if read_java_flag("FLAGR_FLAG_ON") == Some(true)
        && prove_handles("initFlagRHandles", "FLAGR_HANDLES_OK") == Some(true)
    {
        match cplug_sdk::jni_util::with_attached(|env| {
            cplug_sdk::asm::replace_body(env, loader, &gated, &flagr_spec())
        }) {
            Some(Some(b)) if !b.is_empty() => {
                FLAGR_WOVEN.store(true, Ordering::Release);
                cplug_sdk::log::info("[c-entities] precomputed getSharedFlag direct floor");
                b
            }
            _ => {
                FLAGR_DECLINED.store(true, Ordering::Release);
                cplug_sdk::log::info(
                    "[c-entities] getSharedFlag replace refused — vanilla flag reads kept",
                );
                gated
            }
        }
    } else {
        FLAGR_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] flag-read floor disabled/refused (centities.flagRead) — vanilla kept",
        );
        gated
    };
    // the existing rust probe prefixes (box/impulse probes), on the gated bytes
    entity_probes(gated).ok()
}

fn precompute_entity(loader: jni::jobject) {
    let Some(bytes) = STASH.lock().ok().and_then(|s| s.get(ENTITY_CLASS).cloned()) else {
        // Entity loads at boot before the arm in every observed run; if it
        // ever loads post-arm the in-hook path applies the probes and the
        // verifier retries this precompute for the frozen gate.
        cplug_sdk::log::info(
            "[c-entities] Entity not stashed yet — frozen gate deferred to the verifier",
        );
        return;
    };
    let Some(gated) = apply_entity_frozen_and_probes(loader, &bytes) else {
        cplug_sdk::log::info(
            "[c-entities] Entity probe surgery refused in precompute — vanilla kept",
        );
        return;
    };
    if let Ok(mut pc) = PRECOMPUTED.lock() {
        pc.insert(ENTITY_CLASS, gated);
    }
    if !FROZEN_DECLINED.load(Ordering::Acquire) {
        FROZEN_WOVEN.store(true, Ordering::Release);
        cplug_sdk::log::info("[c-entities] precomputed Entity probes + setTicksFrozen gate");
    } else {
        cplug_sdk::log::info("[c-entities] precomputed Entity probes (frozen gate off)");
    }
}

fn precompute_conversion(loader: jni::jobject) {
    if read_java_flag("CONV_FLAG_ON") != Some(true) {
        CONV_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] conv gate disabled (centities.convGate) — vanilla setConverting kept",
        );
        return;
    }
    if prove_handles("initConvHandles", "CONV_HANDLES_OK") != Some(true) {
        CONV_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] conv gate handle probe failed — vanilla setConverting kept",
        );
        return;
    }
    let Some(bytes) = STASH
        .lock()
        .ok()
        .and_then(|mut s| s.remove(CONVERSION_TRACKER_CLASS))
    else {
        cplug_sdk::log::info(
            "[c-entities] ConversionTracker not loaded yet — conv gate deferred to the verifier",
        );
        return;
    };
    let patched = cplug_sdk::jni_util::with_attached(|env| {
        cplug_sdk::asm::replace_body(env, loader, &bytes, &conv_spec())
    });
    match patched {
        Some(Some(b)) if !b.is_empty() => {
            if let Ok(mut pc) = PRECOMPUTED.lock() {
                pc.insert(CONVERSION_TRACKER_CLASS, b);
            }
            CONV_WOVEN.store(true, Ordering::Release);
            cplug_sdk::log::info("[c-entities] precomputed ConversionTracker.setConverting gate");
        }
        _ => {
            cplug_sdk::log::info(
                "[c-entities] FAILED to precompute ConversionTracker gate — vanilla kept",
            );
        }
    }
}

/// L26: the Sensing.tick floor precompute. Same shape as the conv gate:
/// honor the kill switch, prove the handles, weave, stash the patched bytes
/// for the post-arm class load.
fn precompute_sensing(loader: jni::jobject) {
    if read_java_flag("SENSE_FLAG_ON") != Some(true) {
        SENSING_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] sensing floor disabled (centities.sensing) — vanilla tick kept",
        );
        return;
    }
    if prove_handles("initSenseHandles", "SENSE_HANDLES_OK") != Some(true) {
        SENSING_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] sensing floor handle probe failed — vanilla Sensing.tick kept",
        );
        return;
    }
    let Some(bytes) = STASH.lock().ok().and_then(|mut s| s.remove(SENSING_CLASS)) else {
        cplug_sdk::log::info(
            "[c-entities] Sensing not loaded yet — floor deferred to the verifier",
        );
        return;
    };
    let patched = cplug_sdk::jni_util::with_attached(|env| {
        cplug_sdk::asm::replace_body(env, loader, &bytes, &sensing_spec())
    });
    match patched {
        Some(Some(b)) if !b.is_empty() => {
            if let Ok(mut pc) = PRECOMPUTED.lock() {
                pc.insert(SENSING_CLASS, b);
            }
            SENSING_WOVEN.store(true, Ordering::Release);
            cplug_sdk::log::info("[c-entities] precomputed Sensing O(1)-clear floor");
            // NOTE: no retransform here — the precomputes run BEFORE
            // BRIDGE_READY, and the hook's reentrancy check would re-stash
            // and serve the ORIGINAL bytes (the L28a attempt logged rc=0 but
            // was a verified no-op: SenseChecks stayed 0). The boot-loaded
            // class is retransformed by the arm sequence BELOW (SENSING_CLASS
            // joined the list), where the hook serves the precomputed bytes;
            // a post-arm load is served directly by the hook.
        }
        _ => {
            cplug_sdk::log::info("[c-entities] FAILED to precompute Sensing floor — vanilla kept");
        }
    }
}

/// L15c: the TrackedEntity.moonrise$tick guard via the SDK's op-5
/// (receiver + first argument; COMPUTE_FRAMES rebuilds the frames — the
/// hand-rolled rust branch FAILED_VERIFICATION because a branch merge at
/// the original body start needs an explicit full frame). ChunkMap loads
/// at boot (pre-arm); if it ever loads post-arm the hook stashes and the
/// verifier retries.
fn precompute_trackedentity(loader: jni::jobject) {
    if read_java_flag("TT_FLAG_ON") != Some(true) {
        TT_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] tracker-tick gate disabled (centities.trackerTick) — vanilla kept",
        );
        return;
    }
    if prove_handles("initTTHandles", "TT_HANDLES_OK") != Some(true) {
        TT_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info("[c-entities] tracker-tick handle probe failed — vanilla kept");
        return;
    }
    let Some(bytes) = STASH
        .lock()
        .ok()
        .and_then(|s| s.get(TRACKED_ENTITY_CLASS).cloned())
    else {
        cplug_sdk::log::info(
            "[c-entities] TrackedEntity not stashed yet — gate deferred to the verifier",
        );
        return;
    };
    let spec = cplug_sdk::asm::EarlyReturnGuard2 {
        method_name: "moonrise$tick",
        method_desc: "(Lca/spottedleaf/moonrise/common/misc/NearbyPlayers$TrackedChunk;)V",
        check_class: HOOKS_CLASS,
        check_name: "trackerTickNoOp",
        check_desc: "(Ljava/lang/Object;Ljava/lang/Object;)Z",
    };
    let patched = cplug_sdk::jni_util::with_attached(|env| {
        cplug_sdk::asm::insert_early_return_guard2(env, loader, &bytes, &spec)
    })
    .flatten();
    match patched {
        Some(b) if !b.is_empty() => {
            if let Ok(mut pc) = PRECOMPUTED.lock() {
                pc.insert(TRACKED_ENTITY_CLASS, b);
            }
            TT_WOVEN.store(true, Ordering::Release);
            cplug_sdk::log::info("[c-entities] precomputed TrackedEntity.moonrise$tick no-op gate");
        }
        _ => {
            cplug_sdk::log::info(
                "[c-entities] FAILED to precompute TrackedEntity gate — vanilla kept",
            );
        }
    }
}

fn patch_trackedentity(bytes: &[u8]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| patch_trackedentity_inner(bytes))
        .ok()
        .flatten()
}

fn patch_trackedentity_inner(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        let mut st = STASH.lock().ok()?;
        st.entry(TRACKED_ENTITY_CLASS)
            .or_insert_with(|| bytes.to_vec());
        return None;
    }
    let pre = PRECOMPUTED.lock().ok()?.get(TRACKED_ENTITY_CLASS).cloned();
    match pre {
        Some(b) if !b.is_empty() => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] patched TrackedEntity tracker-tick gate");
            Some(b)
        }
        _ if !TT_WOVEN.load(Ordering::Acquire) && !TT_DECLINED.load(Ordering::Acquire) => {
            let mut st = STASH.lock().ok()?;
            st.entry(TRACKED_ENTITY_CLASS)
                .or_insert_with(|| bytes.to_vec());
            cplug_sdk::log::info(
                "[c-entities] TrackedEntity bytes stashed post-arm — verifier will decide",
            );
            None
        }
        _ => None,
    }
}

/// L30: the ChunkMap.newTrackerTick tracker-loop floor — the body becomes
/// `CeHooks.newTrackerTick$(Object)` (the byte-exact vanilla loop mirror +
/// the fused no-tracking fast path; the mirror is always the body, the fast
/// path arms after the ttlCalls liveness proof).
fn trackerloop_spec() -> cplug_sdk::asm::ReplaceBody<'static> {
    cplug_sdk::asm::ReplaceBody {
        method_name: "newTrackerTick",
        method_desc: "()V",
        bridge_owner: HOOKS_CLASS,
        bridge_name: "newTrackerTick$",
        bridge_desc: "(Ljava/lang/Object;)V",
        args: &[cplug_sdk::asm::ArgSpec::Local { slot: 0, ty: b'L' }],
    }
}

fn precompute_chunkmap(loader: jni::jobject) {
    if read_java_flag("TTL_FLAG_ON") != Some(true) {
        TTL_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] tracker-loop floor disabled (centities.trackerLoop) — vanilla kept",
        );
        return;
    }
    if prove_handles("initTrackerLoopHandles", "TTL_HANDLES_OK") != Some(true) {
        TTL_DECLINED.store(true, Ordering::Release);
        cplug_sdk::log::info(
            "[c-entities] tracker-loop handle probe failed — vanilla kept (lastErr in stats)",
        );
        return;
    }
    let Some(bytes) = STASH.lock().ok().and_then(|s| s.get(CHUNKMAP_CLASS).cloned())
    else {
        cplug_sdk::log::info(
            "[c-entities] ChunkMap not stashed yet — tracker-loop floor deferred to the verifier",
        );
        return;
    };
    let patched = cplug_sdk::jni_util::with_attached(|env| {
        cplug_sdk::asm::replace_body(env, loader, &bytes, &trackerloop_spec())
    });
    match patched {
        Some(Some(b)) if !b.is_empty() => {
            if let Ok(mut pc) = PRECOMPUTED.lock() {
                pc.insert(CHUNKMAP_CLASS, b);
            }
            TTL_WOVEN.store(true, Ordering::Release);
            cplug_sdk::log::info("[c-entities] precomputed ChunkMap.newTrackerTick loop floor");
        }
        _ => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(
                "[c-entities] FAILED to precompute ChunkMap tracker-loop floor — vanilla kept",
            );
        }
    }
}

fn patch_chunkmap(bytes: &[u8]) -> Option<Vec<u8>> {
    std::panic::catch_unwind(|| patch_chunkmap_inner(bytes))
        .ok()
        .flatten()
}

fn patch_chunkmap_inner(bytes: &[u8]) -> Option<Vec<u8>> {
    if !BRIDGE_READY.load(Ordering::Acquire) {
        // Boot load: stash the original bytes, stay vanilla. The floor is
        // woven at arm time (precompute_chunkmap).
        let mut st = STASH.lock().ok()?;
        st.entry(CHUNKMAP_CLASS)
            .or_insert_with(|| bytes.to_vec());
        return None;
    }
    let pre = PRECOMPUTED.lock().ok()?.get(CHUNKMAP_CLASS).cloned();
    match pre {
        Some(b) if !b.is_empty() => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] patched ChunkMap.newTrackerTick loop floor");
            Some(b)
        }
        _ if !TTL_WOVEN.load(Ordering::Acquire) && !TTL_DECLINED.load(Ordering::Acquire) => {
            let mut st = STASH.lock().ok()?;
            st.entry(CHUNKMAP_CLASS)
                .or_insert_with(|| bytes.to_vec());
            cplug_sdk::log::info(
                "[c-entities] ChunkMap bytes stashed post-arm — verifier will decide",
            );
            None
        }
        _ => None,
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
    GoalChecks,
    FrozenChecks,
    ConvChecks,
    TTChecks,
    TTLoopChecks,
    EffChecks,
    FlagChecks,
    DeltaChecks,
    SenseChecks,
    FlagRChecks,
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
            // L15 liveness counters: the GoalSelector bridge bumps gsMirrorTicks
            // while the floor is disarmed (proving the weave BEFORE arming);
            // frozenCalls/convCalls bump on every gated-body entry.
            (Counter::GoalChecks, ("gsMirrorTicks", "J")),
            (Counter::FrozenChecks, ("frozenCalls", "J")),
            (Counter::ConvChecks, ("convCalls", "J")),
            (Counter::TTChecks, ("ttCalls", "J")),
            (Counter::TTLoopChecks, ("ttlCalls", "J")),
            (Counter::EffChecks, ("effCalls", "J")),
            (Counter::FlagChecks, ("flagCalls", "J")),
            (Counter::DeltaChecks, ("deltaCalls", "J")),
            (Counter::SenseChecks, ("senseCalls", "J")),
            (Counter::FlagRChecks, ("flagrCalls", "J")),
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
        n_push
            as extern "system" fn(
                *mut jni::JNIEnv,
                jni::jclass,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
            ) -> i64
    );
    push!(
        "nativePushNcol",
        "(I)I",
        n_push_ncol as extern "system" fn(*mut jni::JNIEnv, jni::jclass, i32) -> i32
    );
    push!(
        "nativeMobAiFloor",
        "([I[I)I",
        n_mob_ai_floor
            as extern "system" fn(
                *mut jni::JNIEnv,
                jni::jclass,
                jni::jintArray,
                jni::jintArray,
            ) -> i32
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
    // stash the originals for the precompute (frozen gate + probes chained);
    // idempotent — JVMTI hands the hook the ORIGINAL bytes on every pass
    if let Ok(mut st) = STASH.lock() {
        st.entry(ENTITY_CLASS).or_insert_with(|| bytes.to_vec());
    }
    // Post-arm retransform pass: serve the precomputed rewrite when present.
    if let Some(pre) = PRECOMPUTED
        .lock()
        .ok()
        .and_then(|pc| pc.get(ENTITY_CLASS).cloned())
    {
        if !pre.is_empty() {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(
                "[c-entities] patched Entity (probes + setTicksFrozen gate, precomputed)",
            );
            return Some(pre);
        }
    }
    // First post-arm define (frozen gate still pending — the verifier will
    // complete it): apply the probes inline exactly like the old path.
    match entity_probes(bytes.to_vec()) {
        Ok(b) => {
            PATCHES_OK.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info("[c-entities] patched Entity box probe + wake-on-impulse");
            Some(b)
        }
        Err(e) => {
            PATCHES_FAIL.fetch_add(1, Ordering::AcqRel);
            cplug_sdk::log::info(&format!("[c-entities] Entity probe refused: {e}"));
            None
        }
    }
}

/// The Entity probe prefixes (box + impulse), factored out so the frozen-gate
/// precompute can chain them onto the replace_body output.
fn entity_probes(bytes: Vec<u8>) -> Result<Vec<u8>, String> {
    // setBoundingBox is the SINGLE box-change choke point in 1.21.8
    // (setPos(DDD) -> setPosRaw(DDD,Z) -> makeBoundingBox -> setBoundingBox,
    // javap-verified on the shipped jar). One probe covers every move with
    // the EXACT new box — no separate setPos probe, half the JNI traffic.
    let boxed = patch::insert_static_call_at_start(
        bytes,
        "setBoundingBox",
        "(Lnet/minecraft/world/phys/AABB;)V",
        HOOKS_CLASS,
        "probeBox",
        "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)V",
        &[(0, b'L'), (1, b'L')],
    )?;

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
    let d1 = patch::insert_static_call_at_start(
        boxed,
        "setDeltaMovement",
        "(Lnet/minecraft/world/phys/Vec3;)V",
        HOOKS_CLASS,
        "probeDelta",
        "(Lnet/minecraft/world/entity/Entity;)V",
        &[(0, b'L')],
    )?;
    patch::insert_static_call_at_start(
        d1,
        "setDeltaMovement",
        "(DDD)V",
        HOOKS_CLASS,
        "probeDelta",
        "(Lnet/minecraft/world/entity/Entity;)V",
        &[(0, b'L')],
    )
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
                st.push.set_delta(
                    id,
                    dbdv_buf[j * 3],
                    dbdv_buf[j * 3 + 1],
                    dbdv_buf[j * 3 + 2],
                );
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
            for (name, v) in [("PUSH_SDX", dx), ("PUSH_SDY", dy), ("PUSH_SDZ", dz)] {
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
pub extern "system" fn n_push_ncol(_env: *mut jni::JNIEnv, _cls: jni::jclass, id: i32) -> i32 {
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

// ===================================================================
// L22a — the native goal engine entry (centities-core::goals)
// ===================================================================

/// nativeMobAiFloor(int[] in, int[] out) — copy in, run the engine, copy out.
pub extern "system" fn n_mob_ai_floor(
    env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    in_arr: jni::jintArray,
    out_arr: jni::jintArray,
) -> i32 {
    unsafe {
        let e = JniEnv::from_raw(env);
        let raw = e.raw();
        let vt = *raw;
        let n_in = ((*vt).GetArrayLength)(raw, in_arr);
        let n_out = ((*vt).GetArrayLength)(raw, out_arr);
        if n_in < 1 || n_out < 1 {
            return centities_core::goals::ST_FALLBACK;
        }
        // thread-local scratch (server-thread confined; avoids a copy into a
        // fresh allocation per mob)
        thread_local! {
            static IN_BUF: std::cell::RefCell<Vec<i32>> =
                const { std::cell::RefCell::new(Vec::new()) };
            static OUT_BUF: std::cell::RefCell<Vec<i32>> =
                const { std::cell::RefCell::new(Vec::new()) };
        }
        IN_BUF.with(|c| {
            OUT_BUF.with(|oc| {
                let mut ib = c.borrow_mut();
                let mut ob = oc.borrow_mut();
                ib.resize(n_in as usize, 0);
                ob.resize(n_out as usize, 0);
                ((*vt).GetIntArrayRegion)(raw, in_arr, 0, n_in, ib.as_mut_ptr());
                if e.exception_check() {
                    e.exception_clear();
                    return centities_core::goals::ST_FALLBACK;
                }
                let st = centities_core::goals::ai_floor(&ib, &mut ob);
                ((*vt).SetIntArrayRegion)(raw, out_arr, 0, n_out, ob.as_ptr());
                if e.exception_check() {
                    e.exception_clear();
                    return centities_core::goals::ST_FALLBACK;
                }
                st
            })
        })
    }
}
