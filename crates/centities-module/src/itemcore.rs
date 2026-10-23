//! L6 — module-side item tick core: the per-world persistent store, the
//! flush native (one JNI crossing per world-tick) and the section-cache
//! JNI up-call into CeHooks.itemSectionData.
//!
//! Persistent per-item state is MINIMAL by design: the batch carries the
//! full per-tick state (the collector reads the entity fields), the store
//! keeps only what Java cannot cheaply provide every tick — the cobweb
//! stuck multiplier, the supporting-block triple and the friction memo —
//! tagged by id so id-slot reuse cannot cross-contaminate.

use centities_core::itemtick::Aabb6;
use centities_core::sections::{PaletteEntry, SectionCache, SectionData, SectionGrid, AIR_IDX};
use centities_core::tick::{
    tick_item, tick_item_inactive, ItemState, Scratch, TickEnv, TickOutcome, AB_COMPLEX, AB_FALL,
    AB_FLUID, AB_STUCK, AB_UNLOADED,
};
use jvmti_bindings::env::JniEnv;
use jvmti_bindings::sys::jni;
use std::cell::RefCell;

use crate::HOOKS_CLASS_GLOBAL;

const STORE_CAP: usize = 1 << 17;
const STORE_MASK: u64 = (STORE_CAP - 1) as u64;

#[derive(Clone, Copy)]
struct ItemEntry {
    tag: u64, // (id >> 17) | 1 — validity check against slot aliasing
    // L7 dispatch-owned state: for IF_NATIVE entries the STORE is the source
    // of truth (the dispatch guard suppresses the whole vanilla dispatcher,
    // so nothing else reads or writes these fields between flushes; the
    // applier mirrors every commit back into the entity). Probed external
    // mutations (setBoundingBox/setDeltaMovement prefixes) clear the item's
    // native mode, forcing a full vanilla tick + re-ingest.
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
    age: i32,
    pickup_delay: i32,
    tick_count: i32,
    despawn_rate: i32,
    fall_dist: f32,
    on_ground: bool,
    /// Consecutive native aborts without a commit. >= 4 forces VANILLA_NEXT
    /// so a permanently-imprecise item cannot freeze forever (vanilla runs,
    /// the resync re-ingests, the streak resets).
    aborts: u8,
    // L6 persistent parts (Java cannot cheaply provide these every tick)
    stuck: (f64, f64, f64),
    sup: (i32, i32, i32),
    sup_present: bool,
    sup_no_blocks: bool,
    fr_pos: (i32, i32, i32),
    friction: f32,
    speed: f32,
    fr_step_special: bool,
}

impl ItemEntry {
    fn fresh(tag: u64) -> ItemEntry {
        ItemEntry {
            tag,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            age: 0,
            pickup_delay: 0,
            tick_count: 0,
            // 6000 = vanilla default; a dispatch entry can only appear after
            // a resync flush ingested the real value, so this is a seatbelt.
            despawn_rate: 6000,
            fall_dist: 0.0,
            on_ground: false,
            aborts: 0,
            stuck: (0.0, 0.0, 0.0),
            sup: (0, 0, 0),
            sup_present: false,
            sup_no_blocks: false,
            fr_pos: (i32::MIN, i32::MIN, i32::MIN),
            friction: 0.6,
            speed: 1.0,
            fr_step_special: false,
        }
    }
}

/// Per-world: the item store + the section cache + the PERSISTENT flush
/// scratch (L8.2: the per-tick working arrays are reused instead of
/// reallocated — the previous fresh-Vec-per-flush scheme churned ~25 MB of
/// touched memory per tick at 100k items, page-faulting it in every time).
pub struct ItemWorld {
    entries: Vec<ItemEntry>,
    pub grid: SectionGrid,
    // ---- flush scratch (sized to n; only grows) ----
    /// Phase-1 hydrated states, mutated in place by phase 2.
    sts: Vec<ItemState>,
    tickable: Vec<bool>,
    aborts_in: Vec<u8>,
    /// Phase-2 commit records (SoA, L8.2): a slim flag byte + the L6
    /// persistent parts. The physics fields ride in od/oi as before and are
    /// NOT duplicated here any more (the old fat CommitRec was ~200B x n
    /// = ~12MB of per-tick allocation and double-writes).
    c_flag: Vec<u8>,
    c_l6: Vec<L6Rec>,
}

/// The L6 persistent parts of one committed tick (store-only fields the
/// entity mirror does not carry).
#[derive(Clone, Copy, Default)]
struct L6Rec {
    stuck: [f64; 3],
    sup: [i32; 3],
    fr_pos: [i32; 3],
    friction: f32,
    speed: f32,
    /// sup_present | sup_no_blocks<<1 | fr_step_special<<2
    flags: u8,
}

/// The full per-entry state snapshot used by the L7 dispatch protocol.
struct EntryState {
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
    age: i32,
    pickup_delay: i32,
    tick_count: i32,
    despawn_rate: i32,
    fall_dist: f32,
    on_ground: bool,
}

impl ItemWorld {
    fn new() -> ItemWorld {
        ItemWorld {
            entries: (0..STORE_CAP).map(|_| ItemEntry::fresh(u64::MAX)).collect(),
            grid: SectionGrid::new(),
            sts: Vec::new(),
            tickable: Vec::new(),
            aborts_in: Vec::new(),
            c_flag: Vec::new(),
            c_l6: Vec::new(),
        }
    }

    #[inline]
    fn entry(&mut self, id: i64) -> &mut ItemEntry {
        let slot = (id as u64 & STORE_MASK) as usize;
        let tag = (id as u64 >> 17) | 1;
        let e = &mut self.entries[slot];
        if e.tag != tag {
            *e = ItemEntry::fresh(tag);
        }
        e
    }

    pub fn invalidate_section(&mut self, cx: i32, cy: i32, cz: i32) {
        self.grid.invalidate(cx, cy, cz);
    }
}

/// Per-world item cores (wid 0..7). Worlds are LAZY: each ItemWorld costs
/// ~20 MB (131072 entries x full state), and a server typically ticks 1-3
/// worlds — preallocating all 8 burned ~160 MB of native memory for nothing
/// (the 100k-item bench legs died with runner shutdowns at exactly this
/// kind of resident overhead).
pub struct ItemCore {
    pub worlds: Vec<Option<ItemWorld>>,
}

