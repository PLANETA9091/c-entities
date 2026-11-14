//! L6 — per-section collision cache.
//!
//! One entry per 16x16x16 chunk section: the block palette (collider box,
//! friction, speed factor, fluid flags, block-behavior override flags) plus
//! a per-block palette-index map. Populated lazily through a caller-provided
//! provider (the module bridges this to one JNI up-call per section, reading
//! the section's palette). Invalidated by the module's setBlockState probe.
//!
//! The gather replicates the moonrise block-collision iteration EXACTLY
//! (same minBlock/maxBlock formulas, same sectionAdjust=1 trim for
//! non-special sections, same 1e-7 epsilon intersect test). Sections whose
//! palette contains multi-cuboid or context-dependent collision shapes are
//! marked `complex` — the tick core routes items touching them to vanilla
//! instead of gathering.

use crate::itemtick::{mth_floor, voxel_shape_intersect, Aabb6};
use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, RwLock};

/// 4096 blocks per section, index = x | z << 4 | y << 8.
pub const SEC_BLOCKS: usize = 4096;
/// Palette index meaning "air / no data" (never used by a real palette slot).
pub const AIR_IDX: u16 = 0xFFFF;

/// One palette entry, resolved by the Java provider.
#[derive(Clone, Debug)]
pub struct PaletteEntry {
    /// Collision cuboids relative to the block position (0..=3; genuine
    /// multi-cuboid shapes — stairs, beds, ... — are carried natively).
    /// Context-dependent shape classes set `SectionData::complex` instead.
    pub colliders: Vec<Aabb6>,
    /// `Block.getFriction()` (f32).
    pub friction: f32,
    /// `Block.getSpeedFactor()` (f32).
    pub speed: f32,
    /// `state.is(Blocks.WATER) || state.is(Blocks.BUBBLE_COLUMN)` — the
    /// getBlockSpeedFactor branch.
    pub is_water: bool,
    /// `!state.getFluidState().isEmpty()` — fluid-presence routing.
    pub has_fluid: bool,
    /// WebBlock (cobweb): entityInside applies makeStuckInBlock natively.
    pub cobweb: bool,
    /// entityInside override with non-trivial effects (bush, fire, ...):
    /// items touching it route vanilla.
    pub entity_inside_special: bool,
    /// stepOn override (magma hurt, honey, ...): grounded items route
    /// vanilla (the exact effect runs in the Java body).
    pub step_on_special: bool,
    /// updateEntityMovementAfterFallOn override (slime/bed bounce): items
    /// whose collide touches one route vanilla.
    pub fall_on_special: bool,
}

impl PaletteEntry {
    pub fn air() -> PaletteEntry {
        PaletteEntry {
            colliders: Vec::new(),
            friction: 0.6,
            speed: 1.0,
            is_water: false,
            has_fluid: false,
            cobweb: false,
            entity_inside_special: false,
            step_on_special: false,
            fall_on_special: false,
        }
    }
}

/// Cached section data. `complex`/`any_fluid` are palette-wide ORs.
#[derive(Clone, Debug)]
pub struct SectionData {
    pub entries: Vec<PaletteEntry>,
    /// Per-block palette index, [x | z<<4 | y<<8]; AIR_IDX for empty slots.
    /// EMPTY VEC = all-air section: a sparse world's cache must not pay
    /// 8KB of blocks for sections that are 100% air (the 2400x2400-block
    /// fill cached 457k sections = 8GB of mostly-air blocks arrays).
    pub blocks: Vec<u16>,
    pub complex: bool,
    pub any_fluid: bool,
    /// Any palette entry has an effectful entityInside override (cheap
    /// gate for the checkInsideBlocks path scan).
    pub any_inside_special: bool,
}

impl SectionData {
    /// All-air section (fast path: every block is the AIR_IDX entry).
    pub fn all_air() -> SectionData {
        SectionData {
            entries: vec![PaletteEntry::air()],
            blocks: Vec::new(),
            complex: false,
            any_fluid: false,
            any_inside_special: false,
        }
    }

