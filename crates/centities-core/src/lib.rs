//! c-entities core: exact entity→entity proximity index.
//!
//! Design contract (docs/research/05-module-design.md):
//! * The index may return **false positives** (the Java shim re-checks the
//!   exact f64 AABB before applying the vanilla predicate) but must never
//!   return a **false negative**: every entity whose true f64 AABB overlaps
//!   the query box is always offered.
//! * Single-homing: each normal entity lives in exactly one 16-block XZ
//!   cell (the cell of its AABB center). Queries expand by the tracked max
//!   half-span so no center can be missed. Entities wider than one cell go
//!   to a tiny oversize list scanned linearly.
//! * Coordinates are stored as f32 **offsets from the home cell origin**
//!   (range [-32, 96)), bounding quantization error to ~7.6e-6 — far below
//!   the query margin (1e-3). Global f32 would not be exact (ULP ≈ 2 blocks
//!   at the world edge); cell-relative f32 is exact for ±30M block worlds.
//! * No allocation on the steady-state query path (caller-provided buffer).
//! * No external dependencies, no std HashMap on hot paths, no unsafe.

#![forbid(unsafe_code)]

pub mod itemtick;
pub mod sections;
pub mod tick;

/// 16-block XZ cells. Cell edge is the density lever: dense populations
/// (100k+ entities packed tight) scan k entities per query where k scales
/// with (query_box / cell)^2, so finer cells cut the dense-case candidate
/// set quadratically. Vanilla's largest entity (ender dragon) stays well
/// inside the oversize path.
pub const CELL: i32 = 16;
pub const CELL_SHIFT: u32 = 4; // 16 == 1 << 4
/// Query margin in blocks. Bounded below by the max f32 quantization error
/// of cell-relative coordinates (~7.6e-6) with >100x headroom.
pub const MARGIN: f32 = 1.0e-3;
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

struct IdMap {
    keys: Vec<i64>, // 0 = empty; vanilla entity ids are never 0
    vals: Vec<u32>, // slot+1 (0 = empty)
    mask: u64,
    len: usize,
}

impl IdMap {
    fn new(cap_pow2: usize) -> Self {
        let n = cap_pow2.max(16);
        IdMap { keys: vec![0; n], vals: vec![0; n], mask: (n - 1) as u64, len: 0 }
    }

    #[inline]
    fn find(&self, key: i64) -> Option<u32> {
        let mut i = (hash64(key as u64) & self.mask) as usize;
        loop {
            let k = self.keys[i];
            if k == 0 {
                return None;
            }
            if k == key {
                return Some(self.vals[i] - 1);
            }
            i = (i + 1) & self.mask as usize;
        }
    }

