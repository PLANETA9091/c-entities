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
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, RwLock};

/// 4096 blocks per section, index = x | z << 4 | y << 8.
pub const SEC_BLOCKS: usize = 4096;
/// Palette index meaning "air / no data" (never used by a real palette slot).
pub const AIR_IDX: u16 = 0xFFFF;

/// One palette entry, resolved by the Java provider.
#[derive(Clone, Debug)]
pub struct PaletteEntry {
    /// Single-cuboid collision shape relative to the block position
    /// (None = empty collision shape). Multi-cuboid shapes set
    /// `SectionData::complex` instead.
    pub collider: Option<Aabb6>,
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
            collider: None,
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
#[inline]
pub fn sec_key(cx: i32, cy: i32, cz: i32) -> u64 {
    ((((cx as i64) & 0xFF_FFFF) << 40) | (((cz as i64) & 0xFF_FFFF) << 16) | ((cy as i64) & 0xFFFF))
        as u64
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
        }
    }

    #[inline]
    fn shard(&self, key: u64) -> &RwLock<ShardMap> {
        &self.shards[(key & (GRID_SHARDS as u64 - 1)) as usize]
    }

    pub fn invalidate_all(&self) {
        for s in &self.shards {
            let mut m = s.write().unwrap_or_else(|e| e.into_inner());
            m.clear();
        }
    }

    pub fn invalidate(&self, cx: i32, cy: i32, cz: i32) {
        let key = sec_key(cx, cy, cz);
        let mut m = self.shard(key).write().unwrap_or_else(|e| e.into_inner());
        m.remove(&key);
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
                                let Some(rel) = entry.collider else { continue };
                                let wx = (curr_chunk_x << 4) as f64 + cx as f64;
                                let wy = (curr_chunk_y << 4) as f64 + cy as f64;
                                let wz = (curr_chunk_z << 4) as f64 + cz as f64;
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
                (data.entry(idx).and_then(|e| e.collider), false)
            }
            Err(()) => (None, true),
        }
    }
}