impl ItemCore {
    pub fn new() -> ItemCore {
        ItemCore {
            worlds: (0..8).map(|_| None).collect(),
        }
    }

    #[inline]
    pub fn world(&mut self, wid: usize) -> &mut ItemWorld {
        self.worlds[wid].get_or_insert_with(ItemWorld::new)
    }
}

// ---------------------------------------------------------------------------
// section provider: JNI up-call into CeHooks.itemSectionData
// ---------------------------------------------------------------------------

thread_local! {
    static SEC_B: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
    static SEC_D: RefCell<Vec<f64>> = const { RefCell::new(Vec::new()) };
}

pub fn make_provider(wid: i32) -> impl FnMut(i32, i32, i32, bool) -> Result<SectionData, ()> {
    use std::sync::atomic::Ordering::Relaxed;
    move |cx: i32, cy: i32, cz: i32, load: bool| -> Result<SectionData, ()> {
        SEC_UPCALLS.fetch_add(1, Relaxed);
        SEC_MIN_X.fetch_min(cx, Relaxed);
        SEC_MAX_X.fetch_max(cx, Relaxed);
        SEC_MIN_Y.fetch_min(cy, Relaxed);
        SEC_MAX_Y.fetch_max(cy, Relaxed);
        SEC_MIN_Z.fetch_min(cz, Relaxed);
        SEC_MAX_Z.fetch_max(cz, Relaxed);
        let hooks_cls = HOOKS_CLASS_GLOBAL.load(std::sync::atomic::Ordering::Acquire);
        if hooks_cls == 0 {
            SEC_ERRS.fetch_add(1, Relaxed);
            return Err(());
        }
        cplug_sdk::jni_util::with_attached(|env| unsafe {
            item_section_upcall(env, hooks_cls, wid, cx, cy, cz, load)
        })
        .unwrap_or(Err(()))
    }
}

/// # Safety
/// `env` must be a live JNI env inside the with_attached scope.
unsafe fn item_section_upcall(
    env: &JniEnv,
    hooks_cls: usize,
    wid: i32,
    cx: i32,
    cy: i32,
    cz: i32,
    load: bool,
) -> Result<SectionData, ()> {
    let cls = hooks_cls as jni::jclass;
    let Some(mid) = env.get_static_method_id(cls, "itemSectionData", "(IIIIZ[D[I)Z") else {
        return Err(());
    };
    let raw = env.raw();
    let vt = *raw;
    SEC_D.with(|sd| {
        SEC_B.with(|sb| {
            let mut d = sd.borrow_mut();
            let mut b = sb.borrow_mut();
            d.clear();
            d.resize(24 * 256, 0.0);
            b.clear();
            b.resize(4096, AIR_IDX as i32);
            let arr_d = ((*vt).NewDoubleArray)(raw, (24 * 256) as i32);
            let arr_b = ((*vt).NewIntArray)(raw, 4096 as i32);
            if arr_d.is_null() || arr_b.is_null() {
                return Err(());
            }
            let args: [jni::jvalue; 7] = [
                jni::jvalue { i: wid },
                jni::jvalue { i: cx },
                jni::jvalue { i: cy },
                jni::jvalue { i: cz },
                jni::jvalue { z: load as u8 },
                jni::jvalue { l: arr_d },
                jni::jvalue { l: arr_b },
            ];
            let ok = ((*vt).CallStaticBooleanMethodA)(raw, cls, mid, args.as_ptr());
            if env.exception_check() {
                env.exception_clear();
                ((*vt).DeleteLocalRef)(raw, arr_d);
                ((*vt).DeleteLocalRef)(raw, arr_b);
                return Err(());
            }
            if ok == 0 {
                ((*vt).DeleteLocalRef)(raw, arr_d);
                ((*vt).DeleteLocalRef)(raw, arr_b);
                return Err(());
            }
            ((*vt).GetDoubleArrayRegion)(raw, arr_d, 0, (24 * 256) as i32, d.as_mut_ptr());
            ((*vt).GetIntArrayRegion)(raw, arr_b, 0, 4096, b.as_mut_ptr());
            ((*vt).DeleteLocalRef)(raw, arr_d);
            ((*vt).DeleteLocalRef)(raw, arr_b);
            Ok(parse_section(&d, &b))
        })
    })
}

fn parse_section(d: &[f64], b: &[i32]) -> SectionData {
    // entries: PAL_STRIDE=24 doubles each: [collider-count (0..3),
    //          cuboid0 x0..z1, cuboid1 x0..z1, cuboid2 x0..z1,
    //          friction, speed, water, fluid, flags]
    // flags bits: cobweb=1, insideSpecial=2, stepSpecial=4, fallSpecial=8,
    //             shapeOverride=16, complex=32
    const F_COBWEB: f64 = 1.0;
    const F_INSIDE: f64 = 2.0;
    const F_STEP: f64 = 4.0;
    const F_FALL: f64 = 8.0;
    const F_SHAPE: f64 = 16.0;
    const F_COMPLEX: f64 = 32.0;
    let mut entries: Vec<PaletteEntry> = Vec::with_capacity(16);
    let mut complex = false;
    let mut any_fluid = false;
    let mut any_inside = false;
    for slot in 0..d.len() / 24 {
        let o = slot * 24;
        let flags = d[o + 23] as i64;
        let shape_override = flags & F_SHAPE as i64 != 0;
        let entry_complex = flags & F_COMPLEX as i64 != 0;
        let count = d[o].max(0.0).min(3.0) as usize;
        let mut colliders = Vec::with_capacity(count);
        if !shape_override && !entry_complex {
            for c in 0..count {
                let co = o + 1 + c * 6;
                colliders.push(Aabb6::new(
                    d[co],
                    d[co + 1],
                    d[co + 2],
                    d[co + 3],
                    d[co + 4],
                    d[co + 5],
                ));
            }
        }
        let entry = PaletteEntry {
            colliders,
            friction: d[o + 19] as f32,
            speed: d[o + 20] as f32,
            is_water: d[o + 21] != 0.0,
            has_fluid: d[o + 22] != 0.0,
            cobweb: flags & F_COBWEB as i64 != 0,
            entity_inside_special: flags & F_INSIDE as i64 != 0,
            step_on_special: flags & F_STEP as i64 != 0,
            fall_on_special: flags & F_FALL as i64 != 0,
        };
        // ABORT-STORM ROOT CAUSE (run 39927156887, b66f61b diag): cpx ~= 25k
        // aborts/flush with stuck/unl/fld/fal = 0. The Java provider (L8.2)
        // deliberately writes ZERO-cuboid shape-override entries (plants,
        // torches: getShape overridden, getCollisionShape empty everywhere)
        // with complex=0 — an empty collision shape is natively exact as 0
        // colliders for every vanilla block (context-sensitive collision
        // with boxes — fences, walls, chests, snow layers — carries
        // complex=32 from the Java side). This parser however re-poisoned
        // the whole section from the bare shape_override bit, undoing the
        // Java refinement: one tall-grass tuft in a 16^3 section aborted
        // every item sharing that section (~50% of the bench floor).
        // FIX: the section-complex decision trusts ONLY the complex bit.
        if entry_complex {
            complex = true;
        }
        if entry.has_fluid {
            any_fluid = true;
        }
        if entry.entity_inside_special {
            any_inside = true;
        }
        entries.push(entry);
        if entries.len() >= 256 {
            break;
        }
    }
    // Memory diet (the sparse-fill explosion): an all-air section stores NO
    // blocks array (empty Vec == air in the accessors), and the palette is
    // shrunk to its exact size — a 2400x2400-block fill touches ~500k
    // sections and the previous 8KB-per-section blocks array alone was 4GB
    // of mostly-air cache.
    let all_air = b[..4096].iter().all(|&v| v == 0);
    let blocks = if all_air {
        Vec::new()
    } else {
        let mut blocks = vec![AIR_IDX; 4096];
        for (i, v) in b.iter().take(4096).enumerate() {
            blocks[i] = *v as u16;
        }
        blocks
    };
    let mut entries = entries;
    entries.shrink_to_fit();
    SectionData {
        entries,
        blocks,
        complex,
        any_fluid,
        any_inside_special: any_inside,
    }
}

