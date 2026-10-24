//! c-entities core: exact entity→entity proximity index (v0.2).
//!
//! Design contract (docs/research/05-module-design.md):
//! * The index may return **false positives** (the Java shim re-checks the
//!   exact f64 AABB before applying the vanilla predicate) but must never
//!   return a **false negative**: every entity whose true f64 AABB overlaps
//!   the query box is always offered.
//! * Single-homing: each normal entity lives in exactly one 16-block XZ
//!   cell (the cell of its AABB center). Queries expand by the world's
//!   tracked max half-span so no center can be missed. Entities wider than
//!   one cell go to a tiny oversize list scanned linearly.
//! * Coordinates are stored as f32 **offsets from the home cell origin**
//!   (range [-8, 24)), bounding quantization error to ~9.5e-7 — far below
//!   the query margin (1e-3). Global f32 would not be exact (ULP ≈ 2 blocks
//!   at the world edge); cell-relative f32 is exact for ±30M block worlds.
//! * Steady-state query: contiguous per-cell arrays (array-of-structs,
//!   32 B/entry) — one cache stream per cell, no pointer chasing.
//! * No allocation on the steady-state query path (caller-provided buffer).
//! * No external dependencies, no std HashMap on hot paths, no unsafe.

#![forbid(unsafe_code)]

pub const CELL: i32 = 16;
pub const CELL_SHIFT: u32 = 4; // 16 == 1 << 4
/// Query margin in blocks. Bounds the combined f32 rounding stack:
/// cell-relative quantization (≤9.5e-7 XZ), absolute-Y quantization
/// (≤1.5e-5 at build height), and f32 casts of the test box (≤3e-5) —
/// 1e-4 keeps >3x headroom on the worst axis while making boundary
/// false positives (entities within MARGIN of the box) vanishingly rare.
pub const MARGIN: f32 = 1.0e-4;
/// Entities whose AABB half-span exceeds this live in the oversize list.
pub const MAX_HALF_SPAN: f64 = CELL as f64 / 2.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    pub x0: f64,
    pub y0: f64,
    pub z0: f64,
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
}

impl Aabb {
    #[inline]
    pub fn new(x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64) -> Self {
        Aabb { x0, y0, z0, x1, y1, z1 }
    }

    /// Vanilla `AABB.intersects` semantics: strict inequalities.
    #[inline]
    pub fn intersects(&self, o: &Aabb) -> bool {
        self.x0 < o.x1 && self.x1 > o.x0 && self.y0 < o.y1 && self.y1 > o.y0 && self.z0 < o.z1 && self.z1 > o.z0
    }

    #[inline]
    pub fn center_x(&self) -> f64 {
        (self.x0 + self.x1) * 0.5
    }

    #[inline]
    pub fn center_z(&self) -> f64 {
        (self.z0 + self.z1) * 0.5
    }
}