    #[inline]
    pub fn entry(&self, idx: u16) -> Option<&PaletteEntry> {
        if idx == AIR_IDX {
            return None;
        }
        self.entries.get(idx as usize)
    }
}

/// Pack a section key: (cx:24 | cz:24 | cy:16) — cy is small (world height),
/// cx/cz 24 bits suffice for ±8.3M sections.
///
/// L150f: cy is XOR-BIASED with 0x8000. The raw mask let sec_key(-1, -1, -1)
/// produce EXACTLY u64::MAX — the SectionCache's SC_EMPTY sentinel — so a
/// REAL section at chunk (-1, -1), section Y -1 matched the fresh-cache
/// "empty" way on the hit path and served a None unwrap (the mixed-150k
/// abort, sections.rs data(): the placement-dependent core dump). With the
/// bias the low 16 bits of a real key never reach 0xFFFF, so no key can
/// ever alias the sentinel again.
#[inline]
pub fn sec_key(cx: i32, cy: i32, cz: i32) -> u64 {
    ((((cx as i64) & 0xFF_FFFF) << 40)
        | (((cz as i64) & 0xFF_FFFF) << 16)
        | (((cy as i64) & 0xFFFF) ^ 0x8000)) as u64
}

/// Provider result: Ok(data) — populated; Err(()) — chunk absent.
/// `load=false` (gather): absent chunks gather nothing in vanilla — the
/// caller routes such items vanilla. `load=true` (getBlockState paths:
/// friction/speed below an item): vanilla loads the chunk synchronously, so
/// the provider may block on a load and must only fail for genuine errors.
pub type SectionProvider<'a> = &'a mut dyn FnMut(i32, i32, i32, bool) -> Result<SectionData, ()>;

/// Gather outcome flags.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GatherFlags {
    pub complex: bool,
    pub any_fluid: bool,
    /// true when the gather range included sections whose chunk is absent —
    /// callers MUST route such items vanilla (vanilla also gathers nothing
    /// there, so the vanilla path computes the exact same tick).
    pub unloaded: bool,
    /// A gathered entry has a fallOn override (slime/bed bounce) — exact
    /// collide exists but the post-collide delta handling differs; route
    /// vanilla.
    pub fall_special: bool,
}

/// Small per-block property copy for the hot lookups.
pub type BlockProps = (f32, f32, bool, bool, bool, bool, bool, bool);

// ---------------------------------------------------------------------------
// L8 — THREAD-SAFE SECTION CACHE.
//
// The flush is rayon-parallel (the store shards cleanly by id), so the
// section cache must serve concurrent gathers. Design:
//
//   * 16 shards, each an RwLock<HashMap<u64, Arc<SectionData>>>;
//   * READS take the shard read-lock only to clone an Arc (~8 bytes) —
//     the block scan itself runs LOCK-FREE on the Arc payload;
//   * a MISS runs the provider up-call (JNI) with NO lock held, then takes
//     the write-lock once to insert (double-check via the entry API, so a
//     concurrent hydrate of the same section wins cheaply);
//   * invalidation takes one write-lock — a block change invalidates one
//     section, never the whole grid.
//
// Determinism: a section's payload is immutable once published (block
// changes invalidate + re-publish a NEW Arc), so a gather never observes a
// torn or half-updated section. The up-call itself is thread-safe: the
// Java provider reads loaded chunk sections only (`getChunkNow`, no load)
// or attaches and loads synchronously for the `load=true` paths.
// ---------------------------------------------------------------------------

const GRID_SHARDS: usize = 16;

/// Minimal fast hasher (multiply-rotate; the map keys are already-mixed
/// sec_key values from `hash64`, so a heavy SipHash is pure tax).
#[derive(Clone, Default)]
struct ShardHasher(u64);

