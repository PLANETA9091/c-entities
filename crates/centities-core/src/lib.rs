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

pub struct EntityIndex {
    entries: Vec<Entry>,
    homes: Vec<u32>, // parallel to entries: home cell id (1-based)
    heads: Vec<u32>, // cell id -> first slot+1 (0 = empty); 1-based cells
    cell_keys: Vec<u64>, // parallel to heads: the cell's key (for world clear)
    free_cells: Vec<u32>,
    cellmap: CellMap,
    idmap: IdMap,
    oversize: Vec<Oversize>,
    max_half_span: f64, // tracked high-water mark; removal keeps it (safe direction)
    stats: Stats,
}

impl EntityIndex {
    pub fn new() -> Self {
        EntityIndex {
            entries: Vec::new(),
            homes: Vec::new(),
            heads: vec![0],       // cell 0 unused
            cell_keys: vec![0],   // cell 0 unused
            free_cells: Vec::new(),
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
    /// 6 stores). `world` is a caller-assigned small id per Level instance.
    /// Returns the cell-level change (see [`UpsertEv`]).
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
            if self.entries[slot as usize].world == world && ocx == ncx && ocz == ncz {
                // same cell: bounds update only
                let (ox, oz) = cell_origin(ncx, ncz);
                let e = &mut self.entries[slot as usize];
                e.x0 = (b.x0 - ox) as f32;
                e.y0 = b.y0 as f32;
                e.z0 = (b.z0 - oz) as f32;
                e.x1 = (b.x1 - ox) as f32;
                e.y1 = b.y1 as f32;
                e.z1 = (b.z1 - oz) as f32;
                return UpsertEv::Same;
            }
            // moved cell (or world): unlink, rewrite, relink
            self.unlink(home, slot);
            let old_home = home;
            {
                let (ox, oz) = cell_origin(ncx, ncz);
                let e = &mut self.entries[slot as usize];
                e.x0 = (b.x0 - ox) as f32;
                e.y0 = b.y0 as f32;
                e.z0 = (b.z0 - oz) as f32;
                e.x1 = (b.x1 - ox) as f32;
                e.y1 = b.y1 as f32;
                e.z1 = (b.z1 - oz) as f32;
                e.world = world;
            }
            let home = self.cell_id(world, ncx, ncz);
            self.link(home, slot);
            self.homes[slot as usize] = home;
            return UpsertEv::Moved { old: old_home, new: home };
        } else {
            // if it used to be oversize, drop that record
            let mut from_oversize = false;
            if let Some(p) = self.oversize.iter().position(|o| o.id == id) {
                self.oversize.swap_remove(p);
                self.stats.oversize -= 1;
                from_oversize = true;
            }
            let (ox, oz) = cell_origin(ncx, ncz);
            let e = Entry {
                id,
                x0: (b.x0 - ox) as f32,
                y0: b.y0 as f32,
                z0: (b.z0 - oz) as f32,
                x1: (b.x1 - ox) as f32,
                y1: b.y1 as f32,
                z1: (b.z1 - oz) as f32,
                next: 0,
                world,
            };
            let slot = {
                self.entries.push(e);
                self.homes.push(0);
                (self.entries.len() - 1) as u32
            };
            let home = self.cell_id(world, ncx, ncz);
            self.link(home, slot);
            self.homes[slot as usize] = home;
            self.idmap.insert(id, slot);
            self.stats.entities += 1;
            return UpsertEv::New { cell: home, from_oversize };
        }
    }