// ---------------------------------------------------------------------------
// the flush native
// ---------------------------------------------------------------------------

// out bits (mirror of TickOutcome + the mode protocol)
// The bit positions are the JAVA APPLIER'S CONTRACT (CeHooks.itemTickFlush):
// MOVED 1, ON_GROUND 2, IMPULSE 4, MERGE 32, DESPAWN 128, VOID 256,
// LAND 512 (1<<9), VANILLA_NEXT 2048 (1<<11), NATIVE_OK 4096 (1<<12),
// ABORTED 8192 (1<<13). (The pre-L8 constants had LAND/VANILLA_NEXT/ABORTED
// shifted one row: the applier then read aborted items as normal ones and
// applied zeros — the frozen-world CI failure.)
const B_MOVED: i32 = 1;
const B_ON_GROUND: i32 = 1 << 1;
const B_IMPULSE: i32 = 1 << 2;
const B_MERGE: i32 = 1 << 5;
const B_DESPAWN: i32 = 1 << 7;
const B_VOID: i32 = 1 << 8;
const B_LAND: i32 = 1 << 9;
const B_VANILLA_NEXT: i32 = 1 << 11;
const B_NATIVE_OK: i32 = 1 << 12;
const B_ABORTED: i32 = 1 << 13;

// flags from Java (mirror of CeHooks.IF_*)
const F_INACTIVE: i32 = 1;
#[allow(dead_code)]
const F_NATIVE: i32 = 2;
const F_RESYNC: i32 = 4;
const F_FRIC_OFF: i32 = 8;
const F_FORCE_VANILLA: i32 = 16;
const F_ON_GROUND: i32 = 32;

static FLUSH_NANOS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static FLUSH_COUNT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// Diagnostics: how many section up-calls the provider served and how many
/// SectionData slots the grid retains (the CI 100k-mod legs exploded RSS at
/// ~300MB/s with NMT flat — this line names the growing structure).
static SEC_UPCALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
/// Provider coordinate extremes: if the gather feeds garbage (NaN/huge
/// deltas), these explode — the 100k fill showed 457k retained sections for
/// ~12k items, which is impossible for sane coords.
static SEC_MIN_X: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(i32::MAX);
static SEC_MAX_X: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(i32::MIN);
static SEC_MIN_Y: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(i32::MAX);
static SEC_MAX_Y: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(i32::MIN);
static SEC_MIN_Z: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(i32::MAX);
static SEC_MAX_Z: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(i32::MIN);
static SEC_ERRS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn cst_ab(a: &std::sync::atomic::AtomicU64) -> u64 {
    a.load(std::sync::atomic::Ordering::Relaxed)
}

fn proc_rss_mb() -> u64 {
    // /proc/self/statm: [total, resident, ...] in 4KB pages
    let Ok(s) = std::fs::read_to_string("/proc/self/statm") else {
        return 0;
    };
    s.split_whitespace()
        .nth(1)
        .and_then(|v| v.parse::<u64>().ok())
        .map(|pages| pages * 4096 / 1024 / 1024)
        .unwrap_or(0)
}

thread_local! {
    static T_IDS: RefCell<Vec<i64>> = const { RefCell::new(Vec::new()) };
    static T_FLAGS: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
    static T_IND: RefCell<Vec<f64>> = const { RefCell::new(Vec::new()) };
    static T_INI: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
    static T_OUTD: RefCell<Vec<f64>> = const { RefCell::new(Vec::new()) };
    static T_OUTI: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
}

/// JNI native: env is live on the calling (server) thread.
#[allow(clippy::too_many_arguments)]
pub extern "system" fn n_item_tick(
    env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    wid: i32,
    n: i32,
    ids: jni::jlongArray,
    flags: jni::jintArray,
    in_d: jni::jdoubleArray,
    cdn: i32,
    in_i: jni::jintArray,
    env_d: jni::jdoubleArray,
    env_i: jni::jintArray,
    out_d: jni::jdoubleArray,
    out_i: jni::jintArray,
) -> i32 {
    // SAFETY: the JVM passes a live env; every raw JNI call below mirrors the
    // vtable pattern proven in lib.rs.
    return unsafe {
        n_item_tick_impl(
            env, _cls, wid, n, ids, flags, in_d, cdn, in_i, env_d, env_i, out_d, out_i,
        )
    };
}