impl std::hash::Hasher for ShardHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
    #[inline]
    fn write(&mut self, _bytes: &[u8]) {
        // unreachable: the map hashes u64 keys only
        debug_assert!(false, "ShardHasher hashes u64 keys only");
    }
    #[inline]
    fn write_u64(&mut self, v: u64) {
        self.0 = crate::hash64(v);
    }
}

#[derive(Clone, Default)]
struct ShardBuild;

impl std::hash::BuildHasher for ShardBuild {
    type Hasher = ShardHasher;
    #[inline]
    fn build_hasher(&self) -> ShardHasher {
        ShardHasher(0)
    }
}

type ShardMap = std::collections::HashMap<u64, Arc<SectionData>, ShardBuild>;

/// Per-world section cache.
pub struct SectionGrid {
    /// L150e: the GLOBAL grid revision — bumped on EVERY mutation
    /// (invalidate / invalidate_all / a populate after a miss). The item
    /// kernel's probe memo validates against this one atomic load; a block
    /// change, a chunk (re)load or any section (re)population re-gathers
    /// exactly once and re-memos.
    pub rev: std::sync::atomic::AtomicU64,
    shards: Vec<RwLock<ShardMap>>,
    pub populated: AtomicU64,
    pub lookups: AtomicU64,
}

impl Default for SectionGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl SectionGrid {
    pub fn new() -> SectionGrid {
        SectionGrid {
            shards: (0..GRID_SHARDS)
                .map(|_| {
                    RwLock::new(std::collections::HashMap::with_capacity_and_hasher(
                        1 << 10,
                        ShardBuild,
                    ))
                })
                .collect(),
            populated: AtomicU64::new(0),
            lookups: AtomicU64::new(0),
            rev: std::sync::atomic::AtomicU64::new(0),
        }
    }

    #[inline]
    fn shard(&self, key: u64) -> &RwLock<ShardMap> {
        &self.shards[(key & (GRID_SHARDS as u64 - 1)) as usize]
    }

    pub fn invalidate_all(&self) {
        self.rev.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        for s in &self.shards {
            let mut m = s.write().unwrap_or_else(|e| e.into_inner());
            m.clear();
        }
    }

    pub fn invalidate(&self, cx: i32, cy: i32, cz: i32) {
        self.rev.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let key = sec_key(cx, cy, cz);
        let mut m = self.shard(key).write().unwrap_or_else(|e| e.into_inner());
        m.remove(&key);
    }

    /// L150e: the current grid revision for the probe-memo validity check.
    #[inline]
    pub fn rev(&self) -> u64 {
        self.rev.load(std::sync::atomic::Ordering::Relaxed)
    }

    #[inline]
    fn get_or_populate(
        &self,
        cx: i32,
        cy: i32,
        cz: i32,
        load: bool,
        provider: SectionProvider,
    ) -> Result<Arc<SectionData>, ()> {
        let key = sec_key(cx, cy, cz);
        let shard = self.shard(key);
        {
            let m = shard.read().unwrap_or_else(|e| e.into_inner());
            if let Some(d) = m.get(&key) {
                return Ok(Arc::clone(d));
            }
        }
        // Provider up-call with NO lock held (a JNI call must not sit on a
        // mutex other workers need).
        let data = provider(cx, cy, cz, load)?;
        let arc = Arc::new(data);
        let mut m = shard.write().unwrap_or_else(|e| e.into_inner());
        self.populated
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        // L150e: a (re)population is a grid mutation — the probe memos must
        // re-gather once against the new payload.
        self.rev.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        // Double-check insert: a concurrent worker may have published the
        // same section while the up-call ran — keep exactly one payload.
        Ok(Arc::clone(m.entry(key).or_insert_with(|| Arc::clone(&arc))))
    }

