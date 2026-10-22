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
    ((((cx as i64) & 0xFF_FFFF) << 40) | (((cz as i64) & 0xFF_FFFF) << 16) | ((cy as i64) & 0xFFFF)) as u64
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

/// Open-addressed map from section key to data slot (the CellMap pattern:
/// multiply-shift hash, linear probing, backward-shift deletion — no SipHash
/// on the L6 hot path).
struct SecMap {
    // u64::MAX = empty. sec_key(0,0,0) == 0 (the spawn sections!) — a 0
    // sentinel made that section UNCACHEABLE: every find() saw an empty
    // slot, every gather re-called the provider and inserted again, and
    // get_or_populate pushed a fresh ~18KB data slot per lookup (the 100k
    // fill burned ~34k slots/tick = the 8GB RSS explosion). Real packed
    // keys never reach u64::MAX (all three fields maxed simultaneously is
    // impossible for legal section coords).
    keys: Vec<u64>,
    vals: Vec<u32>, // slot+1
    mask: u64,
    len: usize,
}

#[inline]
fn sec_empty() -> u64 {
    u64::MAX
}

impl SecMap {
    fn new(cap_pow2: usize) -> Self {
        let n = cap_pow2.max(16);
        SecMap { keys: vec![sec_empty(); n], vals: vec![0; n], mask: (n - 1) as u64, len: 0 }
    }

    #[inline]
    fn find(&self, key: u64) -> Option<u32> {
        let mut i = (crate::hash64(key) & self.mask) as usize;
        loop {
            let k = self.keys[i];
            if k == sec_empty() {
                return None;
            }
            if k == key {
                return Some(self.vals[i] - 1);
            }
            i = (i + 1) & self.mask as usize;
        }
    }

    fn insert(&mut self, key: u64, slot: u32) {
        if (self.len + 1) * 10 >= self.keys.len() * 7 {
            self.grow();
        }
        let mut i = (crate::hash64(key) & self.mask) as usize;
        loop {
            let k = self.keys[i];
            if k == sec_empty() {
                self.keys[i] = key;
                self.vals[i] = slot + 1;
                self.len += 1;
                return;
            }
            if k == key {
                self.vals[i] = slot + 1;
                return;
            }
            i = (i + 1) & self.mask as usize;
        }
    }

    fn remove(&mut self, key: u64) {
        let mut i = (crate::hash64(key) & self.mask) as usize;
        loop {
            let k = self.keys[i];
            if k == 0 {
                return;
            }
            if k == key {
                let n = self.keys.len();
                let mut hole = i;
                let mut j = (i + 1) & (n - 1);
                while self.keys[j] != sec_empty() {
                    let ideal = (crate::hash64(self.keys[j]) & self.mask) as usize;
                    let d = (j.wrapping_sub(ideal)) & (n - 1);
                    let h = (hole.wrapping_sub(ideal)) & (n - 1);
                    if h < d {
                        self.keys[hole] = self.keys[j];
                        self.vals[hole] = self.vals[j];
                        hole = j;
                    }
                    j = (j + 1) & (n - 1);
                }
                self.keys[hole] = sec_empty();
                self.vals[hole] = 0;
                self.len -= 1;
                return;
            }
            i = (i + 1) & self.mask as usize;
        }
    }

    fn grow(&mut self) {
        let n = self.keys.len() * 2;
        let old_keys = std::mem::replace(&mut self.keys, vec![sec_empty(); n]);
        let old_vals = std::mem::replace(&mut self.vals, vec![0; n]);
        self.mask = (n - 1) as u64;
        self.len = 0;
        for i in 0..old_keys.len() {
            if old_keys[i] != sec_empty() {
                self.insert(old_keys[i], old_vals[i] - 1);
            }
        }
    }
}

/// Small per-block property copy for the hot lookups.
pub type BlockProps = (f32, f32, bool, bool, bool, bool, bool, bool);

/// Per-world section cache.
pub struct SectionGrid {
    map: SecMap,
    data: Vec<SectionData>, // slot 0 unused
    free: Vec<u32>,
    pub populated: u64,
    pub lookups: u64,
}

impl SectionGrid {
    pub fn new() -> SectionGrid {
        SectionGrid {
            map: SecMap::new(1 << 10),
            data: vec![SectionData::all_air()],
            free: Vec::new(),
            populated: 0,
            lookups: 0,
        }
    }

    pub fn invalidate_all(&mut self) {
        self.map = SecMap::new(1 << 10);
        self.data.truncate(1);
        self.free.clear();
    }