    fn insert(&mut self, key: i64, slot: u32) {
        if (self.len + 1) * 10 >= self.keys.len() * 7 {
            self.grow();
        }
        let mut i = (hash64(key as u64) & self.mask) as usize;
        loop {
            let k = self.keys[i];
            if k == 0 {
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
                self.insert(old_keys[i], old_vals[i] - 1);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Index
// ---------------------------------------------------------------------------

/// What an upsert changed at the CELL level. The module's Java-row publisher
/// (4b.2a) uses it to mark exactly the cells whose published id-row is no
/// longer exact. Same-cell box updates leave the row's ID LIST untouched —
/// the Java path re-filters on live boxes — so they publish nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpsertEv {
    /// Box update within the same home cell (row id list unchanged).
    Same,
    /// Entity gained a normal home cell (new insert, or came back from the
    /// oversize list). `from_oversize` also means the oversize population
    /// changed (Java's OVERSIZE_POP must be refreshed).
    New { cell: u32, from_oversize: bool },
    /// Home cell changed: both cells' rows changed.
    Moved { old: u32, new: u32 },
    /// Entity became oversize (was normal in `old`, or was not tracked).
    ToOversize { old: Option<u32> },
    /// Oversize entity's box updated while staying oversize: no row and no
    /// population change — nothing to publish.
    OversizeSame,
}

/// What a remove changed at the CELL level (see UpsertEv).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RemoveEv {
    /// Id was not tracked.
    Absent,
    /// A normal entity left its home cell.
    Normal { cell: u32 },
    /// An oversize entity left the oversize list (population changed).
    Oversize,
}

#[derive(Clone, Copy)]
struct Entry {
    id: i64,
    // cell-relative XZ (f32, range [-32, 96)), absolute Y (f32, world height)
    x0: f32,
    y0: f32,
    z0: f32,
    x1: f32,
    y1: f32,
    z1: f32,
    next: u32, // 0 = end of chain, otherwise slot+1
    world: u16,
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

/// L5 — contiguous per-cell SoA storage.
///
/// The L1..L4 index chained entries through a global `entries` vec
/// (`next` pointers): a dense-cell query pointer-chased ~700 hops over a
/// 100k-entry array (~14us, measured on CI) while vanilla's section scan
/// iterates a contiguous collection (~10us) — the recorded honest loss
/// (doc 15, "dense-cell queries are the remaining honest loss").
///
/// The rework stores each cell's entities in THREE parallel contiguous
/// arrays (ids / 6-f32 boxes / slots): a dense scan is a flat sequential
/// read at memory bandwidth (~2-3us for 700 entries). Exactness contract
/// is unchanged: f32 cell-relative coords (same quantization bound),
/// false positives allowed (Java re-filters live boxes), false negatives
/// impossible (same single-homing + margin math).
pub struct EntityIndex {
    // per-entity sparse arrays (slot-indexed)
    ids: Vec<i64>,
    worlds: Vec<u16>,
    homes: Vec<u32>, // 1-based cell id
    cpos: Vec<u32>,  // position inside the home cell's arrays
    // per-cell contiguous SoA (1-based cell ids, parallel to cell_keys)
    cell_ids: Vec<Vec<i64>>,
    cell_box: Vec<Vec<f32>>,   // stride 6: x0,y0,z0,x1,y1,z1 (f32)
    cell_slots: Vec<Vec<u32>>, // global slot per position (O(1) swap-remove)
    cell_keys: Vec<u64>,       // 0 = unused cell id
    cellmap: CellMap,
    idmap: IdMap,
    oversize: Vec<Oversize>,
    max_half_span: f64, // tracked high-water mark; removal keeps it (safe direction)
    stats: Stats,
}

impl EntityIndex {
    pub fn new() -> Self {
        EntityIndex {
            ids: Vec::new(),
            worlds: Vec::new(),
            homes: Vec::new(),
            cpos: Vec::new(),
            cell_ids: vec![Vec::new()], // cell 0 unused
            cell_box: vec![Vec::new()],
            cell_slots: vec![Vec::new()],
            cell_keys: vec![0],
            cellmap: CellMap::new(1 << 12),
            idmap: IdMap::new(1 << 12),
            oversize: Vec::new(),
            max_half_span: 1.0,
            stats: Stats::default(),
        }
    }

    pub fn stats(&self) -> Stats {
        self.stats.clone()
    }

    /// Current tracked max half-span (diagnostic; tests assert the bound).
    pub fn max_half_span(&self) -> f64 {
        self.max_half_span
    }

    /// Insert or update an entity's AABB. O(1) typical (same-cell update is
    /// 6 stores into the cell's contiguous arrays). `world` is a
    /// caller-assigned small id per Level instance. Returns the cell-level
    /// change (see [`UpsertEv`]).
    pub fn upsert(&mut self, id: i64, world: u16, b: &Aabb) -> UpsertEv {
        let hx = (b.x1 - b.x0) * 0.5;
        let hz = (b.z1 - b.z0) * 0.5;
        let half = hx.max(hz);

        if half > MAX_HALF_SPAN {
            return self.upsert_oversize(id, world, *b);
        }
        if half > self.max_half_span {
            self.max_half_span = half;
        }

        let ncx = (b.center_x().floor() as i32) >> CELL_SHIFT;
        let ncz = (b.center_z().floor() as i32) >> CELL_SHIFT;

        if let Some(slot) = self.idmap.find(id) {
            let home = self.homes[slot as usize];
            let key = self.cell_keys[home as usize];
            let (ocx, ocz) = key_coords(key);
            if self.worlds[slot as usize] == world && ocx == ncx && ocz == ncz {
                // same cell: bounds update in place (contiguous store write)
                let p = self.cpos[slot as usize] as usize;
                let box_ = &mut self.cell_box[home as usize];
                box_[p * 6] = (b.x0 - ocx as f64 * CELL as f64) as f32;
                box_[p * 6 + 1] = b.y0 as f32;
                box_[p * 6 + 2] = (b.z0 - ocz as f64 * CELL as f64) as f32;
                box_[p * 6 + 3] = (b.x1 - ocx as f64 * CELL as f64) as f32;
                box_[p * 6 + 4] = b.y1 as f32;
                box_[p * 6 + 5] = (b.z1 - ocz as f64 * CELL as f64) as f32;
                return UpsertEv::Same;
            }
            // moved cell (or world): O(1) swap-remove + push
            self.cell_remove(home, self.cpos[slot as usize] as usize);
            let new_home = self.cell_id(world, ncx, ncz);
            self.cell_add(new_home, slot as usize, b, ncx, ncz);
            self.homes[slot as usize] = new_home;
            self.worlds[slot as usize] = world;
            return UpsertEv::Moved { old: home, new: new_home };
        } else {
            // if it used to be oversize, drop that record
            let mut from_oversize = false;
            if let Some(p) = self.oversize.iter().position(|o| o.id == id) {
                self.oversize.swap_remove(p);
                self.stats.oversize -= 1;
                from_oversize = true;
            }
            let slot = {
                self.ids.push(id);
                self.worlds.push(world);
                self.homes.push(0);
                self.cpos.push(0);
                (self.ids.len() - 1) as u32
            };
            let home = self.cell_id(world, ncx, ncz);
            self.cell_add(home, slot as usize, b, ncx, ncz);
            self.homes[slot as usize] = home;
            self.idmap.insert(id, slot);
            self.stats.entities += 1;
            return UpsertEv::New { cell: home, from_oversize };
        }
    }

    pub fn remove(&mut self, id: i64) -> RemoveEv {
        if let Some(slot) = self.idmap.find(id) {
            let home = self.homes[slot as usize];
            self.cell_remove(home, self.cpos[slot as usize] as usize);
            self.idmap.remove(id);
            self.free_slot(slot as usize);
            self.stats.entities -= 1;
            RemoveEv::Normal { cell: home }
        } else if let Some(p) = self.oversize.iter().position(|o| o.id == id) {
            self.oversize.swap_remove(p);
            self.stats.oversize -= 1;
            RemoveEv::Oversize
        } else {
            RemoveEv::Absent
        }
    }

    /// Drop every entity belonging to `world` (used on Level unload).
    pub fn clear_world(&mut self, world: u16) {
        let mut dead = Vec::new();
        for (s, w) in self.worlds.iter().enumerate() {
            if *w == world {
                dead.push(self.ids[s]);
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
    }

    /// Drop everything (module reset / kill switch).
    pub fn clear(&mut self) {
        self.ids.clear();
        self.worlds.clear();
        self.homes.clear();
        self.cpos.clear();
        for v in self.cell_ids.iter_mut() {
            v.clear();
        }
        for v in self.cell_box.iter_mut() {
            v.clear();
        }
        for v in self.cell_slots.iter_mut() {
            v.clear();
        }
        self.cell_keys.truncate(1);
        self.cell_keys[0] = 0;
        self.cell_ids.truncate(1);
        self.cell_box.truncate(1);
        self.cell_slots.truncate(1);
        self.cellmap = CellMap::new(1 << 12);
        self.idmap = IdMap::new(1 << 12);
        self.oversize.clear();
        self.stats = Stats::default();
    }

    /// Exact candidate query (see module contract). Appends ids to `out`
    /// (cleared first). False positives allowed; false negatives impossible:
    ///   - single-homing safety: query XZ box expanded by max_half_span,
    ///   - quantization safety: + MARGIN,
    ///   - oversize entities scanned exactly.
    ///
    /// L5: the per-cell scan is a flat sequential read of the cell's
    /// contiguous id/box arrays (no pointer chasing).
    pub fn query(&self, world: u16, b: &Aabb, out: &mut Vec<i64>) {
        out.clear();
        let ex = self.max_half_span + MARGIN as f64;
        let qx0 = b.x0 - ex;
        let qx1 = b.x1 + ex;
        let qz0 = b.z0 - ex;
        let qz1 = b.z1 + ex;
        let cx0 = (qx0.floor() as i32) >> CELL_SHIFT;
        let cx1 = (qx1.floor() as i32) >> CELL_SHIFT;
        let cz0 = (qz0.floor() as i32) >> CELL_SHIFT;
        let cz1 = (qz1.floor() as i32) >> CELL_SHIFT;
        let my0 = (b.y0 - MARGIN as f64) as f32;
        let my1 = (b.y1 + MARGIN as f64) as f32;

        for cx in cx0..=cx1 {
            for cz in cz0..=cz1 {
                let key = cell_key(world, cx, cz);
                let cid = match self.cellmap.find(key) {
                    Some(v) => v as usize,
                    None => continue,
                };
                let ids = &self.cell_ids[cid];
                let bx = &self.cell_box[cid];
                let n = ids.len();
                let (ox, oz) = cell_origin(cx, cz);
                // X/Z compare in f64 EXACTLY like the pre-L5 chain scan: the
                // f32 cell-relative coord is promoted and the f64 origin is
                // added losslessly — no extra rounding beyond the stored
                // quantization (covered by MARGIN). An f32 compare here
                // could produce a false NEGATIVE at world-scale ULPs.
                let mut i = 0;
                while i < n {
                    let o = i * 6;
                    // y first: most rejections win here
                    if bx[o + 1] < my1 && bx[o + 4] > my0 {
                        let x0 = bx[o] as f64 + ox;
                        let x1 = bx[o + 3] as f64 + ox;
                        let z0 = bx[o + 2] as f64 + oz;
                        let z1 = bx[o + 5] as f64 + oz;
                        if x0 < qx1 && x1 > qx0 && z0 < qz1 && z1 > qz0 {
                            out.push(ids[i]);
                        }
                    }
                    i += 1;
                }
            }
        }
        for o in &self.oversize {
            if o.world == world && o.b.intersects(b) {
                out.push(o.id);
            }
        }
    }

    // ---- internals ----

    /// Create-or-get the 1-based cell id for the key. Cells are never
    /// freed (the publisher's dirty-marking relies on stable cell ids).
    fn cell_id(&mut self, world: u16, cx: i32, cz: i32) -> u32 {
        let key = cell_key(world, cx, cz);
        if let Some(v) = self.cellmap.find(key) {
            return v;
        }
        self.cell_ids.push(Vec::new());
        self.cell_box.push(Vec::new());
        self.cell_slots.push(Vec::new());
        self.cell_keys.push(key);
        let id = (self.cell_keys.len() - 1) as u32;
        self.cellmap.insert(key, id);
        self.stats.cells += 1;
        let _ = world;
        id
    }

    /// Append an entity to a cell's contiguous arrays; fixes cpos.
    #[inline]
    fn cell_add(&mut self, cell: u32, slot: usize, b: &Aabb, cx: i32, cz: i32) {
        let p = self.cell_ids[cell as usize].len() as u32;
        self.cell_ids[cell as usize].push(self.ids[slot]);
        self.cell_box[cell as usize].extend_from_slice(&[
            (b.x0 - cx as f64 * CELL as f64) as f32,
            b.y0 as f32,
            (b.z0 - cz as f64 * CELL as f64) as f32,
            (b.x1 - cx as f64 * CELL as f64) as f32,
            b.y1 as f32,
            (b.z1 - cz as f64 * CELL as f64) as f32,
        ]);
        self.cell_slots[cell as usize].push(slot as u32);
        self.cpos[slot] = p;
    }

    /// Swap-remove position `pos` from a cell's arrays; fixes the moved
    /// entity's cpos. O(1).
    #[inline]
    fn cell_remove(&mut self, cell: u32, pos: usize) {
        let ids = &mut self.cell_ids[cell as usize];
        let last = ids.len() - 1;
        if pos != last {
            let moved_slot = self.cell_slots[cell as usize][last] as usize;
            ids[pos] = ids[last];
            let box_ = &mut self.cell_box[cell as usize];
            for k in 0..6 {
                box_[pos * 6 + k] = box_[last * 6 + k];
            }
            self.cell_slots[cell as usize][pos] = moved_slot as u32;
            self.cpos[moved_slot] = pos as u32;
        }
        ids.pop();
        self.cell_box[cell as usize].truncate((ids.len()) * 6);
        self.cell_slots[cell as usize].pop();
    }

    /// Global swap-remove of `slot` (parallel arrays); fixes the moved
    /// entity's cell_slots entry via its cpos. O(1).
    #[inline]
    fn free_slot(&mut self, slot: usize) {
        let last = self.ids.len() - 1;
        if slot != last {
            let moved_id = self.ids[last];
            let moved_world = self.worlds[last];
            let moved_home = self.homes[last];
            let moved_cpos = self.cpos[last];
            self.ids[slot] = moved_id;
            self.worlds[slot] = moved_world;
            self.homes[slot] = moved_home;
            self.cpos[slot] = moved_cpos;
            let mp = moved_cpos as usize;
            self.cell_slots[moved_home as usize][mp] = slot as u32;
            self.idmap.insert(moved_id, slot as u32);
        }
        self.ids.pop();
        self.worlds.pop();
        self.homes.pop();
        self.cpos.pop();
    }

    fn upsert_oversize(&mut self, id: i64, world: u16, b: Aabb) -> UpsertEv {
        if let Some(slot) = self.idmap.find(id) {
            // was a normal entity: detach it first
            let home = self.homes[slot as usize];
            self.cell_remove(home, self.cpos[slot as usize] as usize);
            self.idmap.remove(id);
            self.free_slot(slot as usize);
            self.stats.entities -= 1;
            if let Some(p) = self.oversize.iter().position(|o| o.id == id) {
                self.oversize[p] = Oversize { id, world, b };
                return UpsertEv::ToOversize { old: Some(home) };
            }
            self.oversize.push(Oversize { id, world, b });
            self.stats.oversize += 1;
            return UpsertEv::ToOversize { old: Some(home) };
        } else if let Some(p) = self.oversize.iter().position(|o| o.id == id) {
            self.oversize[p] = Oversize { id, world, b };
            return UpsertEv::OversizeSame;
        }
        self.oversize.push(Oversize { id, world, b });
        self.stats.oversize += 1;
        UpsertEv::ToOversize { old: None }
    }

    // ---- 4b.2a Java-row publish helpers (module-side publisher) ----

    /// The cell key of `cell`, or None if out of range.
    #[inline]
    pub fn cell_key_of(&self, cell: u32) -> Option<u64> {
        self.cell_keys.get(cell as usize).copied().filter(|k| *k != 0)
    }

    /// The count of `cell`: push up to `cap` ids into `out`, return
    /// (total_count, overflow) where overflow = total > cap. The full count
    /// is always computed (the publisher needs it to decide publishability),
    /// but ids beyond cap are not stored.
    pub fn cell_scan(&self, cell: u32, out: &mut Vec<i64>, cap: usize) -> (usize, bool) {
        out.clear();
        let ids = match self.cell_ids.get(cell as usize) {
            Some(v) => v,
            None => return (0, false),
        };
        let total = ids.len();
        let cap = cap.min(total);
        out.extend_from_slice(&ids[..cap]);
        (total, total > cap)
    }

    /// L4c windowed rows: like [`cell_scan`], but each pushed long packs the
    /// id TOGETHER with a conservative x-interval of the entity's box in
    /// 1/1024-block units relative to the row's cell origin:
    ///
    ///   bits 0..31  id (as i32, low word — same as cell_scan emits)
    ///   bits 32..47 xLo biased by +32768 (floor(x0*1024) - 2)
    ///   bits 48..63 xHi biased by +32768 (ceil(x1*1024) + 2)
    ///
    /// CONSERVATIVENESS CONTRACT (exactness of the Java row scan): the packed
    /// interval is a SUPERSET of the entity's true x-interval — the f32
    /// cell-relative entry coordinates lose at most ~0.002 blocks (ULP at
    /// |rel| < 64), covered by the +-2-unit slack on top of floor/ceil. The
    /// Java reader may therefore SKIP a candidate only when its packed
    /// interval is disjoint from the query's x-window — a skipped candidate
    /// provably has no x overlap, hence cannot intersect the query box.
    /// Non-oversize boxes have half-span <= MAX_HALF_SPAN (8 blocks), so the
    /// true interval lies within +-32 blocks of the cell origin and always
    /// fits the biased 16-bit fields; anything that would clamp is already
    /// oversize and never reaches a cell.
    pub fn cell_scan_packed(&self, cell: u32, out: &mut Vec<i64>, cap: usize) -> (usize, bool) {
        out.clear();
        let ids = match self.cell_ids.get(cell as usize) {
            Some(v) => v,
            None => return (0, false),
        };
        let bx = &self.cell_box[cell as usize];
        let total = ids.len();
        let cap = cap.min(total);
        for i in 0..cap {
            let o = i * 6;
            let id = ids[i];
            let lo = (((bx[o] * 1024.0).floor() - 2.0).max(-32768.0) as i64) + 32768;
            let hi = (((bx[o + 3] * 1024.0).ceil() + 2.0).min(32767.0) as i64) + 32768;
            out.push((id & 0xFFFF_FFFF) | (lo << 32) | (hi << 48));
        }
        (total, total > cap)
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

#[inline]
pub fn key_coords(key: u64) -> (i32, i32) {
    const B: i64 = 1 << 20;
    let cx = ((((key >> 22) & 0x3F_FFFF) as i64) - B) as i32;
    let cz = (((key & 0x3F_FFFF) as i64) - B) as i32;
    (cx, cz)
}

#[doc(hidden)]
impl EntityIndex {
    /// Deep invariant check: cells, homes, cpos, idmap all consistent.
    pub fn check_invariants(&self) {
        use std::collections::HashSet as HS;
        let n = self.ids.len();
        let mut seen_slots: HS<usize> = HS::new();
        for ci in 1..self.cell_keys.len() {
            if self.cell_keys[ci] == 0 {
                continue;
            }
            let world = (self.cell_keys[ci] >> 44) as u16;
            let ids = &self.cell_ids[ci];
            let bx = &self.cell_box[ci];
            let slots = &self.cell_slots[ci];
            assert_eq!(ids.len() * 6, bx.len(), "cell {ci}: box stride mismatch");
            assert_eq!(ids.len(), slots.len(), "cell {ci}: slots mismatch");
            for (p, (&id, &slot)) in ids.iter().zip(slots.iter()).enumerate() {
                let s = slot as usize;
                assert!(s < n, "cell {ci} pos {p}: slot {s} out of range");
                assert!(seen_slots.insert(s), "slot {s} in two cells");
                assert_eq!(self.ids[s], id, "cell {ci} pos {p}: id mismatch");
                assert_eq!(self.homes[s], ci as u32, "homes[{s}] != cell {ci}");
                assert_eq!(self.cpos[s], p as u32, "cpos[{s}] != {p}");
                assert_eq!(self.worlds[s], world, "worlds[{s}] mismatch in cell {ci}");
            }
        }
        for s in 0..n {
            assert_eq!(self.idmap.find(self.ids[s]), Some(s as u32),
                "idmap: entry {s} id {} not mapped", self.ids[s]);
            assert!(seen_slots.contains(&s), "entry {s} not in any cell");
            let ci = self.homes[s] as usize;
            let p = self.cpos[s] as usize;
            assert_eq!(self.cell_ids[ci][p], self.ids[s], "cell {ci}[{p}] != ids[{s}]");
        }
        let mut seen_keys: HS<u64> = HS::new();
        for ci in 1..self.cell_keys.len() {
            if self.cell_keys[ci] == 0 {
                continue;
            }
            assert!(self.cellmap.find(self.cell_keys[ci]) == Some(ci as u32),
                "cellmap does not map cell {ci}");
            assert!(seen_keys.insert(self.cell_keys[ci]), "duplicate cell key");
        }
    }
}
