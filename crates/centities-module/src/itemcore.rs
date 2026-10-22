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
use centities_core::sections::{ AIR_IDX, PaletteEntry, SectionData, SectionGrid, SectionProvider };
use centities_core::tick::{
    tick_item, tick_item_inactive, ItemState, Scratch, TickEnv, TickOutcome,
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

/// Per-world: the item store + the section cache.
pub struct ItemWorld {
    entries: Vec<ItemEntry>,
    pub grid: SectionGrid,
    pub scratch: Scratch,
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
            scratch: Scratch::new(),
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
        ItemCore { worlds: (0..8).map(|_| None).collect() }
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
    move |cx: i32, cy: i32, cz: i32, load: bool| -> Result<SectionData, ()> {
        let hooks_cls = HOOKS_CLASS_GLOBAL.load(std::sync::atomic::Ordering::Acquire);
        if hooks_cls == 0 {
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
            d.resize(18 * 256, 0.0);
            b.clear();
            b.resize(4096, AIR_IDX as i32);
            let arr_d = ((*vt).NewDoubleArray)(raw, (18 * 256) as i32);
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
            ((*vt).GetDoubleArrayRegion)(raw, arr_d, 0, (18 * 256) as i32, d.as_mut_ptr());
            ((*vt).GetIntArrayRegion)(raw, arr_b, 0, 4096, b.as_mut_ptr());
            ((*vt).DeleteLocalRef)(raw, arr_d);
            ((*vt).DeleteLocalRef)(raw, arr_b);
            Ok(parse_section(&d, &b))
        })
    })
}