    pub fn invalidate(&mut self, cx: i32, cy: i32, cz: i32) {
        if let Some(slot) = self.map.find(sec_key(cx, cy, cz)) {
            self.data[slot as usize] = SectionData::all_air(); // keep capacity
            self.map.remove(sec_key(cx, cy, cz));
            self.free.push(slot);
        }
    }

    #[inline]
    fn get_or_populate(&mut self, cx: i32, cy: i32, cz: i32, load: bool, provider: SectionProvider) -> Result<usize, ()> {
        let key = sec_key(cx, cy, cz);
        if let Some(slot) = self.map.find(key) {
            return Ok(slot as usize);
        }
        match provider(cx, cy, cz, load) {
            Ok(data) => {
                self.populated += 1;
                let slot = match self.free.pop() {
                    Some(s) => {
                        self.data[s as usize] = data;
                        s as usize
                    }
                    None => {
                        self.data.push(data);
                        self.data.len() - 1
                    }
                };
                self.map.insert(key, slot as u32);
                Ok(slot)
            }
            Err(()) => {
                // Cache absence for THIS query only; do not poison the map.
                Err(())
            }
        }
    }

    /// Gather every single-cuboid block collider intersecting `box`, exactly
    /// like moonrise `getCollisionsForBlocksOrWorldBorder` for a
    /// non-special universe (collisionFlags = 4: border handled by the
    /// caller; no chunk loading).
    pub fn gather(
        &mut self,
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
                    let min_x_it = if curr_chunk_x == min_chunk_x { (min_block_x & 15) + 1 } else { 0 };
                    let max_x_it = if curr_chunk_x == max_chunk_x { (max_block_x & 15) - 1 } else { 15 };
                    let min_y_it = if curr_chunk_y == min_chunk_y { (min_block_y & 15) + 1 } else { 0 };
                    let max_y_it = if curr_chunk_y == max_chunk_y { (max_block_y & 15) - 1 } else { 15 };
                    let min_z_it = if curr_chunk_z == min_chunk_z { (min_block_z & 15) + 1 } else { 0 };
                    let max_z_it = if curr_chunk_z == max_chunk_z { (max_block_z & 15) - 1 } else { 15 };
                    if min_x_it > max_x_it || min_y_it > max_y_it || min_z_it > max_z_it {
                        continue;
                    }
                    self.lookups += 1;
                    let slot = match self.get_or_populate(curr_chunk_x, curr_chunk_y, curr_chunk_z, false, provider) {
                        Ok(s) => s,
                        Err(()) => {
                            flags.unloaded = true;
                            continue;
                        }
                    };
                    let data = &self.data[slot];
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
                                let Some(entry) = data.entries.get(idx as usize) else { continue };
                                if entry.fall_on_special {
                                    flags.fall_special = true;
                                }
                                let Some(rel) = entry.collider else { continue };
                                let wx = (curr_chunk_x << 4) as f64 + cx as f64;
                                let wy = (curr_chunk_y << 4) as f64 + cy as f64;
                                let wz = (curr_chunk_z << 4) as f64 + cz as f64;
                                let world_box = Aabb6::new(rel.x0 + wx, rel.y0 + wy, rel.z0 + wz, rel.x1 + wx, rel.y1 + wy, rel.z1 + wz);
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
    pub fn block_at(&mut self, x: i32, y: i32, z: i32, min_section_y: i32, max_section_y: i32, provider: SectionProvider) -> (Option<BlockProps>, bool) {
        let cx = x >> 4;
        let cy = y >> 4;
        let cz = z >> 4;
        if cy < min_section_y || cy > max_section_y {
            return (None, false);
        }
        self.lookups += 1;
        match self.get_or_populate(cx, cy, cz, true, provider) {
            Ok(slot) => {
                let data = &self.data[slot];
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
    pub fn block_box_at(&mut self, x: i32, y: i32, z: i32, min_section_y: i32, max_section_y: i32, provider: SectionProvider) -> (Option<Aabb6>, bool) {
        let cx = x >> 4;
        let cy = y >> 4;
        let cz = z >> 4;
        if cy < min_section_y || cy > max_section_y {
            return (None, false);
        }
        self.lookups += 1;
        match self.get_or_populate(cx, cy, cz, true, provider) {
            Ok(slot) => {
                let data = &self.data[slot];
                let local = ((x & 15) | ((z & 15) << 4) | ((y & 15) << 8)) as usize;
                let idx = data.blocks.get(local).copied().unwrap_or(AIR_IDX);
                (data.entry(idx).and_then(|e| e.collider), false)
            }
            Err(()) => (None, true),
        }
    }
}