    pub fn remove(&mut self, id: i64) -> RemoveEv {
        if let Some(slot) = self.idmap.find(id) {
            let home = self.homes[slot as usize];
            self.unlink(home, slot);
            self.idmap.remove(id);
            self.free_slot(slot);
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
        for e in &self.entries {
            if e.world == world {
                dead.push(e.id);
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
        self.entries.clear();
        self.homes.clear();
        self.heads.truncate(1);
        self.heads[0] = 0;
        self.cell_keys.truncate(1);
        self.cell_keys[0] = 0;
        self.free_cells.clear();
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
                let head = match self.cellmap.find(key) {
                    Some(v) => v,
                    None => continue,
                };
                let (ox, oz) = cell_origin(cx, cz);
                let mut cur = self.heads[head as usize];
                while cur != 0 {
                    let e = &self.entries[cur as usize - 1];
                    debug_assert_eq!(e.world, world);
                    if e.y0 < my1 && e.y1 > my0 {
                        let x0 = e.x0 as f64 + ox;
                        let x1 = e.x1 as f64 + ox;
                        let z0 = e.z0 as f64 + oz;
                        let z1 = e.z1 as f64 + oz;
                        if x0 < qx1 && x1 > qx0 && z0 < qz1 && z1 > qz0 {
                            out.push(e.id);
                        }
                    }
                    cur = e.next;
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

    fn cell_id(&mut self, world: u16, cx: i32, cz: i32) -> u32 {
        let key = cell_key(world, cx, cz);
        if let Some(v) = self.cellmap.find(key) {
            return v;
        }
        let id = match self.free_cells.pop() {
            Some(i) => {
                self.heads[i as usize] = 0;
                self.cell_keys[i as usize] = key;
                i
            }
            None => {
                self.heads.push(0);
                self.cell_keys.push(key);
                (self.heads.len() - 1) as u32
            }
        };
        self.cellmap.insert(key, id);
        self.stats.cells += 1;
        id
    }

    #[inline]
    fn link(&mut self, cell: u32, slot: u32) {
        let old = self.heads[cell as usize];
        self.entries[slot as usize].next = old;
        self.heads[cell as usize] = slot + 1;
    }

    fn unlink(&mut self, cell: u32, slot: u32) {
        if self.heads[cell as usize] == slot + 1 {
            let nxt = self.entries[slot as usize].next;
            self.heads[cell as usize] = nxt;
            return;
        }
        let mut cur = self.heads[cell as usize];
        while cur != 0 {
            let i = cur as usize - 1;
            if self.entries[i].next == slot + 1 {
                self.entries[i].next = self.entries[slot as usize].next;
                return;
            }
            cur = self.entries[i].next;
        }
        debug_assert!(false, "unlink: slot missing from cell chain");
    }

    /// After swap-removing `last` into `slot`, fix `to` references in the
    /// moved entity's cell chain (and head).
    fn rechain(&mut self, cell: u32, from: u32, to: u32) {
        if self.heads[cell as usize] == from + 1 {
            self.heads[cell as usize] = to + 1;
            return; // the moved entry was the head; nothing else references it
        }
        let mut cur = self.heads[cell as usize];
        while cur != 0 {
            let i = cur as usize - 1;
            if self.entries[i].next == from + 1 {
                self.entries[i].next = to + 1;
                return;
            }
            cur = self.entries[i].next;
        }
        debug_assert!(false, "rechain: from-slot missing");
    }

    fn free_slot(&mut self, slot: u32) {
        let last = self.entries.len() - 1;
        if slot as usize != last {
            let moved_id = self.entries[last].id;
            let moved_home = self.homes[last];
            self.entries[slot as usize] = self.entries[last];
            self.homes[slot as usize] = moved_home;
            self.rechain(moved_home, last as u32, slot);
            self.idmap.insert(moved_id, slot);
        }
        // the tail index dies with the pop; capacity is reclaimed implicitly
        self.entries.pop();
        self.homes.pop();
    }

    fn upsert_oversize(&mut self, id: i64, world: u16, b: Aabb) -> UpsertEv {
        if let Some(slot) = self.idmap.find(id) {
            // was a normal entity: detach it first
            let home = self.homes[slot as usize];
            self.unlink(home, slot);
            self.idmap.remove(id);
            self.free_slot(slot);
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

    /// Walk the chain of `cell`: push up to `cap` ids into `out`, return
    /// (total_count, overflow) where overflow = total > cap. The full count
    /// is always computed (the publisher needs it to decide publishability),
    /// but ids beyond cap are not stored.
    pub fn cell_scan(&self, cell: u32, out: &mut Vec<i64>, cap: usize) -> (usize, bool) {
        out.clear();
        let head = *self.heads.get(cell as usize).unwrap_or(&0);
        let mut total = 0usize;
        let mut cur = head;
        while cur != 0 {
            let s = cur as usize - 1;
            if total < cap {
                out.push(self.entries[s].id);
            }
            total += 1;
            cur = self.entries[s].next;
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
    /// Deep invariant check: chains, homes, idmap, free list all consistent.
    pub fn check_invariants(&self) {
        use std::collections::HashSet as HS;
        let n = self.entries.len();
        let mut in_chain: HS<usize> = HS::new();
        for ci in 1..self.heads.len() {
            let world = (self.cell_keys[ci] >> 44) as u16;
            let mut cur = self.heads[ci];
            let mut seen = 0usize;
            while cur != 0 {
                let s = cur as usize - 1;
                assert!(s < n, "chain slot {s} out of range (cell {ci})");
                assert!(in_chain.insert(s), "slot {s} in two chains");
                assert_eq!(self.homes[s], ci as u32, "homes[{s}] = {}, expected cell {ci}", self.homes[s]);
                assert_eq!(self.entries[s].world, world, "entry {s} world mismatch in cell {ci}");
                cur = self.entries[s].next;
                seen += 1;
                assert!(seen <= n, "cycle in cell {ci} chain");
            }
        }
        for s in 0..n {
            assert_eq!(self.idmap.find(self.entries[s].id), Some(s as u32),
                "idmap: entry {s} id {} not mapped", self.entries[s].id);
            assert!(in_chain.contains(&s), "entry {s} not in any chain");
        }
        for f in &self.free_cells {
            let fi = *f as usize;
            assert!(fi < self.heads.len(), "free cell {} beyond heads len", fi);
            assert_eq!(self.heads[fi], 0, "freed cell {} still has a chain", fi);
        }
        let mut seen_keys: HS<u64> = HS::new();
        for ci in 1..self.heads.len() {
            assert!(self.cellmap.find(self.cell_keys[ci]) == Some(ci as u32),
                "cellmap does not map cell {ci}");
            assert!(seen_keys.insert(self.cell_keys[ci]), "duplicate cell key");
        }
    }
}