    /// Gather every single-cuboid block collider intersecting `box`, exactly
    /// like moonrise `getCollisionsForBlocksOrWorldBorder` for a
    /// non-special universe (collisionFlags = 4: border handled by the
    /// caller; no chunk loading).
    pub fn gather(
        &self,
        box_: &Aabb6,
        min_section_y: i32,
        max_section_y: i32,
        provider: SectionProvider,
        out: &mut Vec<Aabb6>,
    ) -> GatherFlags {
        let mut flags = GatherFlags::default();
        out.clear();
        // Vanilla formulas: the ±1e-7 slop then ±1 block expansion.
        let min_block_x = mth_floor(box_.x0 - 1.0e-7) - 1;
        let max_block_x = mth_floor(box_.x1 + 1.0e-7) + 1;
        let min_block_y = ((min_section_y << 4) - 1).max(mth_floor(box_.y0 - 1.0e-7) - 1);
        let max_block_y = ((max_section_y << 4) + 16).min(mth_floor(box_.y1 + 1.0e-7) + 1);
        let min_block_z = mth_floor(box_.z0 - 1.0e-7) - 1;
        let max_block_z = mth_floor(box_.z1 + 1.0e-7) + 1;
        if min_block_y > max_block_y {
            return flags;
        }
        let min_chunk_x = min_block_x >> 4;
        let max_chunk_x = max_block_x >> 4;
        let min_chunk_y = min_block_y >> 4;
        let max_chunk_y = max_block_y >> 4;
        let min_chunk_z = min_block_z >> 4;
        let max_chunk_z = max_block_z >> 4;
        for curr_chunk_z in min_chunk_z..=max_chunk_z {
            for curr_chunk_x in min_chunk_x..=max_chunk_x {
                for curr_chunk_y in min_chunk_y..=max_chunk_y {
                    let section_idx = curr_chunk_y - min_section_y;
                    if !(0..=(max_section_y - min_section_y)).contains(&section_idx) {
                        continue;
                    }
                    // sectionAdjust = 1: our sections are non-special by
                    // construction (special ones make items route vanilla).
                    let min_x_it = if curr_chunk_x == min_chunk_x {
                        (min_block_x & 15) + 1
                    } else {
                        0
                    };
                    let max_x_it = if curr_chunk_x == max_chunk_x {
                        (max_block_x & 15) - 1
                    } else {
                        15
                    };
                    let min_y_it = if curr_chunk_y == min_chunk_y {
                        (min_block_y & 15) + 1
                    } else {
                        0
                    };
                    let max_y_it = if curr_chunk_y == max_chunk_y {
                        (max_block_y & 15) - 1
                    } else {
                        15
                    };
                    let min_z_it = if curr_chunk_z == min_chunk_z {
                        (min_block_z & 15) + 1
                    } else {
                        0
                    };
                    let max_z_it = if curr_chunk_z == max_chunk_z {
                        (max_block_z & 15) - 1
                    } else {
                        15
                    };
                    if min_x_it > max_x_it || min_y_it > max_y_it || min_z_it > max_z_it {
                        continue;
                    }
                    self.lookups
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let data = match self.get_or_populate(
                        curr_chunk_x,
                        curr_chunk_y,
                        curr_chunk_z,
                        false,
                        provider,
                    ) {
                        Ok(d) => d,
                        Err(()) => {
                            flags.unloaded = true;
                            continue;
                        }
                    };
                    if data.complex {
                        flags.complex = true;
                        // Multi-cuboid shapes are not stored as boxes; the
                        // caller routes complex-flagged items vanilla.
                        continue;
                    }
                    if data.any_fluid {
                        flags.any_fluid = true;
                    }
                    for cy in min_y_it..=max_y_it {
                        for cz in min_z_it..=max_z_it {
                            for cx in min_x_it..=max_x_it {
                                let local = (cx | (cz << 4) | (cy << 8)) as usize;
                                let idx = data.blocks.get(local).copied().unwrap_or(AIR_IDX);
                                if idx == AIR_IDX {
                                    continue;
                                }
                                let Some(entry) = data.entries.get(idx as usize) else {
                                    continue;
                                };
                                if entry.fall_on_special {
                                    flags.fall_special = true;
                                }
                                let wx = (curr_chunk_x << 4) as f64 + cx as f64;
                                let wy = (curr_chunk_y << 4) as f64 + cy as f64;
                                let wz = (curr_chunk_z << 4) as f64 + cz as f64;
                                for rel in &entry.colliders {
                                    let world_box = Aabb6::new(
                                        rel.x0 + wx,
                                        rel.y0 + wy,
                                        rel.z0 + wz,
                                        rel.x1 + wx,
                                        rel.y1 + wy,
                                        rel.z1 + wz,
                                    );
                                    if voxel_shape_intersect(box_, &world_box) {
                                        out.push(world_box);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        flags
    }

    /// Block-state lookup used by friction / speed-factor / path-scan paths.
    /// Returns (props, unloaded) — unloaded chunks read as air, exactly like
    /// `getBlockState` semantics the caller replicates. Props = (friction,
    /// speed, is_water, has_fluid, cobweb, entity_inside_special,
    /// step_on_special, fall_on_special) — a small copy, no palette clone.
    #[allow(clippy::type_complexity)]
    pub fn block_at(
        &self,
        x: i32,
        y: i32,
        z: i32,
        min_section_y: i32,
        max_section_y: i32,
        provider: SectionProvider,
    ) -> (Option<BlockProps>, bool) {
        let cx = x >> 4;
        let cy = y >> 4;
        let cz = z >> 4;
        if cy < min_section_y || cy > max_section_y {
            return (None, false);
        }
        self.lookups
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        match self.get_or_populate(cx, cy, cz, true, provider) {
            Ok(data) => {
                let local = ((x & 15) | ((z & 15) << 4) | ((y & 15) << 8)) as usize;
                let idx = data.blocks.get(local).copied().unwrap_or(AIR_IDX);
                // copy only the small fields the callers need (no PaletteEntry
                // clone on the hot path)
                match data.entry(idx) {
                    Some(e) => (
                        Some((
                            e.friction,
                            e.speed,
                            e.is_water,
                            e.has_fluid,
                            e.cobweb,
                            e.entity_inside_special,
                            e.step_on_special,
                            e.fall_on_special,
                        )),
                        false,
                    ),
                    None => (None, false),
                }
            }
            Err(()) => (None, true),
        }
    }

    /// The collider of one block, for find_supporting_block (Aabb6 is Copy).
    pub fn block_box_at(
        &self,
        x: i32,
        y: i32,
        z: i32,
        min_section_y: i32,
        max_section_y: i32,
        provider: SectionProvider,
    ) -> (Option<Aabb6>, bool) {
        let cx = x >> 4;
        let cy = y >> 4;
        let cz = z >> 4;
        if cy < min_section_y || cy > max_section_y {
            return (None, false);
        }
        self.lookups
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        match self.get_or_populate(cx, cy, cz, true, provider) {
            Ok(data) => {
                let local = ((x & 15) | ((z & 15) << 4) | ((y & 15) << 8)) as usize;
                let idx = data.blocks.get(local).copied().unwrap_or(AIR_IDX);
                (
                    data.entry(idx).and_then(|e| e.colliders.first().copied()),
                    false,
                )
            }
            Err(()) => (None, true),
        }
    }
}

// ---------------------------------------------------------------------------
// L8.2 — WORKER-LOCAL SECTION CACHE.
//
// The shared sharded grid pays, per section visit: one RwLock acquire/
// release, a hash probe into a shared map, and an Arc refcount inc/dec —
// and at 100k-item scale the hottest sections (the farm floor) are touched
// by EVERY worker, so the Arc count and the shard locks are contended
// cache lines ping-ponging between cores (~200k visits per flush).
//
// `SectionCache` is the fix: a per-flush-chunk (per worker turn) read-
// through cache. Hits resolve in one LOCAL hash lookup and return a plain
// `&SectionData` — no lock, no shared map probe, no refcount traffic. A
// miss fills from the shared grid exactly once per section per worker.
//
// Epoch-correctness: the cache is created inside the flush native and
// dropped before it returns. While the native runs, NO invalidation can
// happen (the drain runs on the same server thread AFTER the native
// returns), so every local entry is fresh by construction.
// ---------------------------------------------------------------------------

/// Per-worker-turn read-through cache over a shared [`SectionGrid`].
///
/// L121: the backing store is a TWO-WAY SET-ASSOCIATIVE direct-mapped array,
/// not a HashMap. The item tick touches 4-6 sections per item per tick (the
/// noPhysics probe + the move-collide gather); at CI items-100k scale the
/// batch order walks the pile spatially, so a worker's 512 items revisit the
/// same ~100-110 sections constantly. The old HashMap paid TWO hash probes
/// (contains_key + get) plus pointer-chasing nodes per hit — the flush's
/// 606k looks/flush were ~80% probe tax (~200 ns/item, the JFR self-time of
/// the gather path). A direct-mapped hit is ONE multiply, ONE mask, ONE
/// sequential array load and a key compare (~4 ns); the 2-way buckets absorb
/// the key clustering (110 live sections over 256 sets). Exactness: the same
/// `SectionData` is served, misses still fill through `get_or_populate`.
pub struct SectionCache<'a> {
    grid: &'a SectionGrid,
    provider: SectionProvider<'a>,
    /// Two-way set associative: slots[2*set] and slots[2*set + 1], each
    /// (key, data) with key == 0 meaning EMPTY (sec_key packs cx<<40 |
    /// cz<<16 | cy with cx/cz masked to 24 bits — a zero key is a legal
    /// section (0,0,0), so EMPTY is encoded as u64::MAX, which sec_key can
    /// never produce: cx/cz are 24-bit (<<40 / <<16) and cy 16-bit, the
    /// top byte 0xFF is unreachable).
    slots: Vec<(u64, Option<Arc<SectionData>>)>,
    /// Section visits served locally (folded into the shared counter once
    /// per chunk instead of one contended atomic per visit).
    pub visits: u64,
}

impl<'a> SectionCache<'a> {
    /// L150e: the grid revision for the probe-memo validity check.
    #[inline]
    pub fn grid_rev(&self) -> u64 {
        self.grid.rev()
    }
}

/// 256 sets x 2 ways. 110 live sections at CI items-100k scale fit with
/// room for the 16x-wide spread legs; a hostile world with more live
/// sections than sets just degrades to the miss path (correct, slower).
const SC_SETS: usize = 256;
const SC_EMPTY: u64 = u64::MAX;

impl<'a> SectionCache<'a> {
    pub fn new(grid: &'a SectionGrid, provider: SectionProvider<'a>) -> SectionCache<'a> {
        SectionCache {
            grid,
            provider,
            slots: vec![(SC_EMPTY, None); SC_SETS * 2],
            visits: 0,
        }
    }

    #[inline]
    fn data(&mut self, cx: i32, cy: i32, cz: i32, load: bool) -> Result<&SectionData, ()> {
        let key = sec_key(cx, cy, cz);
        let set = ((key.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 41) as usize) & (SC_SETS - 1);
        let b0 = set * 2;
        // Resolve the way FIRST (no reference held), mutate only on the
        // miss path, and produce the served reference in the tail
        // expression — the borrow then ties to the &mut self lifetime the
        // same way the old HashMap get() did.
        let way = if self.slots[b0].0 == key {
            b0
        } else if self.slots[b0 + 1].0 == key {
            b0 + 1
        } else {
            let arc = self
                .grid
                .get_or_populate(cx, cy, cz, load, &mut *self.provider)?;
            // Fill the LRU-ish way: replace way 0 unless way 0 is newer
            // than way 1 (a 2-entry round-robin per set — good enough for
            // the section locality; misses are once-per-section-per-worker).
            let w = if self.slots[b0].0 == SC_EMPTY { b0 } else { b0 + 1 };
            self.slots[w] = (key, Some(arc));
            w
        };
        self.visits += 1;
        // Serve through the slot: the cache owns the Arc, the reference
        // borrows it for the gather's lifetime. A failed populate returned
        // Err above (the ?), so the data is Some() on every path here.
        Ok(self.slots[way].1.as_ref().unwrap())
    }

    /// Gather — same exactness contract as [`SectionGrid::gather`], served
    /// through the worker-local cache.
    pub fn gather(
        &mut self,
        box_: &Aabb6,
        min_section_y: i32,
        max_section_y: i32,
        out: &mut Vec<Aabb6>,
    ) -> GatherFlags {
        let mut flags = GatherFlags::default();
        out.clear();
        let min_block_x = mth_floor(box_.x0 - 1.0e-7) - 1;
        let max_block_x = mth_floor(box_.x1 + 1.0e-7) + 1;
        let min_block_y = ((min_section_y << 4) - 1).max(mth_floor(box_.y0 - 1.0e-7) - 1);
        let max_block_y = ((max_section_y << 4) + 16).min(mth_floor(box_.y1 + 1.0e-7) + 1);
        let min_block_z = mth_floor(box_.z0 - 1.0e-7) - 1;
        let max_block_z = mth_floor(box_.z1 + 1.0e-7) + 1;
        if min_block_y > max_block_y {
            return flags;
        }
        let min_chunk_x = min_block_x >> 4;
        let max_chunk_x = max_block_x >> 4;
        let min_chunk_y = min_block_y >> 4;
        let max_chunk_y = max_block_y >> 4;
        let min_chunk_z = min_block_z >> 4;
        let max_chunk_z = max_block_z >> 4;
        for curr_chunk_z in min_chunk_z..=max_chunk_z {
            for curr_chunk_x in min_chunk_x..=max_chunk_x {
                for curr_chunk_y in min_chunk_y..=max_chunk_y {
                    let section_idx = curr_chunk_y - min_section_y;
                    if !(0..=(max_section_y - min_section_y)).contains(&section_idx) {
                        continue;
                    }
                    let min_x_it = if curr_chunk_x == min_chunk_x {
                        (min_block_x & 15) + 1
                    } else {
                        0
                    };
                    let max_x_it = if curr_chunk_x == max_chunk_x {
                        (max_block_x & 15) - 1
                    } else {
                        15
                    };
                    let min_y_it = if curr_chunk_y == min_chunk_y {
                        (min_block_y & 15) + 1
                    } else {
                        0
                    };
                    let max_y_it = if curr_chunk_y == max_chunk_y {
                        (max_block_y & 15) - 1
                    } else {
                        15
                    };
                    let min_z_it = if curr_chunk_z == min_chunk_z {
                        (min_block_z & 15) + 1
                    } else {
                        0
                    };
                    let max_z_it = if curr_chunk_z == max_chunk_z {
                        (max_block_z & 15) - 1
                    } else {
                        15
                    };
                    if min_x_it > max_x_it || min_y_it > max_y_it || min_z_it > max_z_it {
                        continue;
                    }
                    self.visits += 1;
                    let data = match self.data(curr_chunk_x, curr_chunk_y, curr_chunk_z, false) {
                        Ok(d) => d,
                        Err(()) => {
                            flags.unloaded = true;
                            continue;
                        }
                    };
                    if data.complex {
                        flags.complex = true;
                        continue;
                    }
                    if data.any_fluid {
                        flags.any_fluid = true;
                    }
                    for cy in min_y_it..=max_y_it {
                        for cz in min_z_it..=max_z_it {
                            for cx in min_x_it..=max_x_it {
                                let local = (cx | (cz << 4) | (cy << 8)) as usize;
                                let idx = data.blocks.get(local).copied().unwrap_or(AIR_IDX);
                                if idx == AIR_IDX {
                                    continue;
                                }
                                let Some(entry) = data.entries.get(idx as usize) else {
                                    continue;
                                };
                                if entry.fall_on_special {
                                    flags.fall_special = true;
                                }
                                let wx = (curr_chunk_x << 4) as f64 + cx as f64;
                                let wy = (curr_chunk_y << 4) as f64 + cy as f64;
                                let wz = (curr_chunk_z << 4) as f64 + cz as f64;
                                for rel in &entry.colliders {
                                    let world_box = Aabb6::new(
                                        rel.x0 + wx,
                                        rel.y0 + wy,
                                        rel.z0 + wz,
                                        rel.x1 + wx,
                                        rel.y1 + wy,
                                        rel.z1 + wz,
                                    );
                                    if voxel_shape_intersect(box_, &world_box) {
                                        out.push(world_box);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        flags
    }

    /// Block lookup — same contract as [`SectionGrid::block_at`].
    #[allow(clippy::type_complexity)]
    pub fn block_at(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        min_section_y: i32,
        max_section_y: i32,
    ) -> (Option<BlockProps>, bool) {
        let cx = x >> 4;
        let cy = y >> 4;
        let cz = z >> 4;
        if cy < min_section_y || cy > max_section_y {
            return (None, false);
        }
        self.visits += 1;
        match self.data(cx, cy, cz, true) {
            Ok(data) => {
                let local = ((x & 15) | ((z & 15) << 4) | ((y & 15) << 8)) as usize;
                let idx = data.blocks.get(local).copied().unwrap_or(AIR_IDX);
                match data.entry(idx) {
                    Some(e) => (
                        Some((
                            e.friction,
                            e.speed,
                            e.is_water,
                            e.has_fluid,
                            e.cobweb,
                            e.entity_inside_special,
                            e.step_on_special,
                            e.fall_on_special,
                        )),
                        false,
                    ),
                    None => (None, false),
                }
            }
            Err(()) => (None, true),
        }
    }

    /// Supporting-block collider lookup — same contract as
    /// [`SectionGrid::block_box_at`].
    pub fn block_box_at(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        min_section_y: i32,
        max_section_y: i32,
    ) -> (Option<Aabb6>, bool) {
        let cx = x >> 4;
        let cy = y >> 4;
        let cz = z >> 4;
        if cy < min_section_y || cy > max_section_y {
            return (None, false);
        }
        self.visits += 1;
        match self.data(cx, cy, cz, true) {
            Ok(data) => {
                let local = ((x & 15) | ((z & 15) << 4) | ((y & 15) << 8)) as usize;
                let idx = data.blocks.get(local).copied().unwrap_or(AIR_IDX);
                (
                    data.entry(idx).and_then(|e| e.colliders.first().copied()),
                    false,
                )
            }
            Err(()) => (None, true),
        }
    }
}

#[cfg(test)]
mod sec_key_tests {
    use super::*;

    /// L150f: sec_key(-1, -1, -1) used to fold EXACTLY onto u64::MAX — the
    /// SectionCache's SC_EMPTY sentinel — so the REAL section at chunk
    /// (-1, -1) / section Y -1 matched the fresh-cache empty way and the
    /// hit path served a None unwrap (the mixed-150k abort). The cy bias
    /// must keep every real key away from the sentinel.
    #[test]
    fn sec_key_never_aliases_the_cache_sentinel() {
        assert_ne!(sec_key(-1, -1, -1), u64::MAX);
        assert_ne!(sec_key(0, 0, 0), u64::MAX);
        // the whole practical world range
        for cx in -4096..=4096 {
            for cz in -4096..=4096 {
                for cy in -24..=32 {
                    assert_ne!(sec_key(cx, cy, cz), u64::MAX, "cx={cx} cy={cy} cz={cz}");
                }
            }
        }
        // sec_key stays injective over a sanity window (the bias must not
        // fold distinct sections together)
        assert_ne!(sec_key(-1, -1, -1), sec_key(-1, 0, -1));
        assert_ne!(sec_key(-1, -1, -1), sec_key(0, -1, -1));
        assert_eq!(sec_key(-1, -1, -1), sec_key(-1, -1, -1));
    }
}