#[allow(clippy::too_many_arguments)]
unsafe fn n_item_tick_impl(
    env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    wid: i32,
    n: i32,
    ids: jni::jlongArray,
    flags: jni::jintArray,
    in_d: jni::jdoubleArray,
    cdn: i32,
    in_i: jni::jintArray,
    env_d: jni::jdoubleArray,
    env_i: jni::jintArray,
    out_d: jni::jdoubleArray,
    out_i: jni::jintArray,
) -> i32 {
    let n = n.max(0) as usize;
    if n == 0 || wid < 0 || wid >= 8 {
        return -1;
    }
    let Ok(mut g) = crate::INDEX.try_lock() else {
        return -1;
    };
    let cst = g.get_or_insert_with(crate::new_state);
    if cst.items.is_none() {
        cst.items = Some(ItemCore::new());
    }
    let core = cst.items.as_mut().unwrap();
    let e = JniEnv::from_raw(env);
    let raw = e.raw();
    let vt = *raw;

    T_IND.with(|td| {
        T_INI.with(|ti| {
            T_IDS.with(|tid| {
                T_FLAGS.with(|tf| {
                    T_OUTD.with(|tod| {
                        T_OUTI.with(|toi| {
                            let mut d = td.borrow_mut();
                            let mut iv = ti.borrow_mut();
                            let mut idv = tid.borrow_mut();
                            let mut fv = tf.borrow_mut();
                            let mut od = tod.borrow_mut();
                            let mut oi = toi.borrow_mut();
                            iv.resize(n * 4, 0);
                            idv.resize(n, 0);
                            fv.resize(n, 0);
                            od.resize(n * 8, 0.0);
                            // L7 outI layout: [age, pickupDelay, bits, tickCount] —
                            // the applier mirrors the dispatcher's tickCount++/totalEntityAge++
                            // for entries whose vanilla dispatcher was suppressed.
                            oi.resize(n * 4, 0);
                            ((*vt).GetLongArrayRegion)(raw, ids, 0, n as i32, idv.as_mut_ptr());
                            ((*vt).GetIntArrayRegion)(raw, flags, 0, n as i32, fv.as_mut_ptr());
                            ((*vt).GetIntArrayRegion)(raw, in_i, 0, (n * 4) as i32, iv.as_mut_ptr());
                            let mut envv_d = [0.0f64; 6];
                            let mut envv_i = [0i32; 4];
                            ((*vt).GetDoubleArrayRegion)(raw, env_d, 0, 6, envv_d.as_mut_ptr());
                            ((*vt).GetIntArrayRegion)(raw, env_i, 0, 4, envv_i.as_mut_ptr());
                            if e.exception_check() {
                                e.exception_clear();
                                return -1;
                            }
                            // L8.2 COMPACT BATCH: dispatch-owned entries (the
                            // dominant population) carry ONLY id+flags — their
                            // state lives in the store. Collector-data entries
                            // (resync / force-vanilla) side-pack their 14-double
                            // stride into a contiguous segment; the 1-based slot
                            // number rides in flags bits 6+. The JNI copy
                            // shrinks from n*14 doubles (~7MB/tick of dead zeros
                            // at 60k items) to cd_n*14. cdn (the segment count)
                            // comes from Java and is authoritative.
                            let cd_n = cdn.max(0) as usize;
                            d.resize(cd_n * 14, 0.0);
                            if cd_n > 0 {
                                ((*vt).GetDoubleArrayRegion)(
                                    raw,
                                    in_d,
                                    0,
                                    (cd_n * 14) as i32,
                                    d.as_mut_ptr(),
                                );
                                if e.exception_check() {
                                    e.exception_clear();
                                    return -1;
                                }
                            }
                            let tenv = TickEnv {
                                min_section_y: envv_i[0],
                                max_section_y: envv_i[1],
                                void_damage: envv_i[2] != 0,
                                void_threshold: envv_d[0],
                                border_min_x: envv_d[1],
                                border_max_x: envv_d[2],
                                border_min_z: envv_d[3],
                                border_max_z: envv_d[4],
                                border_margin: envv_d[5],
                            };
                            let items_native = envv_i[3] != 0;
                            let t0 = std::time::Instant::now();
                            let world = core.world(wid as usize);

                            // ------------------------------------------------
                            // L8 PARALLEL FLUSH — three phases.
                            //
                            // Phase 1 (sequential): parse the batch + hydrate
                            // the persistent parts from the store. The store
                            // is touched ONLY here and in phase 3.
                            // Phase 2 (rayon-parallel): the tick itself —
                            // every item is independent; each chunk gets a
                            // worker-local SectionCache (no shared locks on
                            // the hot path) and per-chunk scratch/provider.
                            // All outputs are written by disjoint per-chunk
                            // slots, so the result is bit-identical to the
                            // sequential loop regardless of scheduling.
                            // Phase 3 (sequential): commit the tick results
                            // into the store in batch order (== vanilla tick
                            // order) + the diagnostic fold.
                            // ------------------------------------------------

                            let mut max_abs_delta: f64 = 0.0f64;
                            let mut max_abs_pos: f64 = 0.0f64;
                            // per-flush outcome counters (flush-log visibility:
                            // ok = committed native ticks, ab = aborted)
                            let mut flush_ok: u64 = 0;
                            let mut flush_aborts: u64 = 0;

                            // ---- phase 1: parse + hydrate (sequential) ----
                            // L8.2: the working arrays live in the world and are
                            // REUSED (resize-only) — a fresh alloc per flush
                            // churned ~25MB/tick of page-faulting memory.
                            if world.sts.len() != n {
                                world.sts.resize(n, ItemState::new());
                                world.tickable.clear();
                                world.tickable.resize(n, false);
                                world.aborts_in.clear();
                                world.aborts_in.resize(n, 0);
                                world.c_flag.clear();
                                world.c_flag.resize(n, 0);
                                world.c_l6.clear();
                                world.c_l6.resize(n, L6Rec::default());
                            } else {
                                world.tickable.fill(false);
                                world.aborts_in.fill(0);
                                world.c_flag.fill(0);
                                world.c_l6.fill(L6Rec::default());
                            }
                            for k in 0..n {
                                let id = idv[k];
                                let f_raw = fv[k];
                                let f = f_raw & 0x3F;
                                // 1-based collector-data slot in flags bits 6+;
                                // 0 = dispatch-owned (no per-tick fields). The
                                // clamp is a belt against a malformed batch.
                                let slot_raw = (f_raw >> 6) as usize;
                                let cd_slot = if slot_raw > 0 { slot_raw.min(cd_n) } else { 0 };
                                let has_cd = cd_slot > 0;
                                let dbo = if has_cd { (cd_slot - 1) * 14 } else { 0 };
                                let mut st = ItemState {
                                    x: if has_cd { d[dbo] } else { 0.0 },
                                    y: if has_cd { d[dbo + 1] } else { 0.0 },
                                    z: if has_cd { d[dbo + 2] } else { 0.0 },
                                    dx: if has_cd { d[dbo + 3] } else { 0.0 },
                                    dy: if has_cd { d[dbo + 4] } else { 0.0 },
                                    dz: if has_cd { d[dbo + 5] } else { 0.0 },
                                    xo: if has_cd { d[dbo] } else { 0.0 },
                                    yo: if has_cd { d[dbo + 1] } else { 0.0 },
                                    zo: if has_cd { d[dbo + 2] } else { 0.0 },
                                    on_ground: (f & F_ON_GROUND) != 0,
                                    fall_dist: if has_cd { d[dbo + 6] as f32 } else { 0.0 },
                                    // dispatcher-incremented; the core does NOT ++
                                    tick_count: iv[k * 4],
                                    age: iv[k * 4 + 1],
                                    pickup_delay: iv[k * 4 + 2],
                                    despawn_rate: iv[k * 4 + 3],
                                    has_impulse: false,
                                    stuck: (0.0, 0.0, 0.0),
                                    sup_x: 0,
                                    sup_y: 0,
                                    sup_z: 0,
                                    sup_present: false,
                                    on_ground_no_blocks: false,
                                    fr_pos: (i32::MIN, i32::MIN, i32::MIN),
                                    friction: 0.6,
                                    speed: 1.0,
                                    fr_step_special: false,
                                    friction_disabled: (f & F_FRIC_OFF) != 0,
                                    land_fall: 0.0,
                                };
                                if (f & (F_RESYNC | F_FORCE_VANILLA)) != 0 {
                                    // vanilla body ran this tick: ingest the fresh
                                    // state (incl. stuck/sup) and re-eval the mode.
                                    // L7: FULL state ingest — the entry becomes the
                                    // dispatch-owned source of truth once the item
                                    // re-arms, so everything must be current here.
                                    let entry = world.entry(id);
                                    if has_cd {
                                        entry.stuck = (d[dbo + 7], d[dbo + 8], d[dbo + 9]);
                                        entry.sup = (
                                            d[dbo + 10] as i32,
                                            d[dbo + 11] as i32,
                                            d[dbo + 12] as i32,
                                        );
                                        entry.sup_present = d[dbo + 13] != 0.0;
                                    }
                                    entry.x = st.x;
                                    entry.y = st.y;
                                    entry.z = st.z;
                                    entry.dx = st.dx;
                                    entry.dy = st.dy;
                                    entry.dz = st.dz;
                                    entry.age = iv[k * 4 + 1];
                                    entry.pickup_delay = iv[k * 4 + 2];
                                    entry.tick_count = iv[k * 4];
                                    entry.despawn_rate = iv[k * 4 + 3];
                                    entry.fall_dist = st.fall_dist;
                                    entry.on_ground = (f & F_ON_GROUND) != 0;
                                    entry.aborts = 0;
                                    st.stuck = entry.stuck;
                                    st.sup_x = entry.sup.0;
                                    st.sup_y = entry.sup.1;
                                    st.sup_z = entry.sup.2;
                                    st.sup_present = entry.sup_present;
                                    let mut bits: i32 = 0;
                                    if items_native
                                        && centities_core::tick::want_native(&st, &tenv)
                                    {
                                        bits |= B_NATIVE_OK;
                                    } else {
                                        bits |= B_VANILLA_NEXT;
                                    }
                                    od[k * 8] = st.dx;
                                    od[k * 8 + 1] = st.dy;
                                    od[k * 8 + 2] = st.dz;
                                    od[k * 8 + 3] = st.x;
                                    od[k * 8 + 4] = st.y;
                                    od[k * 8 + 5] = st.z;
                                    od[k * 8 + 6] = st.fall_dist as f64;
                                    oi[k * 4] = st.age;
                                    oi[k * 4 + 1] = st.pickup_delay;
                                    oi[k * 4 + 2] = bits;
                                    oi[k * 4 + 3] = st.tick_count;
                                    world.sts[k] = st; // tickable[k] stays false
                                    continue;
                                }
                                // hydrate the persistent parts
                                {
                                    let entry = world.entry(id);
                                    st.stuck = entry.stuck;
                                    st.sup_x = entry.sup.0;
                                    st.sup_y = entry.sup.1;
                                    st.sup_z = entry.sup.2;
                                    st.sup_present = entry.sup_present;
                                    st.on_ground_no_blocks = entry.sup_no_blocks;
                                    if entry.fr_pos == fr_key_of(&st) {
                                        st.fr_pos = entry.fr_pos;
                                        st.friction = entry.friction;
                                        st.speed = entry.speed;
                                        st.fr_step_special = entry.fr_step_special;
                                    }
                                }
                                // L7 dispatch-owned entries: the STORE is the state
                                // source (the vanilla dispatcher was suppressed, the
                                // batch carries no fresh fields except the guard's
                                // pickupDelay read for plugin parity).
                                if (f & F_NATIVE) != 0 {
                                    let es = {
                                        let e = world.entry(id);
                                        EntryState {
                                            x: e.x,
                                            y: e.y,
                                            z: e.z,
                                            dx: e.dx,
                                            dy: e.dy,
                                            dz: e.dz,
                                            age: e.age,
                                            pickup_delay: e.pickup_delay,
                                            tick_count: e.tick_count,
                                            despawn_rate: e.despawn_rate,
                                            fall_dist: e.fall_dist,
                                            on_ground: e.on_ground,
                                        }
                                    };
                                    st.x = es.x;
                                    st.y = es.y;
                                    st.z = es.z;
                                    st.dx = es.dx;
                                    st.dy = es.dy;
                                    st.dz = es.dz;
                                    st.xo = es.x;
                                    st.yo = es.y;
                                    st.zo = es.z;
                                    st.age = es.age;
                                    st.despawn_rate = es.despawn_rate;
                                    st.fall_dist = es.fall_dist;
                                    st.on_ground = es.on_ground;
                                    st.on_ground_no_blocks = world.entry(id).sup_no_blocks;
                                    // the suppressed tickNonPassenger would have ++ed
                                    st.tick_count = es.tick_count.wrapping_add(1);
                                    // plugin-parity: the guard re-read pickupDelay
                                    // every tick; a mid-flight change wins.
                                    let fresh_pd = iv[k * 4 + 2];
                                    if fresh_pd != es.pickup_delay {
                                        st.pickup_delay = fresh_pd;
                                    } else {
                                        st.pickup_delay = es.pickup_delay;
                                    }
                                }
                                world.aborts_in[k] = world.entry(id).aborts;
                                world.tickable[k] = true;
                                world.sts[k] = st;
                            }

                            // ---- phase 2: the tick (rayon-parallel) ----
                            {
                                // Split the world borrow: the grid is read-only for
                                // the tick, the SoA scratch arrays are chunk-
                                // disjoint (each parallel chunk touches only its
                                // own 512 slots).
                                let ItemWorld { grid, sts, tickable, aborts_in, c_flag, c_l6, .. } =
                                    world;
                                let grid = &*grid;
                                use rayon::prelude::*;
                                sts.par_chunks_mut(512)
                                    .zip(od.par_chunks_mut(512 * 8))
                                    .zip(oi.par_chunks_mut(512 * 4))
                                    .zip(c_flag.par_chunks_mut(512))
                                    .zip(c_l6.par_chunks_mut(512))
                                    .zip(tickable.par_chunks(512))
                                    .zip(aborts_in.par_chunks(512))
                                    .zip(idv.par_chunks(512))
                                    .zip(fv.par_chunks(512))
                                    .for_each(
                                        |((((((((st_c, od_c), oi_c), cf_c), cl_c), tk_c), ab_c), id_c), f_c)| {
                                            let mut scratch = Scratch::new();
                                            let mut provider = make_provider(wid);
                                            let mut cache = SectionCache::new(grid, &mut provider);
                                            for j in 0..st_c.len() {
                                                if !tk_c[j] {
                                                    continue;
                                                }
                                                let id = id_c[j];
                                                let f = f_c[j] & 0x3F;
                                                let st = &mut st_c[j];
                                                let outcome = if (f & F_INACTIVE) != 0 {
                                                    tick_item_inactive(st)
                                                } else {
                                                    tick_item(st, id, &tenv, &mut cache, &mut scratch)
                                                };
                                                if outcome.has(TickOutcome::ABORTED) {
                                                    // no commit, no writeback: the item
                                                    // froze this tick; vanilla computes it
                                                    // next tick. L7 seatbelt: a run of 4
                                                    // aborts forces a vanilla tick (the
                                                    // streak is carried from phase 1 and
                                                    // updated in phase 3).
                                                    let want = items_native
                                                        && ab_c[j] < 4
                                                        && centities_core::tick::want_native(st, &tenv);
                                                    let mode =
                                                        if want { B_NATIVE_OK } else { B_VANILLA_NEXT };
                                                    oi_c[j * 4 + 2] = B_ABORTED | mode;
                                                    // c_flag: 2 = abort (streak+1),
                                                    // 3 = abort + clear the streak (the
                                                    // vanilla tick re-ingests)
                                                    cf_c[j] = if want { 2 } else { 3 };
                                                    continue;
                                                }
                                                od_c[j * 8] = st.dx;
                                                od_c[j * 8 + 1] = st.dy;
                                                od_c[j * 8 + 2] = st.dz;
                                                od_c[j * 8 + 3] = st.x;
                                                od_c[j * 8 + 4] = st.y;
                                                od_c[j * 8 + 5] = st.z;
                                                od_c[j * 8 + 6] = st.fall_dist as f64;
                                                od_c[j * 8 + 7] = st.land_fall as f64;
                                                oi_c[j * 4] = st.age;
                                                oi_c[j * 4 + 1] = st.pickup_delay;
                                                let mut ob = 0i32;
                                                if outcome.has(TickOutcome::MOVED) { ob |= B_MOVED; }
                                                if outcome.has(TickOutcome::ON_GROUND) { ob |= B_ON_GROUND; }
                                                if outcome.has(TickOutcome::HAS_IMPULSE) { ob |= B_IMPULSE; }
                                                if outcome.has(TickOutcome::MERGE) { ob |= B_MERGE; }
                                                if outcome.has(TickOutcome::DESPAWN) { ob |= B_DESPAWN; }
                                                if outcome.has(TickOutcome::VOID) { ob |= B_VOID; }
                                                if outcome.has(TickOutcome::LAND) { ob |= B_LAND; }
                                                if outcome.has(TickOutcome::VANILLA_NEXT)
                                                    || !items_native
                                                    // L7 cadence belt: every 256th tick the
                                                    // item runs one full vanilla tick +
                                                    // re-ingest (heals any drift window).
                                                    || (st.tick_count & 0xFF) == 0
                                                {
                                                    ob |= B_VANILLA_NEXT;
                                                } else {
                                                    ob |= B_NATIVE_OK;
                                                }
                                                oi_c[j * 4 + 2] = ob;
                                                oi_c[j * 4 + 3] = st.tick_count;
                                                // L6 persistent parts only — the physics
                                                // fields ride in od/oi (phase 3 reads them
                                                // back); the fat per-item CommitRec is gone.
                                                cf_c[j] = 1;
                                                cl_c[j] = L6Rec {
                                                    stuck: [st.stuck.0, st.stuck.1, st.stuck.2],
                                                    sup: [st.sup_x, st.sup_y, st.sup_z],
                                                    fr_pos: [st.fr_pos.0, st.fr_pos.1, st.fr_pos.2],
                                                    friction: st.friction,
                                                    speed: st.speed,
                                                    flags: (st.sup_present as u8)
                                                        | ((st.on_ground_no_blocks as u8) << 1)
                                                        | ((st.fr_step_special as u8) << 2),
                                                };
                                            }
                                            // one shared-counter bump per chunk (not per
                                            // visit) — the old per-visit atomic was a
                                            // contended cache line across all workers
                                            grid.lookups
                                                .fetch_add(cache.visits, std::sync::atomic::Ordering::Relaxed);
                                        },
                                    );
                            }

                            // ---- phase 3: commit + diagnostics (sequential) ----
                            // c_flag: 0 = nothing (resync/inactive-collected),
                            // 1 = ok commit, 2 = abort, 3 = abort + clear streak.
                            // Physics fields are read back from od/oi (written by
                            // phase 2) instead of a fat per-item record.
                            for k in 0..n {
                                let flg = world.c_flag[k];
                                if flg == 0 {
                                    continue;
                                }
                                let id = idv[k];
                                let f = fv[k] & 0x3F;
                                if flg >= 2 {
                                    {
                                        let streak = world.aborts_in[k];
                                        let entry = world.entry(id);
                                        entry.aborts = streak.saturating_add(1);
                                        if flg == 3 {
                                            // L7 seatbelt: the vanilla tick will
                                            // re-ingest; clear the streak with it.
                                            entry.aborts = 0;
                                        }
                                    }
                                    flush_aborts += 1;
                                    continue;
                                }
                                flush_ok += 1;
                                let rec = world.c_l6[k];
                                {
                                    let entry = world.entry(id);
                                    entry.stuck = (rec.stuck[0], rec.stuck[1], rec.stuck[2]);
                                    entry.sup = (rec.sup[0], rec.sup[1], rec.sup[2]);
                                    entry.sup_present = rec.flags & 1 != 0;
                                    entry.sup_no_blocks = rec.flags & 2 != 0;
                                    entry.fr_pos = (rec.fr_pos[0], rec.fr_pos[1], rec.fr_pos[2]);
                                    entry.friction = rec.friction;
                                    entry.speed = rec.speed;
                                    entry.fr_step_special = rec.flags & 4 != 0;
                                    // L7: mirror the full state back (dispatch-owned)
                                    entry.x = od[k * 8 + 3];
                                    entry.y = od[k * 8 + 4];
                                    entry.z = od[k * 8 + 5];
                                    entry.dx = od[k * 8];
                                    entry.dy = od[k * 8 + 1];
                                    entry.dz = od[k * 8 + 2];
                                    entry.age = oi[k * 4];
                                    entry.pickup_delay = oi[k * 4 + 1];
                                    entry.tick_count = oi[k * 4 + 3];
                                    entry.fall_dist = od[k * 8 + 6] as f32;
                                    entry.on_ground = (oi[k * 4 + 2] & B_ON_GROUND) != 0;
                                    entry.aborts = 0;
                                    if (f & F_NATIVE) == 0 {
                                        // collector-carried entries refresh despawnRate
                                        entry.despawn_rate = iv[k * 4 + 3];
                                    }
                                }
                                // state extremes (garbage-detector diagnostics)
                                let ad = od[k * 8]
                                    .abs()
                                    .max(od[k * 8 + 1].abs())
                                    .max(od[k * 8 + 2].abs());
                                if ad.is_finite() && ad > max_abs_delta {
                                    max_abs_delta = ad;
                                }
                                let ap = od[k * 8 + 3]
                                    .abs()
                                    .max(od[k * 8 + 4].abs())
                                    .max(od[k * 8 + 5].abs());
                                if ap.is_finite() && ap > max_abs_pos {
                                    max_abs_pos = ap;
                                }
                            }

                            ((*vt).SetDoubleArrayRegion)(raw, out_d, 0, (n * 8) as i32, od.as_ptr());
                            ((*vt).SetIntArrayRegion)(raw, out_i, 0, (n * 4) as i32, oi.as_ptr());
                            {
                                let dt = t0.elapsed().as_nanos() as u64;
                                FLUSH_NANOS.fetch_add(dt, std::sync::atomic::Ordering::Relaxed);
                                let cnt = FLUSH_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                                if cnt % 50 == 0 {
                                    let total = FLUSH_NANOS.load(std::sync::atomic::Ordering::Relaxed);
                                    let g = &world.grid;
                                    let idx_stats = cst.idx.stats();
                                    cplug_sdk::log::info(&format!(
                                        "[c-entities] item-flush: {} flushes, avg {:.1}us, last n={} cd={} rss={}MB, upcalls={} errs={} grid pop={} look={} idx ents={} cells={} oversize={} secX[{}..{}] secY[{}..{}] secZ[{}..{}] maxD{} maxP{} ok{} ab{} stuck{} unl{} fld{} cpx{} fal{})",
                                        cnt,
                                        total as f64 / cnt as f64 / 1000.0,
                                        n,
                                        cd_n,
                                        proc_rss_mb(),
                                        SEC_UPCALLS.load(std::sync::atomic::Ordering::Relaxed),
                                        SEC_ERRS.load(std::sync::atomic::Ordering::Relaxed),
                                        g.populated.load(std::sync::atomic::Ordering::Relaxed),
                                        g.lookups.load(std::sync::atomic::Ordering::Relaxed),
                                        idx_stats.entities,
                                        idx_stats.cells,
                                        idx_stats.oversize,
                                        SEC_MIN_X.load(std::sync::atomic::Ordering::Relaxed),
                                        SEC_MAX_X.load(std::sync::atomic::Ordering::Relaxed),
                                        SEC_MIN_Y.load(std::sync::atomic::Ordering::Relaxed),
                                        SEC_MAX_Y.load(std::sync::atomic::Ordering::Relaxed),
                                        SEC_MIN_Z.load(std::sync::atomic::Ordering::Relaxed),
                                        SEC_MAX_Z.load(std::sync::atomic::Ordering::Relaxed),
                                        format_args!("{:e}", max_abs_delta),
                                        format_args!("{:e}", max_abs_pos),
                                        flush_ok,
                                        flush_aborts,
                                        cst_ab(&AB_STUCK),
                                        cst_ab(&AB_UNLOADED),
                                        cst_ab(&AB_FLUID),
                                        cst_ab(&AB_COMPLEX),
                                        cst_ab(&AB_FALL),
                                    ));
                                }
                            }
                            0
                        })
                    })
                })
            })
        })
    })
}