#[inline]
fn hash64(mut x: u64) -> u64 {
    // splitmix64 finalizer: good avalanche, constant-time, dependency-free
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

/// world:16 | cx:22 (biased) | cz:22 (biased); never zero.
///
/// The 22-bit biased fields cover cell coords in [-2^20, 2^20) — i.e. block
/// coords within ±134M, far beyond vanilla's ±30M world border. Both biased
/// components are nonzero for any reachable cell, so the composed key can
/// never be 0 (0 is the empty-slot sentinel of the open-addressed tables).
#[inline]
fn cell_key(world: u16, cx: i32, cz: i32) -> u64 {
    const B: i64 = 1 << 20;
    ((world as u64) << 44)
        | (((cx as i64 + B) as u64 & 0x3F_FFFF) << 22)
        | ((cz as i64 + B) as u64 & 0x3F_FFFF)
}

// ---------------------------------------------------------------------------
// Open-addressed maps (linear probing, backward-shift deletion, no unsafe)
// ---------------------------------------------------------------------------

/// Open-addressed u64 -> u32 map (hidden: test surface only).
#[doc(hidden)]
pub struct CellMap {
    pub keys: Vec<u64>,
    pub vals: Vec<u32>, // cell id (never stored as 0: cells are 1-based)
    mask: u64,
    len: usize,
}

impl CellMap {
    pub fn new(cap_pow2: usize) -> Self {
        let n = cap_pow2.max(16);
        CellMap { keys: vec![0; n], vals: vec![0; n], mask: (n - 1) as u64, len: 0 }
    }

    #[inline]
    pub fn find(&self, key: u64) -> Option<u32> {
        let mut i = (hash64(key) & self.mask) as usize;
        loop {
            let k = self.keys[i];
            if k == 0 {
                return None;
            }
            if k == key {
                return Some(self.vals[i]);
            }
            i = (i + 1) & self.mask as usize;
        }
    }

    pub fn insert(&mut self, key: u64, val: u32) {
        debug_assert_ne!(val, 0, "cell ids are 1-based");
        if (self.len + 1) * 10 >= self.keys.len() * 7 {
            self.grow();
        }
        let mut i = (hash64(key) & self.mask) as usize;
        loop {
            let k = self.keys[i];
            if k == 0 {
                self.keys[i] = key;
                self.vals[i] = val;
                self.len += 1;
                return;
            }
            if k == key {
                self.vals[i] = val;
                return;
            }
            i = (i + 1) & self.mask as usize;
        }
    }

    pub fn remove(&mut self, key: u64) {
        let mut i = (hash64(key) & self.mask) as usize;
        loop {
            let k = self.keys[i];
            if k == 0 {
                return;
            }
            if k == key {
                let n = self.keys.len();
                let mut hole = i;
                let mut j = (i + 1) & (n - 1);
                while self.keys[j] != 0 {
                    let ideal = (hash64(self.keys[j]) & self.mask) as usize;
                    let d = (j.wrapping_sub(ideal)) & (n - 1);
                    let h = (hole.wrapping_sub(ideal)) & (n - 1);
                    if h < d {
                        self.keys[hole] = self.keys[j];
                        self.vals[hole] = self.vals[j];
                        hole = j;
                    }
                    j = (j + 1) & (n - 1);
                }
                self.keys[hole] = 0;
                self.vals[hole] = 0;
                self.len -= 1;
                return;
            }
            i = (i + 1) & self.mask as usize;
        }
    }

    fn grow(&mut self) {
        let n = self.keys.len() * 2;
        let old_keys = std::mem::replace(&mut self.keys, vec![0; n]);
        let old_vals = std::mem::replace(&mut self.vals, vec![0; n]);
        self.mask = (n - 1) as u64;
        self.len = 0;
        for i in 0..old_keys.len() {
            if old_keys[i] != 0 {
                self.insert(old_keys[i], old_vals[i]);
            }
        }
    }
}

/// Open-addressed i64 -> u64 map (id -> (cell_id << 32) | idx, 0 = empty).
struct IdMap {
    keys: Vec<i64>, // 0 = empty; vanilla entity ids are never 0
    vals: Vec<u64>, // packed (cell_id, idx); 0 = empty
    mask: u64,
    len: usize,
}

impl IdMap {
    fn new(cap_pow2: usize) -> Self {
        let n = cap_pow2.max(16);
        IdMap { keys: vec![0; n], vals: vec![0; n], mask: (n - 1) as u64, len: 0 }
    }

    #[inline]
    fn find(&self, key: i64) -> Option<u64> {
        let mut i = (hash64(key as u64) & self.mask) as usize;
        loop {
            let k = self.keys[i];
            if k == 0 {
                return None;
            }
            if k == key {
                return Some(self.vals[i]);
            }
            i = (i + 1) & self.mask as usize;
        }
    }

    fn insert(&mut self, key: i64, val: u64) {
        debug_assert_ne!(val, 0);
        if (self.len + 1) * 10 >= self.keys.len() * 7 {
            self.grow();
        }
        let mut i = (hash64(key as u64) & self.mask) as usize;
        loop {
            let k = self.keys[i];
            if k == 0 {
                self.keys[i] = key;
                self.vals[i] = val;
                self.len += 1;
                return;
            }
            if k == key {
                self.vals[i] = val;
                return;
            }
            i = (i + 1) & self.mask as usize;
        }
    }

    fn remove(&mut self, key: i64) {
        let mut i = (hash64(key as u64) & self.mask) as usize;
        loop {
            let k = self.keys[i];
            if k == 0 {
                return;
            }
            if k == key {
                let n = self.keys.len();
                let mut hole = i;
                let mut j = (i + 1) & (n - 1);
                while self.keys[j] != 0 {
                    let ideal = (hash64(self.keys[j] as u64) & self.mask) as usize;
                    let d = (j.wrapping_sub(ideal)) & (n - 1);
                    let h = (hole.wrapping_sub(ideal)) & (n - 1);
                    if h < d {
                        self.keys[hole] = self.keys[j];
                        self.vals[hole] = self.vals[j];
                        hole = j;
                    }
                    j = (j + 1) & (n - 1);
                }
                self.keys[hole] = 0;
                self.vals[hole] = 0;
                self.len -= 1;
                return;
            }
            i = (i + 1) & self.mask as usize;
        }
    }

    fn grow(&mut self) {
        let n = self.keys.len() * 2;
        let old_keys = std::mem::replace(&mut self.keys, vec![0; n]);
        let old_vals = std::mem::replace(&mut self.vals, vec![0; n]);
        self.mask = (n - 1) as u64;
        self.len = 0;
        for i in 0..old_keys.len() {
            if old_keys[i] != 0 {
                self.insert(old_keys[i], old_vals[i]);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Index
// ---------------------------------------------------------------------------

/// Packed entity record inside a cell (array-of-structs, 32 B): one cache
/// stream per cell during the query walk.
#[derive(Clone, Copy)]
struct CellEnt {
    id: i64,
    // cell-relative XZ (f32, range [-8, 24)), absolute Y (f32, world height)
    x0: f32,
    y0: f32,
    z0: f32,
    x1: f32,
    y1: f32,
    z1: f32,
}

#[derive(Clone)]
struct CellData {
    key: u64,
    ents: Vec<CellEnt>,
}

/// Oversize entity (AABB wider than one cell): flat list, linear scan.
/// Vanilla oversize entities are rare (ender dragon class), so this stays tiny.
struct Oversize {
    id: i64,
    world: u16,
    b: Aabb, // exact f64, no quantization concerns
}

#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub entities: usize,
    pub cells: usize,
    pub oversize: usize,
}

pub struct EntityIndex {
    cells: Vec<CellData>, // cell id (1-based) -> data
    free_cells: Vec<u32>,
    cellmap: CellMap, // cell key -> cell id
    idmap: IdMap,     // entity id -> (cell_id << 32) | idx
    oversize: Vec<Oversize>,
    /// Per-world tracked max half-span (removal keeps it: safe direction).
    world_half_span: Vec<f64>,
    stats: Stats,
}

impl EntityIndex {
    pub fn new() -> Self {
        EntityIndex {
            cells: vec![CellData { key: 0, ents: Vec::new() }], // cell 0 unused
            free_cells: Vec::new(),
            cellmap: CellMap::new(1 << 13),
            idmap: IdMap::new(1 << 13),
            oversize: Vec::new(),
            world_half_span: vec![1.0],
            stats: Stats::default(),
        }
    }

    pub fn stats(&self) -> Stats {
        self.stats.clone()
    }

    /// Tracked max half-span of `world` (diagnostic; tests assert the bound).
    pub fn max_half_span(&self, world: u16) -> f64 {
        self.world_half_span
            .get(world as usize)
            .copied()
            .unwrap_or(1.0)
    }

    /// Insert or update an entity's AABB. O(1) typical (same-cell update is
    /// 6 stores). `world` is a caller-assigned small id per Level instance.
    pub fn upsert(&mut self, id: i64, world: u16, b: &Aabb) {
        let hx = (b.x1 - b.x0) * 0.5;
        let hz = (b.z1 - b.z0) * 0.5;
        let half = hx.max(hz);

        if half > MAX_HALF_SPAN {
            self.upsert_oversize(id, world, *b);
            return;
        }
        if half > self.max_half_span(world) {
            let wi = world as usize;
            if wi >= self.world_half_span.len() {
                self.world_half_span.resize(wi + 1, 1.0);
            }
            self.world_half_span[wi] = half;
        }

        let ncx = (b.center_x().floor() as i32) >> CELL_SHIFT;
        let ncz = (b.center_z().floor() as i32) >> CELL_SHIFT;
        let nkey = cell_key(world, ncx, ncz);

        if let Some(packed) = self.idmap.find(id) {
            let cell_id = (packed >> 32) as u32;
            let idx = (packed & 0xFFFF_FFFF) as usize;
            if self.cells[cell_id as usize].key == nkey {
                // same cell: bounds update only
                let (ox, oz) = cell_origin(ncx, ncz);
                let e = &mut self.cells[cell_id as usize].ents[idx];
                e.x0 = (b.x0 - ox) as f32;
                e.y0 = b.y0 as f32;
                e.z0 = (b.z0 - oz) as f32;
                e.x1 = (b.x1 - ox) as f32;
                e.y1 = b.y1 as f32;
                e.z1 = (b.z1 - oz) as f32;
                return;
            }
            // moved cell (or world): swap-remove from the old cell, insert new
            self.remove_at(cell_id, idx, id);
        } else {
            // if it used to be oversize, drop that record
            if let Some(p) = self.oversize.iter().position(|o| o.id == id) {
                self.oversize.swap_remove(p);
                self.stats.oversize -= 1;
            }
            self.stats.entities += 1;
        }

        // insert into the (possibly new) cell
        let cell_id = self.cell_id(world, ncx, ncz);
        let (ox, oz) = cell_origin(ncx, ncz);
        let ent = CellEnt {
            id,
            x0: (b.x0 - ox) as f32,
            y0: b.y0 as f32,
            z0: (b.z0 - oz) as f32,
            x1: (b.x1 - ox) as f32,
            y1: b.y1 as f32,
            z1: (b.z1 - oz) as f32,
        };
        let cd = &mut self.cells[cell_id as usize];
        let idx = cd.ents.len();
        cd.ents.push(ent);
        self.idmap.insert(id, ((cell_id as u64) << 32) | idx as u64);
    }

    pub fn remove(&mut self, id: i64) {
        if let Some(packed) = self.idmap.find(id) {
            let cell_id = (packed >> 32) as u32;
            let idx = (packed & 0xFFFF_FFFF) as usize;
            self.remove_at(cell_id, idx, id);
            self.stats.entities -= 1;
        } else if let Some(p) = self.oversize.iter().position(|o| o.id == id) {
            self.oversize.swap_remove(p);
            self.stats.oversize -= 1;
        }
    }

    /// Drop every entity belonging to `world` (used on Level unload).
    pub fn clear_world(&mut self, world: u16) {
        let mut dead = Vec::new();
        for cd in self.cells.iter().skip(1) {
            if (cd.key >> 44) as u16 == world {
                for e in &cd.ents {
                    dead.push(e.id);
                }
            }
        }
        for id in dead {
            self.remove(id);
        }
        let mut p = 0;
        while p < self.oversize.len() {
            if self.oversize[p].world == world {
                self.oversize.swap_remove(p);
                self.stats.oversize -= 1;
            } else {
                p += 1;
            }
        }
        let wi = world as usize;
        if wi < self.world_half_span.len() {
            self.world_half_span[wi] = 1.0;
        }
    }

    /// Drop everything (module reset / kill switch).
    pub fn clear(&mut self) {
        self.cells.truncate(1);
        self.cells[0] = CellData { key: 0, ents: Vec::new() };
        self.free_cells.clear();
        self.cellmap = CellMap::new(1 << 13);
        self.idmap = IdMap::new(1 << 13);
        self.oversize.clear();
        self.world_half_span.clear();
        self.world_half_span.push(1.0);
        self.stats = Stats::default();
    }

    /// Cheap cost estimate for `query`: total entity population of the cells
    /// the query would touch (O(cells touched) hash lookups, no entry reads).
    /// Used to skip the native walk entirely for huge dense queries.
    pub fn query_population(&self, world: u16, b: &Aabb) -> usize {
        let ex = self.max_half_span(world) + MARGIN as f64;
        let cx0 = ((b.x0 - ex).floor() as i32) >> CELL_SHIFT;
        let cx1 = ((b.x1 + ex).floor() as i32) >> CELL_SHIFT;
        let cz0 = ((b.z0 - ex).floor() as i32) >> CELL_SHIFT;
        let cz1 = ((b.z1 + ex).floor() as i32) >> CELL_SHIFT;
        let mut n = 0usize;
        for cx in cx0..=cx1 {
            for cz in cz0..=cz1 {
                if let Some(head) = self.cellmap.find(cell_key(world, cx, cz)) {
                    n += self.cells[head as usize].ents.len();
                }
            }
        }
        n + self.oversize.len()
    }

    /// Exact candidate query (see module contract). Appends ids to `out`
    /// (cleared first) and returns the number of entries WALKED (cells
    /// scanned), for the bridge's fast-vs-mirror cost heuristic.
    pub fn query(&self, world: u16, b: &Aabb, out: &mut Vec<i64>) -> usize {
        out.clear();
        let ex = self.max_half_span(world) + MARGIN as f64;
        // centers of intersecting entities lie within the box expanded by ex
        let cx0 = ((b.x0 - ex).floor() as i32) >> CELL_SHIFT;
        let cx1 = ((b.x1 + ex).floor() as i32) >> CELL_SHIFT;
        let cz0 = ((b.z0 - ex).floor() as i32) >> CELL_SHIFT;
        let cz1 = ((b.z1 + ex).floor() as i32) >> CELL_SHIFT;

        // conservative test box: stored box widened by MARGIN on both sides
        let tx0 = b.x0 - MARGIN as f64;
        let tx1 = b.x1 + MARGIN as f64;
        let ty0 = b.y0 - MARGIN as f64;
        let ty1 = b.y1 + MARGIN as f64;
        let tz0 = b.z0 - MARGIN as f64;
        let tz1 = b.z1 + MARGIN as f64;

        let mut visited = 0usize;
        for cx in cx0..=cx1 {
            for cz in cz0..=cz1 {
                let key = cell_key(world, cx, cz);
                let head = match self.cellmap.find(key) {
                    Some(v) => v,
                    None => continue,
                };
                let (ox, oz) = cell_origin(cx, cz);
                let ox = ox as f32;
                let oz = oz as f32;
                let ents = &self.cells[head as usize].ents;
                visited += ents.len();
                // conservative test box in cell-relative f32: comparing
                // (e - origin) against (b ∓ origin) is equivalent to
                // comparing e against b but saves two f64 adds per coord.
                // Casting rounds by ≤1 ulp (≈1e-6 XZ, ≈3e-5 Y) — both are
                // far below MARGIN, so no intersection can be missed.
                let bx0 = (tx0 - ox as f64) as f32;
                let bx1 = (tx1 - ox as f64) as f32;
                let by0 = ty0 as f32;
                let by1 = ty1 as f32;
                let bz0 = (tz0 - oz as f64) as f32;
                let bz1 = (tz1 - oz as f64) as f32;
                for e in ents {
                    if e.x0 < bx1 && e.x1 > bx0 && e.y0 < by1 && e.y1 > by0
                        && e.z0 < bz1 && e.z1 > bz0
                    {
                        out.push(e.id);
                    }
                }
            }
        }
        for o in &self.oversize {
            if o.world == world && o.b.intersects(b) {
                out.push(o.id);
            }
        }
        visited
    }

    // ---- internals ----

    fn cell_id(&mut self, world: u16, cx: i32, cz: i32) -> u32 {
        let key = cell_key(world, cx, cz);
        if let Some(v) = self.cellmap.find(key) {
            return v;
        }
        let id = match self.free_cells.pop() {
            Some(i) => {
                self.cells[i as usize].key = key;
                i
            }
            None => {
                self.cells.push(CellData { key, ents: Vec::new() });
                (self.cells.len() - 1) as u32
            }
        };
        self.cellmap.insert(key, id);
        self.stats.cells += 1;
        id
    }

    /// Swap-remove entry `idx` (entity `removed_id`) from cell `cell_id`,
    /// fixing the moved entry's idmap record and dropping the removed id's
    /// mapping. Empty cells are freed.
    fn remove_at(&mut self, cell_id: u32, idx: usize, removed_id: i64) {
        let moved_id;
        {
            let cd = &mut self.cells[cell_id as usize];
            let last = cd.ents.len() - 1;
            if idx != last {
                moved_id = cd.ents[last].id;
                cd.ents.swap(idx, last);
            } else {
                moved_id = 0;
            }
            cd.ents.pop();
        }
        if moved_id != 0 {
            self.idmap.insert(moved_id, ((cell_id as u64) << 32) | idx as u64);
        }
        self.idmap.remove(removed_id);
        if self.cells[cell_id as usize].ents.is_empty() {
            let key = self.cells[cell_id as usize].key;
            self.cellmap.remove(key);
            self.cells[cell_id as usize].key = 0;
            self.free_cells.push(cell_id);
            self.stats.cells -= 1;
        }
    }

    fn upsert_oversize(&mut self, id: i64, world: u16, b: Aabb) {
        if let Some(p) = self.oversize.iter().position(|o| o.id == id) {
            self.oversize[p] = Oversize { id, world, b };
            return;
        }
        if let Some(packed) = self.idmap.find(id) {
            // was a normal entity: detach it first
            let cell_id = (packed >> 32) as u32;
            let idx = (packed & 0xFFFF_FFFF) as usize;
            self.remove_at(cell_id, idx, id);
            self.stats.entities -= 1;
            if let Some(p) = self.oversize.iter().position(|o| o.id == id) {
                self.oversize[p] = Oversize { id, world, b };
                return;
            }
        }
        self.oversize.push(Oversize { id, world, b });
        self.stats.oversize += 1;
    }
}

impl Default for EntityIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[inline]
fn cell_origin(cx: i32, cz: i32) -> (f64, f64) {
    (
        (cx as i64 * CELL as i64) as f64,
        (cz as i64 * CELL as i64) as f64,
    )
}

#[doc(hidden)]
impl EntityIndex {
    /// Deep invariant check: idmap, cells, free list all consistent.
    pub fn check_invariants(&self) {
        use std::collections::HashSet as HS;
        // every cell id either live (mapped in cellmap, non-empty) or free
        for ci in 1..self.cells.len() {
            let cd = &self.cells[ci];
            let live = self.cellmap.find(cd.key) == Some(ci as u32);
            assert_eq!(
                live,
                cd.key != 0 && !cd.ents.is_empty(),
                "cell {ci} live/empty mismatch (key={}, ents={})",
                cd.key,
                cd.ents.len()
            );
            if let Some(f) = self.free_cells.iter().find(|f| **f as usize == ci) {
                let _ = f;
                assert!(cd.key == 0 || cd.ents.is_empty(), "free cell {ci} still holds entities");
            }
        }
        // every cellmap value points at a live cell with that key
        for ci in 1..self.cells.len() {
            let cd = &self.cells[ci];
            if cd.key != 0 && !cd.ents.is_empty() {
                assert!(self.cellmap.find(cd.key).is_some(), "live cell {ci} not in cellmap");
            }
        }
        // every mapped id resolves to the right cell+idx
        let mut seen: HS<i64> = HS::new();
        for ci in 1..self.cells.len() {
            let cd = &self.cells[ci];
            for (idx, e) in cd.ents.iter().enumerate() {
                let packed = ((ci as u64) << 32) | idx as u64;
                assert_eq!(self.idmap.find(e.id), Some(packed),
                    "idmap: id {} -> {:?}, expected cell {ci} idx {idx}", e.id, self.idmap.find(e.id));
                assert!(seen.insert(e.id), "duplicate entity id {}", e.id);
            }
        }
        // oversize ids never collide with cell ids
        for o in &self.oversize {
            assert!(self.idmap.find(o.id).is_none(), "id {} both oversize and indexed", o.id);
        }
    }
}