fn parse_section(d: &[f64], b: &[i32]) -> SectionData {
    // entries: 18 doubles each: [collider, x0,y0,z0,x1,y1,z1, friction, speed,
    //          water, fluid, cobweb, insideSpecial, stepSpecial, fallSpecial,
    //          shapeOverride, complex, reserved]
    let mut entries: Vec<PaletteEntry> = Vec::with_capacity(16);
    let mut complex = false;
    let mut any_fluid = false;
    let mut any_inside = false;
    for slot in 0..d.len() / 18 {
        let o = slot * 18;
        let shape_override = d[o + 15] != 0.0;
        let entry_complex = d[o + 16] != 0.0;
        let collider = if d[o] == 1.0 && !shape_override && !entry_complex {
            Some(Aabb6::new(d[o + 1], d[o + 2], d[o + 3], d[o + 4], d[o + 5], d[o + 6]))
        } else {
            None
        };
        let entry = PaletteEntry {
            collider,
            friction: d[o + 7] as f32,
            speed: d[o + 8] as f32,
            is_water: d[o + 9] != 0.0,
            has_fluid: d[o + 10] != 0.0,
            cobweb: d[o + 11] != 0.0,
            entity_inside_special: d[o + 12] != 0.0 && d[o + 11] == 0.0,
            step_on_special: d[o + 13] != 0.0,
            fall_on_special: d[o + 14] != 0.0,
        };
        if shape_override || entry_complex {
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
    let mut blocks = vec![AIR_IDX; 4096];
    for (i, v) in b.iter().take(4096).enumerate() {
        blocks[i] = *v as u16;
    }
    SectionData { entries, blocks, complex, any_fluid, any_inside_special: any_inside }
}

// ---------------------------------------------------------------------------
// the flush native
// ---------------------------------------------------------------------------

// out bits (mirror of TickOutcome + the mode protocol)
const B_MOVED: i32 = 1;
const B_ON_GROUND: i32 = 1 << 1;
const B_IMPULSE: i32 = 1 << 2;
const B_MERGE: i32 = 1 << 5;
const B_DESPAWN: i32 = 1 << 7;
const B_VOID: i32 = 1 << 8;
const B_VANILLA_NEXT: i32 = 1 << 9;
const B_ABORTED: i32 = 1 << 10;
const B_LAND: i32 = 1 << 11;
const B_NATIVE_OK: i32 = 1 << 12;

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
    in_i: jni::jintArray,
    env_d: jni::jdoubleArray,
    env_i: jni::jintArray,
    out_d: jni::jdoubleArray,
    out_i: jni::jintArray,
) -> i32 {
    // SAFETY: the JVM passes a live env; every raw JNI call below mirrors the
    // vtable pattern proven in lib.rs.
    return unsafe { n_item_tick_impl(env, _cls, wid, n, ids, flags, in_d, in_i, env_d, env_i, out_d, out_i) };
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
                            d.resize(n * 14, 0.0);
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
                            ((*vt).GetDoubleArrayRegion)(raw, in_d, 0, (n * 14) as i32, d.as_mut_ptr());
                            ((*vt).GetIntArrayRegion)(raw, in_i, 0, (n * 4) as i32, iv.as_mut_ptr());
                            let mut envv_d = [0.0f64; 6];
                            let mut envv_i = [0i32; 4];
                            ((*vt).GetDoubleArrayRegion)(raw, env_d, 0, 6, envv_d.as_mut_ptr());
                            ((*vt).GetIntArrayRegion)(raw, env_i, 0, 4, envv_i.as_mut_ptr());
                            if e.exception_check() {
                                e.exception_clear();
                                return -1;
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
                            let mut provider = make_provider(wid);
                            let prov: SectionProvider = &mut provider;
                            for k in 0..n {
                                let id = idv[k];
                                let f = fv[k];
                                let mut st = ItemState {
                                    x: d[k * 14],
                                    y: d[k * 14 + 1],
                                    z: d[k * 14 + 2],
                                    dx: d[k * 14 + 3],
                                    dy: d[k * 14 + 4],
                                    dz: d[k * 14 + 5],
                                    xo: d[k * 14],
                                    yo: d[k * 14 + 1],
                                    zo: d[k * 14 + 2],
                                    on_ground: (f & F_ON_GROUND) != 0,
                                    fall_dist: d[k * 14 + 6] as f32,
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
                                let mut bits: i32 = 0;
                                if (f & (F_RESYNC | F_FORCE_VANILLA)) != 0 {
                                    // vanilla body ran this tick: ingest the fresh
                                    // state (incl. stuck/sup) and re-eval the mode.
                                    // L7: FULL state ingest — the entry becomes the
                                    // dispatch-owned source of truth once the item
                                    // re-arms, so everything must be current here.
                                    let entry = world.entry(id);
                                    entry.stuck = (d[k * 14 + 7], d[k * 14 + 8], d[k * 14 + 9]);
                                    entry.sup = (
                                        d[k * 14 + 10] as i32,
                                        d[k * 14 + 11] as i32,
                                        d[k * 14 + 12] as i32,
                                    );
                                    entry.sup_present = d[k * 14 + 13] != 0.0;
                                    entry.x = d[k * 14];
                                    entry.y = d[k * 14 + 1];
                                    entry.z = d[k * 14 + 2];
                                    entry.dx = d[k * 14 + 3];
                                    entry.dy = d[k * 14 + 4];
                                    entry.dz = d[k * 14 + 5];
                                    entry.age = iv[k * 4 + 1];
                                    entry.pickup_delay = iv[k * 4 + 2];
                                    entry.tick_count = iv[k * 4];
                                    entry.despawn_rate = iv[k * 4 + 3];
                                    entry.fall_dist = d[k * 14 + 6] as f32;
                                    entry.on_ground = (f & F_ON_GROUND) != 0;
                                    entry.aborts = 0;
                                    st.stuck = entry.stuck;
                                    st.sup_x = entry.sup.0;
                                    st.sup_y = entry.sup.1;
                                    st.sup_z = entry.sup.2;
                                    st.sup_present = entry.sup_present;
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
                                let outcome = if (f & F_INACTIVE) != 0 {
                                    tick_item_inactive(&mut st)
                                } else {
                                    tick_item(&mut st, id, &tenv, &mut world.grid, &mut world.scratch, prov)
                                };
                                if outcome.has(TickOutcome::ABORTED) {
                                    // no commit, no writeback: the item froze
                                    // this tick; vanilla computes it next tick.
                                    // L7 seatbelt: an item that aborts 4 times
                                    // in a row must not freeze forever — force
                                    // a vanilla tick (the resync re-ingests and
                                    // clears the streak).
                                    let (mode, clear) = {
                                        let entry = world.entry(id);
                                        entry.aborts = entry.aborts.saturating_add(1);
                                        if items_native
                                            && entry.aborts < 4
                                            && centities_core::tick::want_native(&st, &tenv)
                                        {
                                            (B_NATIVE_OK, false)
                                        } else {
                                            (B_VANILLA_NEXT, true)
                                        }
                                    };
                                    if clear {
                                        world.entry(id).aborts = 0;
                                    }
                                    oi[k * 4 + 2] = B_ABORTED | mode;
                                    continue;
                                }
                                // commit the store
                                {
                                    let entry = world.entry(id);
                                    entry.stuck = st.stuck;
                                    entry.sup = (st.sup_x, st.sup_y, st.sup_z);
                                    entry.sup_present = st.sup_present;
                                    entry.sup_no_blocks = st.on_ground_no_blocks;
                                    entry.fr_pos = st.fr_pos;
                                    entry.friction = st.friction;
                                    entry.speed = st.speed;
                                    entry.fr_step_special = st.fr_step_special;
                                    // L7: mirror the full state back (dispatch-owned)
                                    entry.x = st.x;
                                    entry.y = st.y;
                                    entry.z = st.z;
                                    entry.dx = st.dx;
                                    entry.dy = st.dy;
                                    entry.dz = st.dz;
                                    entry.age = st.age;
                                    entry.pickup_delay = st.pickup_delay;
                                    entry.tick_count = st.tick_count;
                                    entry.fall_dist = st.fall_dist;
                                    entry.on_ground = st.on_ground;
                                    entry.aborts = 0;
                                    if (f & F_NATIVE) == 0 {
                                        // collector-carried entries refresh despawnRate
                                        entry.despawn_rate = iv[k * 4 + 3];
                                    }
                                }
                                od[k * 8] = st.dx;
                                od[k * 8 + 1] = st.dy;
                                od[k * 8 + 2] = st.dz;
                                od[k * 8 + 3] = st.x;
                                od[k * 8 + 4] = st.y;
                                od[k * 8 + 5] = st.z;
                                od[k * 8 + 6] = st.fall_dist as f64;
                                od[k * 8 + 7] = st.land_fall as f64;
                                oi[k * 4] = st.age;
                                oi[k * 4 + 1] = st.pickup_delay;
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
                                    // L7 cadence belt: every 256th tick the item runs
                                    // one full vanilla tick + re-ingest (heals any
                                    // drift window: probes, hash gaps, plugin writes).
                                    || (st.tick_count & 0xFF) == 0
                                {
                                    ob |= B_VANILLA_NEXT;
                                } else {
                                    ob |= B_NATIVE_OK;
                                }
                                oi[k * 4 + 2] = ob;
                                oi[k * 4 + 3] = st.tick_count;
                            }
                            ((*vt).SetDoubleArrayRegion)(raw, out_d, 0, (n * 8) as i32, od.as_ptr());
                            ((*vt).SetIntArrayRegion)(raw, out_i, 0, (n * 4) as i32, oi.as_ptr());
                            {
                                let dt = t0.elapsed().as_nanos() as u64;
                                FLUSH_NANOS.fetch_add(dt, std::sync::atomic::Ordering::Relaxed);
                                let cnt = FLUSH_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                                if cnt % 200 == 0 {
                                    let total = FLUSH_NANOS.load(std::sync::atomic::Ordering::Relaxed);
                                    cplug_sdk::log::info(&format!(
                                        "[c-entities] item-flush: {} flushes, avg {:.1}us, last n={}",
                                        cnt,
                                        total as f64 / cnt as f64 / 1000.0,
                                        n
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
        (st.sup_x, centities_core::itemtick::mth_floor(st.y - 0.999_999f32 as f64), st.sup_z)
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
    let Ok(mut g) = crate::INDEX.try_lock() else { return };
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