/// The friction-block position the memo is keyed by (mirrors the core's
/// get_on_pos_friction — only used to decide memo reuse on hydrate).
#[inline]
fn fr_key_of(st: &ItemState) -> (i32, i32, i32) {
    if st.sup_present {
        (
            st.sup_x,
            centities_core::itemtick::mth_floor(st.y - 0.999_999f32 as f64),
            st.sup_z,
        )
    } else {
        (
            centities_core::itemtick::mth_floor(st.x),
            centities_core::itemtick::mth_floor(st.y - 0.999_999f32 as f64),
            centities_core::itemtick::mth_floor(st.z),
        )
    }
}

/// JNI native: env is live on the calling (server) thread.
pub extern "system" fn n_item_invalidate(
    env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    wid: i32,
    n: i32,
    xs: jni::jintArray,
    ys: jni::jintArray,
    zs: jni::jintArray,
) {
    unsafe { n_item_invalidate_impl(env, _cls, wid, n, xs, ys, zs) }
}

unsafe fn n_item_invalidate_impl(
    env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    wid: i32,
    n: i32,
    xs: jni::jintArray,
    ys: jni::jintArray,
    zs: jni::jintArray,
) {
    let n = n.max(0) as usize;
    if n == 0 || wid < 0 || wid >= 8 {
        return;
    }
    let Ok(mut g) = crate::INDEX.try_lock() else {
        return;
    };
    let cst = g.get_or_insert_with(crate::new_state);
    if cst.items.is_none() {
        cst.items = Some(ItemCore::new());
    }
    let e = JniEnv::from_raw(env);
    let raw = e.raw();
    let vt = *raw;
    let mut vx = vec![0i32; n];
    let mut vy = vec![0i32; n];
    let mut vz = vec![0i32; n];
    ((*vt).GetIntArrayRegion)(raw, xs, 0, n as i32, vx.as_mut_ptr());
    ((*vt).GetIntArrayRegion)(raw, ys, 0, n as i32, vy.as_mut_ptr());
    ((*vt).GetIntArrayRegion)(raw, zs, 0, n as i32, vz.as_mut_ptr());
    let core = cst.items.as_mut().unwrap();
    let world = core.world(wid as usize);
    for k in 0..n {
        world.invalidate_section(vx[k], vy[k], vz[k]);
        // fr-memo hole-closer: items overlapping the changed section may sit
        // on a block whose friction/speed just changed. Query the index
        // (in-process, zero JNI) for the section's entity ids and clear
        // their memos — the next native tick re-resolves exactly.
        let sx = (vx[k] << 4) as f64;
        let sy = (vy[k] << 4) as f64;
        let sz = (vz[k] << 4) as f64;
        let box_ = centities_core::Aabb::new(sx, sy, sz, sx + 16.0, sy + 16.0, sz + 16.0);
        let mut hits: Vec<i64> = Vec::with_capacity(64);
        cst.idx.query(wid as u16, &box_, &mut hits);
        for id in hits {
            let slot = (id as u64 & STORE_MASK) as usize;
            let tag = (id as u64 >> 17) | 1;
            let e = &mut world.entries[slot];
            if e.tag == tag {
                e.fr_pos = (i32::MIN, i32::MIN, i32::MIN);
            }
        }
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    /// Java writes shapeOv=16 + complex=0 + zero cuboids for zero-collision
    /// context-dependent shapes (plants, torches). The section must NOT be
    /// poisoned as complex (the b66f61b-era parser aborted ~50% of the bench
    /// exactly this way); the entry carries no colliders.
    #[test]
    fn plant_shape_override_is_not_section_complex() {
        let mut d = vec![0.0; 24 * 2];
        d[24 + 23] = 16.0; // shape_override only, NO complex bit
        let b = vec![0i32; 4096];
        let s = parse_section(&d, &b);
        assert!(
            !s.complex,
            "shapeOv with 0 cuboids must not poison the section"
        );
        assert!(!s.entries[1].colliders.is_empty() == false);
        assert!(s.entries[1].colliders.is_empty());
        assert!(!s.entries[1].fall_on_special);
        assert!(!s.entries[1].step_on_special);
    }

    /// A genuine complex entry (fence/chest/snow-layer class: shapeOv AND
    /// boxes, or >3 cuboids) carries complex=32 from the provider and must
    /// route the section vanilla.
    #[test]
    fn complex_bit_routes_section_vanilla() {
        let mut d = vec![0.0; 24 * 2];
        d[24 + 23] = 32.0;
        let b = vec![0i32; 4096];
        assert!(parse_section(&d, &b).complex);
        // shapeOv + complex together (Java sets both for boxed shape overrides)
        let mut d2 = vec![0.0; 24 * 2];
        d2[24 + 23] = 48.0;
        let s2 = parse_section(&d2, &b);
        assert!(s2.complex);
        assert!(
            s2.entries[1].colliders.is_empty(),
            "no cuboids may parse for complex entries"
        );
    }

    /// A plain full-cube block (no shape override, no special bits) parses
    /// its single collider and leaves the section natively tickable.
    #[test]
    fn plain_block_keeps_single_collider() {
        let mut d = vec![0.0; 24 * 2];
        let o = 24;
        d[o] = 1.0; // 1 cuboid
        d[o + 1] = 0.0;
        d[o + 2] = 0.0;
        d[o + 3] = 0.0;
        d[o + 4] = 1.0;
        d[o + 5] = 1.0;
        d[o + 6] = 1.0;
        d[o + 19] = 0.6; // friction
        d[o + 20] = 1.0; // speed
        let b = vec![0i32; 4096];
        let s = parse_section(&d, &b);
        assert!(!s.complex);
        assert_eq!(s.entries[1].colliders.len(), 1);
        assert_eq!(s.entries[1].colliders[0].x1, 1.0);
        assert_eq!(s.entries[1].friction, 0.6);
    }

    /// stepOn/fallOn override bits (slime, honey, magma) survive the parse
    /// and stay per-entry (NOT section-complex — the tick core handles the
    /// routing from the per-block flags during the swept gather).
    #[test]
    fn step_fall_bits_stay_per_entry() {
        let mut d = vec![0.0; 24 * 2];
        d[24] = 1.0;
        d[24 + 4] = 1.0;
        d[24 + 5] = 1.0;
        d[24 + 6] = 1.0;
        d[24 + 7] = 1.0;
        d[24 + 23] = 12.0; // stepOn(4) + fallOn(8)
        let b = vec![0i32; 4096];
        let s = parse_section(&d, &b);
        assert!(!s.complex);
        assert!(s.entries[1].step_on_special);
        assert!(s.entries[1].fall_on_special);
    }
}
