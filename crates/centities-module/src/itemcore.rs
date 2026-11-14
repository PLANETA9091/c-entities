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

/// The L55-diag traced id (CE_RTRACE_ID) — shared by the FTICK flush print
/// and the CMT commit witness. Cached: the env is fixed for the process.
fn rtrace_id() -> i64 {
    static RT: std::sync::OnceLock<i64> = std::sync::OnceLock::new();
    *RT.get_or_init(|| {
        std::env::var("CE_RTRACE_ID")
            .ok()
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(-1)
    })
}

#[derive(Clone, Copy)]
struct ItemEntry {
    tag: u64, // (id >> 17) | 1 — validity check against slot aliasing
    /// THE EXACT entity id. The tag degenerates below 2^18 ids — every id
    /// 0..262143 folds to (id>>17)|1 == 1, so at the 150k-entity scale a
    /// zombie's masked slot can carry a MATCHING tag (the mixed-150k
    /// ClassCastException disarm: the merge scan returned a zombie id whose
    /// tag equaled the live item's). The full id is the only honest
    /// membership test; entry() stamps it at ingest, fresh(empty) writes 0
    /// (ids are strictly positive) so an empty slot matches nothing.
    id: i64,
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
            id: 0,
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
        if e.tag != tag || e.id != id {
            *e = ItemEntry::fresh(tag);
            e.id = id;
        }
        e
    }

    /// The exact item-membership test for index candidates: the shared
    /// query index holds EVERY entity (the push/mob wires), so a merge scan
    /// over a cell can hit a zombie standing on a dropped stack. The entry
    /// table is items-only (entry() ingests item state exclusively), and
    /// the id compare is exact — a live item in this world matches, a
    /// never-ingested or aliased slot does not.
    #[inline]
    fn is_live_item(&self, id: i64) -> bool {
        let e = &self.entries[(id as u64 & STORE_MASK) as usize];
        e.id == id
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
/// Diagnostic: the merge cadence saw a floor change (rate 2, not 40) —
/// rides ONLY to feed the MS telemetry line (mv=1); the applier ignores it.
const B_MERGE2: i32 = 1 << 6;
const B_DESPAWN: i32 = 1 << 7;
const B_VOID: i32 = 1 << 8;
const B_LAND: i32 = 1 << 9;
const B_VANILLA_NEXT: i32 = 1 << 11;
const B_NATIVE_OK: i32 = 1 << 12;
const B_ABORTED: i32 = 1 << 13;
/// L16: post-tick delta is bit-identical to the pre-tick delta. Vanilla
/// still allocates a fresh Vec3 and rewrites the field every tick — the
/// FIELD VALUE is the same, so skipping the Java writeback is value-exact.
/// A resting item reaches the (0,0,0) fixed point after landing: gravity
/// (-0.04) is applied, move() collides and zeroes the collided axis, and
/// 0.0 * friction == 0.0 exactly — so the mask holds every tick after the
/// item settles and the flush drops the setDeltaMovement call entirely.
const B_DELTA_SAME: i32 = 1 << 14;
/// REST candidate: the tick was a VALUE fixed point (pos/delta/ground/fall
/// identical, no events, no vanilla_next) — the dispatch guard may run the
/// counter arithmetic (tickCount/totalEntityAge/age/pickupDelay/despawn)
/// inline and keep the item out of the flush cycle entirely. Wakes
/// (probes, section changes, the 256-tick belt) clear the rest mode; the
/// next full native tick re-proves it.
const B_REST_CAND: i32 = 1 << 15;
/// L78 TAIL_NONE: the Java applier's ENTIRE post-counter tail is a provable
/// no-op for this entry (DELTA_SAME + no event bits + the out-fd/pd/onGround
/// equal their pre-states; for inactive entries — no DESPAWN/VOID/LAND + the
/// out-pd equals the pre-pd). The applier skips straight to the next entry
/// after the counters — the skipped compares are exactly the ones the tails
/// would skip themselves.
const B_TAIL_NONE: i32 = 1 << 16;

// L118-diag: the outcome-bits census (the merge=0 hunt) — module-level so
// both the per-item body and the flush-stats print reach them.
static CB_MERGE_S: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static CB_ACT_S: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static CB_INACT_S: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static CB_TICKS_S: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
use CB_MERGE_S as CB_MERGE;
use CB_ACT_S as CB_ACT;
use CB_INACT_S as CB_INACT;
use CB_TICKS_S as CB_TICKS;
/// i0-a STAY verdict (doc 62): the Java dispatch guard's activation port
/// early-returns TRUE with ZERO side effects on the %4 rotation clause
/// ((tickCount+1+id) % 4 == 0 — itemCheckActive's first clause returns
/// before isTemporarilyActive=false). The core owns tick_count (the
/// applier mirrors it into e.tickCount) and the entity id, so the NEXT
/// dispatch's fast-path verdict is computable exactly here: the Java
/// guard (CeHooks.itemDispatch) skips the whole gate chain and appends
/// active. Inactive entries and the 256-tick belt keep the proven slow
/// chain (VANILLA_NEXT => no verdict; the applier clears the slot).
const B_STAY: i32 = 1 << 17;
/// i0-a2 (doc 62): the full verdict's decision — the fast path's active
/// flag (DISP_VAL); paired with B_IS_TEMP (the isTemporarilyActive
/// writeback bit).
const B_VERDICT: i32 = 1 << 18;
const B_IS_TEMP: i32 = 1 << 19;
/// i0-a2b: the INACTIVE verdict — the fast path appends with
/// IF_INACTIVE for these (the stamp covers ~100% of the steady state).
const B_VERDICT_INACT: i32 = 1 << 20;
/// i0-a2c: the activatedTick writeback is REAL (at_write was Some) — the
/// applier writes the Java field only then; the echo case (None) leaves
/// the field untouched (value-exact: the batch carried the current value).
const B_AT_WRITE: i32 = 1 << 21;

// flags from Java (mirror of CeHooks.IF_*)
const F_INACTIVE: i32 = 1;

/// i0-a2 (doc 62): the activation verdict — the EXACT port of
/// CeHooks.itemCheckActive, computed post-tick over the batch-carried
/// truth: `at` = the Java activatedTick at the NEXT dispatch (the
/// vanilla sweep's stamps ride it — the sweep runs before the
/// dispatches), `was_temp` = the field's pre-verdict value, `def` =
/// defaultActivationState. The clause order is the port's; the Java-
/// visible side effects return packed: (is_active, at_write, is_temp).
///
/// The port for reference (the %4 clause first — ZERO side effects):
///   tc = tickCount+1; if ((tc+id) % 4 == 0) return true;
///   if (defaultActivationState) return true;
///   if (tc < 200) return true;
///   isActive = activatedTick >= cur; isTemporarilyActive = false;
///   if (!isActive && (cur - activatedTick - 1) % 20 == 0) {
///       i = itemImmunities(e); if (i >= 0) activatedTick = cur + i;
///       else isTemporarilyActive = true; isActive = true; }
///   return isActive;
///
/// L118 THE SWEEP-TIME CONVENTIONS (the parity-items zero-merge root, red
/// since i0-a2b): the verdict serves the NEXT tick's ACTIVATION SWEEP, and
/// the sweep reads the entity's tickCount BEFORE that tick's dispatcher
/// increment plus the NEXT MinecraftServer.currentTick — the port's
/// dispatch-time conventions (tc = post-++ tickCount, cur = this tick)
/// shifted EVERY clause by one: the %4 exception fired on the wrong tick
/// (the no-player parity legs went 100% inactive — the full body never
/// ran — tick_item_inactive served counters only — the merge cadence,
/// which lives in the full body, never fired: merge=0, the ON leg kept
/// 20000 items while vanilla merged 3463). The javap ground truth
/// (ActivationRange.checkIfActive, 26.3-249): the sweep's tickCount = the
/// post-previous-tick value = st.tick_count (the core's post-body mirror)
/// = tc - 1, and the sweep's currentTick = this batch's cur_tick + 1.
///
/// itemImmunities for a NATIVE item reduces exactly: fire routes vanilla
/// (the mirror carries no fire — the water/lava/fire gates), the
/// activatedImmunityTick clause is dead (CeHooks never writes the field —
/// javap cross-check), water routes vanilla — the remaining: !onGround →
/// 100, onGround && |dv|² > 9.999999747378752E-6 → 100, else -1.
#[inline]
fn activation_verdict(
    id: i64,
    tc: i32,
    tenv: &TickEnv,
    at: i64,
    was_temp: bool,
    def: bool,
    on_ground: bool,
    dx: f64,
    dz: f64,
) -> (bool, Option<i64>, bool) {
    // L119 THE PHASE FIX (the L118 sweep-time note mis-read tickNonPassenger):
    // the verdict tc argument = st.tick_count + 1 = POST_{T+1} = EXACTLY the
    // tickCount the vanilla checkIfActive reads — ServerLevel.tickNonPassenger
    // 26.3-249 offsets 55-62: commonTick (tickCount++) runs BEFORE
    // checkIfActive, so the sweep sees the POST value, not the PRE. The Java
    // source (CeHooks.itemCheckActive, tc = e.tickCount + 1) was always
    // right; the port's "- 1" and "tc < 201" were the off-by-ones — the
    // %4 activation phase was inverted (vanilla-active ticks went inactive
    // and vice versa) and POST=200 granted one extra active tick (the L52
    // bug class, reintroduced). Byte-exact ground trace: run tr29, OFF id 29
    // moved at POST tc 15/19/23 = (tc+id)%4==0 with tc<200 active-everywhere.
    if ((tc as i64 + id) % 4) == 0 {
        return (true, None, was_temp);
    }
    if def {
        return (true, None, was_temp);
    }
    if tc < 200 {
        return (true, None, was_temp);
    }
    let cur = tenv.cur_tick as i64 + 1;
    let mut is_active = at >= cur;
    let mut at_write: Option<i64> = None;
    let mut is_temp = false; // e.isTemporarilyActive = false (the port writes it)
    if !is_active && (cur - at - 1) % 20 == 0 {
        let dv2 = dx * dx + dz * dz;
        let immunity: i64 = if !on_ground || dv2 > 9.999_999_747_378_752E-6 {
            100
        } else {
            -1
        };
        if immunity >= 0 {
            at_write = Some(cur + immunity);
        } else {
            is_temp = true;
        }
        is_active = true;
    }
    (is_active, at_write, is_temp)
}
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
    /// i0-a2 (doc 62): the per-entry activation truth — [atLo, atHi, flags2]
    /// (flags2: bit0 = isTemporarilyActive, bit1 = defaultActivationState),
    /// packed by the flush from the live entities (the vanilla sweep's
    /// stamps ride it).
    static T_ATI: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
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
    at: jni::jintArray,
) -> i32 { let _site_guard = centities_core::SiteGuard::enter(6); 
    // SAFETY: the JVM passes a live env; every raw JNI call below mirrors the
    // vtable pattern proven in lib.rs.
    return unsafe {
        n_item_tick_impl(
            env, _cls, wid, n, ids, flags, in_d, cdn, in_i, env_d, env_i, out_d, out_i, at,
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
    at: jni::jintArray,
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
                    T_ATI.with(|tat| {
                            let mut d = td.borrow_mut();
                            let mut iv = ti.borrow_mut();
                            let mut idv = tid.borrow_mut();
                            let mut fv = tf.borrow_mut();
                            let mut od = tod.borrow_mut();
                            let mut oi = toi.borrow_mut();
                            let mut atv = tat.borrow_mut();
                            atv.resize(n * 3, 0);
                            ((*vt).GetIntArrayRegion)(raw, at, 0, (n * 3) as i32, atv.as_mut_ptr());
                            iv.resize(n * 4, 0);
                            idv.resize(n, 0);
                            fv.resize(n, 0);
                            od.resize(n * 9, 0.0);
                            // L7 outI layout: [age, pickupDelay, bits, tickCount] —
                            // the applier mirrors the dispatcher's tickCount++/totalEntityAge++
                            // for entries whose vanilla dispatcher was suppressed.
                            // L79 COMPACT APPLY: outI grows to n*4 + 1 + n —
                            // outI[n*4] = cn, then the batch indices whose Java
                            // post-counter tail is NOT the proven TAIL_NONE
                            // no-op (the applier's loop B walks only those).
                            oi.resize(n * 5 + 1, 0);
                            ((*vt).GetLongArrayRegion)(raw, ids, 0, n as i32, idv.as_mut_ptr());
                            ((*vt).GetIntArrayRegion)(raw, flags, 0, n as i32, fv.as_mut_ptr());
                            ((*vt).GetIntArrayRegion)(raw, in_i, 0, (n * 4) as i32, iv.as_mut_ptr());
                            let mut envv_d = [0.0f64; 6];
                            let mut envv_i = [0i32; 6];
                            ((*vt).GetDoubleArrayRegion)(raw, env_d, 0, 6, envv_d.as_mut_ptr());
                            ((*vt).GetIntArrayRegion)(raw, env_i, 0, 6, envv_i.as_mut_ptr());
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
                                cur_tick: envv_i[4],
                                verdict_owned: envv_i[5] != 0,
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
                                    // Batch tc semantics: PRE when the body was
                                    // suppressed (IF_NATIVE — the dispatcher's ++
                                    // runs after the tick body, trace-proven run
                                    // 40728505667), POST when the vanilla body
                                    // already ran (IF_RESYNC / IF_FORCE_VANILLA).
                                    // The store convention is POST; tick_item's
                                    // vanilla cadences read the PRE value via
                                    // tc-1, so both paths must land on POST.
                                    tick_count: if (f & F_NATIVE) != 0 {
                                        iv[k * 4].wrapping_add(1)
                                    } else {
                                        iv[k * 4]
                                    },
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
                                    default_activation: false,
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
                                    od[k * 9] = st.dx;
                                    od[k * 9 + 1] = st.dy;
                                    od[k * 9 + 2] = st.dz;
                                    od[k * 9 + 3] = st.x;
                                    od[k * 9 + 4] = st.y;
                                    od[k * 9 + 5] = st.z;
                                    od[k * 9 + 6] = st.fall_dist as f64;
                                    // i0-a2: the resync entries carry no verdict (the
                                    // next dispatch re-derives on the slow path); the
                                    // writeback echoes the batch's at so the applier
                                    // never writes a stale slot-8 value.
                                    od[k * 9 + 8] = ((atv[k * 3 + 1] as i64) << 32
                                        | (atv[k * 3] as i64 & 0xFFFF_FFFF))
                                        as f64;
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
                                    .zip(od.par_chunks_mut(512 * 9))
                                    .zip(oi.par_chunks_mut(512 * 4))
                                    .zip(c_flag.par_chunks_mut(512))
                                    .zip(c_l6.par_chunks_mut(512))
                                    .zip(tickable.par_chunks(512))
                                    .zip(aborts_in.par_chunks(512))
                                    .zip(idv.par_chunks(512))
                                    .zip(fv.par_chunks(512))
                                    .zip(atv.par_chunks(512 * 3))
                                    .for_each(
                                        |(((((((((st_c, od_c), oi_c), cf_c), cl_c), tk_c), ab_c), id_c), f_c), at_c)| {
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
                                                // L53-diag: the flush-level view of the
                                                // traced id (CE_RTRACE_ID): the batch
                                                // flags + the store state AT the tick.
                                                static RTICK_ID: std::sync::OnceLock<i64> =
                                                    std::sync::OnceLock::new();
                                                let rtick = *RTICK_ID.get_or_init(|| {
                                                    let v = std::env::var("CE_RTRACE_ID")
                                                        .ok()
                                                        .and_then(|v| v.parse::<i64>().ok())
                                                        .unwrap_or(-1);
                                                    eprintln!("[ce] RTICK init: env={} -> {}", std::env::var("CE_RTRACE_ID").unwrap_or_else(|_| "<absent>".into()), v);
                                                    v
                                                });
                                                if rtick >= 0 && id == rtick {
                                                    eprintln!(
                                                        "[ce] FTICK id={} f={:#x} inactive={} tc={} onG={} dx={:e} dy={:e}",
                                                        id, f, (f & F_INACTIVE) != 0, st.tick_count, st.on_ground, st.dx, st.dy
                                                    );
                                                }
                                                // L16: pre-tick delta snapshot for
                                                // the DELTA_SAME mask below; the
                                                // rest of the state snapshots feed
                                                // the REST_CAND fixed-point test
                                                let (pdx, pdy, pdz) = (st.dx, st.dy, st.dz);
                                                let (px, py, pz) = (st.x, st.y, st.z);
                                                let pground = st.on_ground;
                                                let pfall = st.fall_dist;
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
                                                od_c[j * 9] = st.dx;
                                                od_c[j * 9 + 1] = st.dy;
                                                od_c[j * 9 + 2] = st.dz;
                                                od_c[j * 9 + 3] = st.x;
                                                od_c[j * 9 + 4] = st.y;
                                                od_c[j * 9 + 5] = st.z;
                                                od_c[j * 9 + 6] = st.fall_dist as f64;
                                                od_c[j * 9 + 7] = st.land_fall as f64;
                                                oi_c[j * 4] = st.age;
                                                oi_c[j * 4 + 1] = st.pickup_delay;
                                                let mut ob = 0i32;
                                                if outcome.has(TickOutcome::MOVED) { ob |= B_MOVED; }
                                                if outcome.has(TickOutcome::ON_GROUND) { ob |= B_ON_GROUND; }
                                                if outcome.has(TickOutcome::HAS_IMPULSE) { ob |= B_IMPULSE; }
                                                if st.dx == pdx && st.dy == pdy && st.dz == pdz {
                                                    ob |= B_DELTA_SAME;
                                                }
                                                if outcome.has(TickOutcome::MERGE) { ob |= B_MERGE; }
                                                if outcome.has(TickOutcome::MERGE2) { ob |= B_MERGE2; }
                                                // L118-diag: the outcome-bits census (the merge=0 hunt)
                                                // (the counters are module-level: CB_*_S, aliased
                                                // CB_* above — the print in the flush-stats reads
                                                // the same pair)
                                                CB_TICKS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                                                {
                                                    let tcc = CB_TICKS.load(std::sync::atomic::Ordering::Relaxed);
                                                    if tcc % 20000 == 0 {
                                                        eprintln!("[ce] tickc={} merge={} id={}", tcc, CB_MERGE.load(std::sync::atomic::Ordering::Relaxed), id);
                                                    }
                                                }
                                                if ob & B_MERGE != 0 { CB_MERGE.fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
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
                                                    // REST candidate: a value fixed point
                                                    // with no events. The %4 zero-delta
                                                    // move gate writes setPos with
                                                    // IDENTICAL values (MOVED set, no
                                                    // state change) — the pos compare
                                                    // below is the exact test, so MOVED
                                                    // is deliberately not in the mask.
                                                    let quiet = outcome.0
                                                        & (TickOutcome::HAS_IMPULSE
                                                            | TickOutcome::H_COLL
                                                            | TickOutcome::V_COLL
                                                            | TickOutcome::MERGE
                                                            | TickOutcome::MERGE2
                                                            | TickOutcome::DESPAWN
                                                            | TickOutcome::VOID
                                                            | TickOutcome::ABORTED
                                                            | TickOutcome::LAND)
                                                        == 0;
                                                    if quiet
                                                        && st.x == px
                                                        && st.y == py
                                                        && st.z == pz
                                                        && st.dx == pdx
                                                        && st.dy == pdy
                                                        && st.dz == pdz
                                                        && st.on_ground == pground
                                                        && st.fall_dist == pfall
                                                    {
                                                        ob |= B_REST_CAND;
                                                    }
                                                    // i0-a2 (doc 62): the FULL verdict, computed
                                                    // POST-tick — tick_count is the post-T value
                                                    // the T+1 dispatch will read; `at` rides the
                                                    // batch (the post-sweep Java truth). The STAY
                                                    // bit (the %4 clause) folds into the verdict:
                                                    // the fast path fires from DISP_VAL for every
                                                    // native entry whose decision the core knows.
                                                    let at_in = ((at_c[j * 3 + 1] as i64) << 32)
                                                        | (at_c[j * 3] as i64 & 0xFFFF_FFFF);
                                                    let (v_active, at_write, v_temp) =
                                                        activation_verdict(
                                                            id,
                                                            st.tick_count + 1,
                                                            &tenv,
                                                            at_in,
                                                            (at_c[j * 3 + 2] & 1) != 0,
                                                            (at_c[j * 3 + 2] & 2) != 0,
                                                            st.on_ground,
                                                            st.dx,
                                                            st.dz,
                                                        );
                                                    if v_active {
                                                        ob |= B_VERDICT;
                                                    } else {
                                                        ob |= B_VERDICT_INACT;
                                                    }
                                                    if v_temp {
                                                        ob |= B_IS_TEMP;
                                                    }
                                                    // od slot 8: the activatedTick the applier
                                                    // writes back ONLY on a real stamp (bit 21);
                                                    // the echo case leaves the field untouched —
                                                    // the batch carried the current value and the
                                                    // port had no side effect (the %4 early-return
                                                    // and the already-false isTemp write skip).
                                                    match at_write {
                                                        Some(v) => {
                                                            od_c[j * 9 + 8] = v as f64;
                                                            ob |= B_AT_WRITE;
                                                        }
                                                        None => od_c[j * 9 + 8] = 0.0,
                                                    }
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
                            // L79: the compact index list lives at oi[n4+1..];
                            // the count lands at oi[n4] after the loop.
                            let n4 = n * 4;
                            let mut cn: usize = 0;
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
                                    entry.x = od[k * 9 + 3];
                                    entry.y = od[k * 9 + 4];
                                    entry.z = od[k * 9 + 5];
                                    entry.dx = od[k * 9];
                                    entry.dy = od[k * 9 + 1];
                                    entry.dz = od[k * 9 + 2];
                                    entry.age = oi[k * 4];
                                    entry.pickup_delay = oi[k * 4 + 1];
                                    entry.tick_count = oi[k * 4 + 3];
                                    // L78 TAIL_NONE: the pre-fd/ong must be read
                                    // BEFORE the overwrite below — they are the
                                    // same pre-states the Java tails compare
                                    // against (the store mirrors the entity at
                                    // batch time; mid-batch mutation rides the
                                    // removed/merge paths which never reach this
                                    // commit).
                                    let l78_pre_fd = entry.fall_dist;
                                    let l78_pre_ong = entry.on_ground;
                                    entry.fall_dist = od[k * 9 + 6] as f32;
                                    // L55-diag: the commit-side witness for the
                                    // traced id — the outcome bit AND the entry's
                                    // pre/post on_ground at the SAME line. The
                                    // belt dust (addendum 14) showed the store's
                                    // on_ground false at a t=412 ingest with no
                                    // body commit in the t=411 window; this names
                                    // the writing commit (or proves the t=410
                                    // rotation's own commit wrote FALSE while the
                                    // writeback left the Java entity TRUE).
                                    let cmt_pre_ong = entry.on_ground;
                                    let cmt_out_ong = (oi[k * 4 + 2] & B_ON_GROUND) != 0;
                                    let cmt_aborts = entry.aborts;
                                    entry.on_ground = cmt_out_ong;
                                    if rtrace_id() == id {
                                        eprintln!(
                                            "[ce] CMT id={} tc={} out_onG={} pre_onG={} aborts_in={}",
                                            id, entry.tick_count, cmt_out_ong, cmt_pre_ong, cmt_aborts
                                        );
                                    }
                                    entry.aborts = 0;
                                    if (f & F_NATIVE) == 0 {
                                        // collector-carried entries refresh despawnRate
                                        entry.despawn_rate = iv[k * 4 + 3];
                                    }
                                    // L78 TAIL_NONE — see the B_TAIL_NONE doc.
                                    // The pre-fd/ong were captured above BEFORE
                                    // the store overwrite (l78_pre_fd/ong).
                                    let bits_now = oi[k * 4 + 2];
                                    let inactive_k = (fv[k] & F_INACTIVE) != 0;
                                    let out_pd = oi[k * 4 + 1];
                                    let pre_pd = iv[k * 4 + 2];
                                    let tail_none = if inactive_k {
                                        (bits_now & (B_DESPAWN | B_VOID | B_LAND)) == 0
                                            && out_pd == pre_pd
                                    } else {
                                        (bits_now & B_DELTA_SAME) != 0
                                            && (bits_now
                                                & (B_MOVED | B_IMPULSE | B_MERGE
                                                    | B_DESPAWN | B_VOID | B_LAND))
                                                == 0
                                            && (od[k * 9 + 6] as f32) == l78_pre_fd
                                            && out_pd == pre_pd
                                            && ((bits_now & B_ON_GROUND) != 0) == l78_pre_ong
                                    };
                                    if tail_none {
                                        oi[k * 4 + 2] = bits_now | B_TAIL_NONE;
                                    } else if (f & F_NATIVE) != 0 {
                                        // L79 COMPACT APPLY: the applier's loop B
                                        // runs the full tail for exactly these —
                                        // the !TAIL_NONE set, the same predicate
                                        // the 12/12 L78 gate proved (non-NATIVE
                                        // entries never reach the Java tail: the
                                        // applier's IF_NATIVE gate continues first).
                                        oi[n4 + 1 + cn] = k as i32;
                                        cn += 1;
                                    }
                                }
                                // state extremes (garbage-detector diagnostics)
                                let ad = od[k * 9]
                                    .abs()
                                    .max(od[k * 9 + 1].abs())
                                    .max(od[k * 9 + 2].abs());
                                if ad.is_finite() && ad > max_abs_delta {
                                    max_abs_delta = ad;
                                }
                                let ap = od[k * 9 + 3]
                                    .abs()
                                    .max(od[k * 9 + 4].abs())
                                    .max(od[k * 9 + 5].abs());
                                if ap.is_finite() && ap > max_abs_pos {
                                    max_abs_pos = ap;
                                }
                            }

                            oi[n4] = cn as i32;
                            ((*vt).SetDoubleArrayRegion)(raw, out_d, 0, (n * 9) as i32, od.as_ptr());
                            ((*vt).SetIntArrayRegion)(raw, out_i, 0, (n4 + 1 + cn) as i32, oi.as_ptr());
                            {
                                let dt = t0.elapsed().as_nanos() as u64;
                                FLUSH_NANOS.fetch_add(dt, std::sync::atomic::Ordering::Relaxed);
                                let cnt = FLUSH_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                                if cnt % 50 == 0 {
                                    let total = FLUSH_NANOS.load(std::sync::atomic::Ordering::Relaxed);
                                    let g = &world.grid;
                                    let idx_stats = cst.idx.stats();
                                    cplug_sdk::log::info(&format!(
                                        "[c-entities] item-flush: {} flushes, avg {:.1}us, last n={} cd={} rss={}MB, upcalls={} errs={} grid pop={} look={} idx ents={} cells={} oversize={} secX[{}..{}] secY[{}..{}] secZ[{}..{}] maxD{} maxP{} ok{} ab{} stuck{} unl{} fld{} cpx{} fal{}) bits(merge={} act={} inact={} ticks={})",
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
                                        CB_MERGE.load(std::sync::atomic::Ordering::Relaxed),
                                        CB_ACT.load(std::sync::atomic::Ordering::Relaxed),
                                        CB_INACT.load(std::sync::atomic::Ordering::Relaxed),
                                        CB_TICKS.load(std::sync::atomic::Ordering::Relaxed),
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
            let e = &mut world.entries[slot];
            // exact id (the tag degenerates below 2^18 — an aliased clear
            // would wipe an innocent item's friction memo)
            if e.id == id {
                e.fr_pos = (i32::MIN, i32::MIN, i32::MIN);
            }
        }
    }
}

/// THE STORM BISECT (doc 62 addendum 5): the v3 export re-applied — the
/// Java call happens but the result is IGNORED (fast=false always). If the
/// storm persists, the CALL ITSELF is the trigger; if not, the absorption.
#[allow(clippy::too_many_arguments)]
pub extern "system" fn n_merge_scan(
    env: *mut jni::JNIEnv,
    _cls: jni::jclass,
    wid: i32,
    x: f64,
    y: f64,
    z: f64,
    r: f64,
    yinf: f64,
    self_id: jni::jlong,
    out: jni::jlongArray,
) -> i32 { let _site_guard = centities_core::SiteGuard::enter(5); 
    // L118-diag: the scan's candidate census (the parity-items zero-merge
    // hunt) — the counters ride the hooks line via MS_STATS take.
    static MS_CALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    static MS_RAW: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    static MS_STALE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    static MS_ZERO: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    static MS_REFUSED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    pub static MS_STATS: [std::sync::atomic::AtomicU64; 7] = [
        std::sync::atomic::AtomicU64::new(0), std::sync::atomic::AtomicU64::new(0),
        std::sync::atomic::AtomicU64::new(0), std::sync::atomic::AtomicU64::new(0),
        std::sync::atomic::AtomicU64::new(0), std::sync::atomic::AtomicU64::new(0),
        std::sync::atomic::AtomicU64::new(0),
    ];
    let refused = |which: usize| -> i32 {
        MS_STATS[which].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        MS_REFUSED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        -1
    };
    if wid < 0 || wid >= 8 || r.is_nan() {
        return refused(1);
    }
    let Ok(mut g) = crate::INDEX.try_lock() else {
        return refused(2);
    };
    let cst = g.get_or_insert_with(crate::new_state);
    let Some(core) = cst.items.as_mut() else {
        return refused(3);
    };
    if wid as usize >= core.worlds.len() || core.worlds[wid as usize].is_none() {
        return refused(4);
    }
    // L118 HEALTH GUARD: the merge scan's fast=true-with-zero-candidates on
    // an UNPOPULATED index silently preempts the vanilla-body fallback (the
    // parity legs: the void-world index never populated while the items
    // store rode the batch — the ON leg performed ZERO merges where vanilla
    // merged 3463). An empty index means the scan cannot know the truth:
    // refuse (-1) and let the legacy scanner's own refusal route the exact
    // vanilla body. The perf legs hold a populated index — the guard never
    // fires there.
    if cst.idx.stats.entities == 0 {
        return refused(6);
    }
    MS_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let world = core.world(wid as usize);
    let qbox = centities_core::Aabb::new(
        x - 0.125 - r,
        y - yinf,
        z - 0.125 - r,
        x + 0.125 + r,
        y + 0.25 + yinf,
        z + 0.125 + r,
    );
    // L119-DIAG: the surgical scan trace — CE_MSCAN_TRACE=<entity id>
    // dumps the walked cells + every row's stored box + the exact-f64
    // intersect verdict for THAT scanner's query, before the query runs.
    static MT_ID: std::sync::OnceLock<i64> = std::sync::OnceLock::new();
    let mt = *MT_ID.get_or_init(|| {
        std::env::var("CE_MSCAN_TRACE")
            .ok()
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(-1)
    });
    if mt == self_id {
        eprintln!(
            "[ce] MSCAN self={} {}",
            self_id,
            cst.idx.debug_scan_trace(wid as u16, &qbox)
        );
    }
    let mut cands: Vec<i64> = Vec::with_capacity(16);
    cst.idx.query(wid as u16, &qbox, &mut cands);
    // L119 THE EXACT VIEW TRIM: the index answers a PROVABLE SUPERSET (the
    // f32 store's 7.6e-6 cell-relative rounding + the 1e-2 EPSF prefilter —
    // the doc-21 contract; the caller was expected to exactify). The stored
    // boxes ARE the scan-time view (the positional drain holds every not-
    // yet-reached entity's post-move box, so the stored box equals the live
    // box vanilla's mid-loop getEntities would read). Trim here with the
    // vanilla strict AABB.intersects on the stored boxes — the Java-side
    // live-box trim was WRONG (the live bb is POST-loop for the not-yet-
    // reached entities, which ate the true 0.0061-tall t=159 overlap).
    cands.retain(|&cid| {
        match cst.idx.box_of(wid as u16, cid) {
            Some(b) => {
                b[3] > qbox.x0 && b[0] < qbox.x1
                    && b[4] > qbox.y0 && b[1] < qbox.y1
                    && b[5] > qbox.z0 && b[2] < qbox.z1
            }
            None => false,
        }
    });
    MS_RAW.fetch_add(cands.len() as u64, std::sync::atomic::Ordering::Relaxed);
    let mut n = 0usize;
    unsafe {
        let e = JniEnv::from_raw(env);
        let raw = e.raw();
        let vt = *raw;
        let out_len = ((*vt).GetArrayLength)(raw, out) as usize;
        let mut buf: Vec<i64> = Vec::with_capacity(out_len);
        for &cid in cands.iter() {
            if cid == self_id {
                continue;
            }
            // L150a EXACT MEMBERSHIP: the shared index answers with EVERY
            // entity in the box — a zombie overlapping the scan qbox carried
            // a matching degenerate tag ((id>>17)|1 == 1 for ALL ids below
            // 2^18 — the whole 150k fill) and reached the Java cast as an
            // ItemEntity, disarming the module mid-run (the mixed-150k
            // ClassCastException). The entry table is items-only; the full
            // id compare is the exact filter, and it also kills the latent
            // stale-skip hole the tag scheme had at this scale.
            if !world.is_live_item(cid) {
                MS_STALE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                continue;
            }
            if n >= out_len {
                return refused(5);
            }
            buf.push(cid);
            n += 1;
        }
        ((*vt).SetLongArrayRegion)(raw, out, 0, n as i32, buf.as_ptr());
    }
    if n == 0 {
        let z = MS_ZERO.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if z % 2000 == 0 {
            eprintln!(
                "[ms] calls={} raw={} stale={} zero={} refused={{1:{},2:{},3:{},4:{},5:{}}}",
                MS_CALLS.load(std::sync::atomic::Ordering::Relaxed),
                MS_RAW.load(std::sync::atomic::Ordering::Relaxed),
                MS_STALE.load(std::sync::atomic::Ordering::Relaxed),
                z,
                MS_STATS[1].load(std::sync::atomic::Ordering::Relaxed),
                MS_STATS[2].load(std::sync::atomic::Ordering::Relaxed),
                MS_STATS[3].load(std::sync::atomic::Ordering::Relaxed),
                MS_STATS[4].load(std::sync::atomic::Ordering::Relaxed),
                MS_REFUSED.load(std::sync::atomic::Ordering::Relaxed),
            );
        }
    }
    MS_STATS[0].store(MS_CALLS.load(std::sync::atomic::Ordering::Relaxed), std::sync::atomic::Ordering::Relaxed);
    MS_STATS[1].store(MS_RAW.load(std::sync::atomic::Ordering::Relaxed), std::sync::atomic::Ordering::Relaxed);
    MS_STATS[2].store(MS_STALE.load(std::sync::atomic::Ordering::Relaxed), std::sync::atomic::Ordering::Relaxed);
    MS_STATS[3].store(MS_ZERO.load(std::sync::atomic::Ordering::Relaxed), std::sync::atomic::Ordering::Relaxed);
    MS_STATS[4].store(MS_REFUSED.load(std::sync::atomic::Ordering::Relaxed), std::sync::atomic::Ordering::Relaxed);
    n as i32
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
