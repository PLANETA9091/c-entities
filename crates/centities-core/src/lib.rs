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
//! * No external dependencies, no std HashMap on hot paths.
//! * Unsafe is confined to src/simd.rs (masked AVX2 loads over in-bounds
//!   blocked-SoA arrays; scalar fallback + differential parity tests).

#![deny(unsafe_code)]

pub mod goals;
pub mod itemtick;
pub mod mobtravel;
pub mod sections;
pub mod simd;
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

/// Blocked-SoA geometry (doc 21): one block of 8 candidates, six contiguous
/// segments of 8 f32 (x0,y0,z0,x1,y1,z1). Same bytes-per-candidate as the
/// old stride-6 layout, but the query scan reads each segment as a single
/// 8-wide AVX2 vector — zero shuffles. Keep in sync with src/simd.rs.
pub const BLOCK: usize = 8;
pub const SEGMENTS: usize = 6;
pub const BLOCK_F32: usize = BLOCK * SEGMENTS; // 48 f32 per block

/// Prefilter padding for unallocated block lanes: x0 = +INF fails `x0 < qx1`,
/// x1 = -INF fails `x1 > qx0` — padded lanes reject themselves.
pub(crate) const PAD_X0: f32 = f32::INFINITY;
pub(crate) const PAD_X1: f32 = f32::NEG_INFINITY;

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
        Aabb {
            x0,
            y0,
            z0,
            x1,
            y1,
            z1,
        }
    }

    /// Vanilla `AABB.intersects` semantics: strict inequalities.
    #[inline]
    pub fn intersects(&self, o: &Aabb) -> bool {
        self.x0 < o.x1
            && self.x1 > o.x0
            && self.y0 < o.y1
            && self.y1 > o.y0
            && self.z0 < o.z1
            && self.z1 > o.z0
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

/// L12 push kernel: the public cell-key builder (module-side sort stamps
/// are keyed by it).
#[inline]
pub fn cell_key_pub(world: u16, cx: i32, cz: i32) -> u64 {
    cell_key(world, cx, cz)
}

pub fn hash64(mut x: u64) -> u64 {
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
        CellMap {
            keys: vec![0; n],
            vals: vec![0; n],
            mask: (n - 1) as u64,
            len: 0,
        }
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
        IdMap {
            keys: vec![0; n],
            vals: vec![0; n],
            mask: (n - 1) as u64,
            len: 0,
        }
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
    /// vanilla scan-order stamps (moonrise section Y + insertion seq) —
    /// the query's oversize merge sorts by the same key as cell entries
    sy: i32,
    seq: i64,
}

/// Cells below this member count scan whole (a 2x2 split cannot amortize).
const QUAD_MIN: usize = 16;

/// Quadrant sub-partition of one cell (doc 37). Cells with QUAD_MIN+
/// members carry a 4x4 XZ split of their blocked-SoA population: sub-store
/// q holds the members whose stored-f32 box CENTER falls in the 4x4-block
/// tile (xb = q&3, zb = q>>2; ranges xb*4..xb*4+4 cell-relative). Each
/// sub-store is a self-contained blocked-SoA run in the same (sy,seq)
/// order as the parent layout (stable partition), so a query scanning one
/// sub-store emits the vanilla-ordered survivors directly and a
/// multi-store query merges through the per-candidate kb keys carried in
/// `keys` — no idmap probes in the merge (it was 283 ns/query through
/// order_key_of; the 4-way keyed merge is ~30 ns).
///
/// SKIP PROOF (no false negatives): a member of tile q has f64 center c
/// in [xb*4, xb*4+4) and stored half-span <= mh[q] (+f32 rounding,
/// covered by the doc-21 EPSF=1e-2 slack, 1000x above ULP(24)). If the
/// query box margin-expanded by mh[q]+EPSF is disjoint from the tile
/// range, every member's exact f64 test fails on that axis — the member
/// cannot survive, so skipping the tile drops nothing.
struct Quad {
    mh: [f64; 16],    // per-tile max stored half-span (f64 from f32s)
    nb: [u32; 16],    // candidates per tile
    off: [u32; 16],   // id offset into ids/keys
    boff: [u32; 16],  // block offset into bx
    ids: Vec<i64>,    // tile-major, (sy,seq) order inside each tile
    keys: Vec<i64>,   // per-candidate kb sort key ((sy biased)<<40 | seq)
    bx: Vec<f32>,     // blocked-SoA, tile-major
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
    // PARITY ORDER (doc 36b): the vanilla EntityLookup.getEntities candidate
    // order is (chunkZ, chunkX, sectionY, insertion seq). The Java probes
    // carry the entity's exact moonrise section Y and ORD_SEQ insertion
    // stamp with every upsert; each cell keeps an order version that is
    // bumped whenever a member's (sy, seq) or membership changes, and the
    // blocked-SoA layout is re-sorted to the vanilla key before a query
    // streams it (ensure_cell_order). Queries traverse cells chunkZ-major,
    // so the concatenated stream is bit-identical to vanilla's scan order —
    // merge partner ties, maxEntityCollisions caps and targeting all see
    // the same neighbor sequence as the vanilla server.
    sy: Vec<i32>,       // per-slot section Y stamp
    seq: Vec<i64>,      // per-slot insertion seq stamp
    cell_vers: Vec<u64>, // packed order-version word: low32 = current
                         // order version (bumped on change), high32 = the
                         // version the layout was last sorted to. One load
                         // + one compare per query cell (was two loads).
    cell_quad: Vec<Option<Box<Quad>>>, // 4x4 XZ sub-partition (doc 37)
    // scratch for the multi-tile keyed merge (reused, never reallocs hot)
    merge_ids: Vec<i64>,
    merge_keys: Vec<i64>,
    merge_segs: Vec<(usize, usize, usize, usize)>, // (out0, out1, key0, key1)
    // per-cell contiguous SoA (1-based cell ids, parallel to cell_keys)
    cell_ids: Vec<Vec<i64>>,
    cell_box: Vec<Vec<f32>>,   // blocked-SoA: blocks of 48 f32 (simd.rs)
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
            sy: Vec::new(),
            seq: Vec::new(),
            cell_vers: vec![0], // cell 0 unused
            cell_quad: vec![None],
            merge_ids: Vec::new(),
            merge_keys: Vec::new(),
            merge_segs: Vec::new(),
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

    /// Insert or update an entity's AABB without order stamps (pidx view /
    /// tests). The cell layout of this index is then order-insensitive.
    pub fn upsert(&mut self, id: i64, world: u16, b: &Aabb) -> UpsertEv {
        self.upsert_ord(id, world, b, 0, 0)
    }

    /// Insert or update an entity's AABB with its vanilla scan-order stamps.
    /// `sy` = moonrise section Y, `seq` = the ORD_SEQ insertion stamp Java
    /// maintains at addEntity / moveEntity re-append (0,0 = unknown: treated
    /// as a stable key — two stamps of (0,0) never bump the cell version).
    /// O(1) typical (same-cell update is 6 stores into the cell's
    /// contiguous arrays). `world` is a caller-assigned small id per Level
    /// instance. Returns the cell-level change (see [`UpsertEv`]).
    pub fn upsert_ord(&mut self, id: i64, world: u16, b: &Aabb, sy: i32, seq: i64) -> UpsertEv {
        let hx = (b.x1 - b.x0) * 0.5;
        let hz = (b.z1 - b.z0) * 0.5;
        let half = hx.max(hz);

        if half > MAX_HALF_SPAN {
            return self.upsert_oversize_ord(id, world, *b, sy, seq);
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
                let base = (p >> 3) * BLOCK_F32 + (p & 7);
                box_[base] = (b.x0 - ocx as f64 * CELL as f64) as f32;
                box_[base + 8] = b.y0 as f32;
                box_[base + 16] = (b.z0 - ocz as f64 * CELL as f64) as f32;
                box_[base + 24] = (b.x1 - ocx as f64 * CELL as f64) as f32;
                box_[base + 32] = b.y1 as f32;
                box_[base + 40] = (b.z1 - ocz as f64 * CELL as f64) as f32;
                // order stamps: a changed (sy, seq) means vanilla re-appended
                // this entity (or its section moved) — the cell's vanilla
                // order is stale and must be re-sorted before the next stream
                if self.sy[slot as usize] != sy || self.seq[slot as usize] != seq {
                    self.sy[slot as usize] = sy;
                    self.seq[slot as usize] = seq;
                    let v = &mut self.cell_vers[home as usize]; *v = (*v & 0xFFFF_FFFF_0000_0000) | ((*v).wrapping_add(1) & 0xFFFF_FFFF);
                }
                return UpsertEv::Same;
            }
            // moved cell (or world): O(1) swap-remove + push
            self.cell_remove(home, self.cpos[slot as usize] as usize);
            let v = &mut self.cell_vers[home as usize]; *v = (*v & 0xFFFF_FFFF_0000_0000) | ((*v).wrapping_add(1) & 0xFFFF_FFFF);
            let new_home = self.cell_id(world, ncx, ncz);
            self.cell_add(new_home, slot as usize, b, ncx, ncz);
            self.homes[slot as usize] = new_home;
            self.worlds[slot as usize] = world;
            self.sy[slot as usize] = sy;
            self.seq[slot as usize] = seq;
            let v = &mut self.cell_vers[new_home as usize]; *v = (*v & 0xFFFF_FFFF_0000_0000) | ((*v).wrapping_add(1) & 0xFFFF_FFFF);
            return UpsertEv::Moved {
                old: home,
                new: new_home,
            };
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
                self.sy.push(sy);
                self.seq.push(seq);
                (self.ids.len() - 1) as u32
            };
            let home = self.cell_id(world, ncx, ncz);
            self.cell_add(home, slot as usize, b, ncx, ncz);
            self.homes[slot as usize] = home;
            self.idmap.insert(id, slot);
            self.stats.entities += 1;
            let v = &mut self.cell_vers[home as usize]; *v = (*v & 0xFFFF_FFFF_0000_0000) | ((*v).wrapping_add(1) & 0xFFFF_FFFF);
            return UpsertEv::New {
                cell: home,
                from_oversize,
            };
        }
    }

    pub fn remove(&mut self, id: i64) -> RemoveEv {
        if let Some(slot) = self.idmap.find(id) {
            let home = self.homes[slot as usize];
            let v = &mut self.cell_vers[home as usize]; *v = (*v & 0xFFFF_FFFF_0000_0000) | ((*v).wrapping_add(1) & 0xFFFF_FFFF);
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
        self.sy.clear();
        self.seq.clear();
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
        self.cell_vers.truncate(1);
        self.cell_vers[0] = 0;
        self.cell_quad.truncate(1);
        self.cell_quad[0] = None;
        self.merge_ids.clear();
        self.merge_keys.clear();
        self.merge_segs.clear();
        self.cellmap = CellMap::new(1 << 12);
        self.idmap = IdMap::new(1 << 12);
        self.oversize.clear();
        self.stats = Stats::default();
    }

    /// Exact candidate query (see module contract). Appends ids to `out`
    /// (cleared first) in the EXACT vanilla EntityLookup.getEntities scan
    /// order: cells chunkZ-major, each cell's members by (sectionY, seq).
    /// False positives allowed; false negatives impossible:
    ///   - single-homing safety: query XZ box expanded by max_half_span,
    ///   - quantization safety: + MARGIN,
    ///   - oversize entities scanned exactly (and full-order-sorted when
    ///     they match — rare; benches carry none).
    ///
    /// L5: the per-cell scan is a flat sequential read of the cell's
    /// contiguous id/box arrays (no pointer chasing).
    ///
    /// Doc 21: the scan is an AVX2 f32 prefilter over the blocked-SoA box
    /// store (8 candidates per vector iteration, zero shuffles) whose
    /// bounds are the exact bounds expanded by EPSF=1e-2 — a provable
    /// superset of the exact f64 check (f32 rounding of the cell-relative
    /// query bound is <= 7.6e-6, 1300x below EPSF). Survivors are
    /// re-checked by the EXACT f64-promoted compare, so the result set is
    /// bit-identical to the scalar path (differential-tested in
    /// tests/simd_parity.rs). No AVX2 (or mode=1): the scalar exact scan.
    ///
    /// PARITY ORDER (doc 36b): the physical blocked-SoA layout is re-sorted
    /// to the vanilla key lazily (ensure_cell_order) — a versioned,
    /// amortized once-per-cell-between-changes operation — so scan_cells
    /// streams survivors in the vanilla order and the concatenated stream
    /// across the chunkZ-major cell walk is bit-identical to vanilla's.
    pub fn query(&mut self, world: u16, b: &Aabb, out: &mut Vec<i64>) {
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

        // chunkZ-MAJOR cell walk: vanilla iterates regions/chunks Z outer,
        // X inner — the concatenated per-cell streams must follow it.
        for cz in cz0..=cz1 {
            for cx in cx0..=cx1 {
                let key = cell_key(world, cx, cz);
                let cid = match self.cellmap.find(key) {
                    Some(v) => v as usize,
                    None => continue,
                };
                let n = self.cell_ids[cid].len();
                if n == 0 {
                    continue;
                }
                // vanilla order for this cell's members (versioned; no-op
                // when the layout is already current)
                self.ensure_cell_order(cid as u32);
                let (ox, oz) = cell_origin(cx, cz);
                // QUADRANT SUB-PARTITION (doc 37): dense cells scan only the
                // 2x2 XZ sub-stores the margin-expanded box can touch — a
                // provable superset skip (Quad doc), survivors in the same
                // (sy,seq) order as the parent layout
                let mut cell_sources = 0usize;
                match self.cell_quad[cid].as_ref() {
                    Some(q) => {
                        const QEPS: f64 = 1.0e-2; // doc-21 slack (f32 storage)
                        let rel_x0 = qx0 - ox;
                        let rel_x1 = qx1 - ox;
                        let rel_z0 = qz0 - oz;
                        let rel_z1 = qz1 - oz;
                        self.merge_segs.clear();
                        for qb in 0..16 {
                            let nq = q.nb[qb] as usize;
                            if nq == 0 {
                                continue;
                            }
                            let mh = q.mh[qb] + QEPS;
                            let xl = ((qb & 3) * 4) as f64;
                            let zl = ((qb >> 2) * 4) as f64;
                            // skip only when provably disjoint (Quad doc:
                            // center-bucketing + mh expansion + EPSF slack)
                            if rel_x1 + mh < xl
                                || rel_x0 - mh > xl + 4.0
                                || rel_z1 + mh < zl
                                || rel_z0 - mh > zl + 4.0
                            {
                                continue;
                            }
                            let before = out.len();
                            let kbefore = self.merge_keys.len();
                            let s = q.off[qb] as usize;
                            let bo = q.boff[qb] as usize * BLOCK_F32;
                            simd::scan_cells_keys(
                                &q.bx[bo..],
                                &q.ids[s..],
                                &q.keys[s..],
                                nq,
                                ox,
                                oz,
                                qx0,
                                qx1,
                                qz0,
                                qz1,
                                my0,
                                my1,
                                out,
                                &mut self.merge_keys,
                            );
                            if out.len() != before {
                                self.merge_segs
                                    .push((before, out.len(), kbefore, self.merge_keys.len()));
                                cell_sources += 1;
                            }
                        }
                        // multi-tile merge: the emitting segments are each
                        // (sy,seq)-ascending; interleave them by key. The
                        // keys ride with the tile stores — no idmap probes
                        // (the order_key_of sort cost ~283 ns/query).
                        if cell_sources > 1 {
                            let base_out = self.merge_segs[0].0;
                            self.merge_ids.clear();
                            loop {
                                let mut picked = usize::MAX;
                                let mut bestk = i64::MAX;
                                for si in 0..self.merge_segs.len() {
                                    let (o0, o1, k0, _k1) = self.merge_segs[si];
                                    if o0 < o1 {
                                        let k = self.merge_keys[k0];
                                        if k < bestk {
                                            bestk = k;
                                            picked = si;
                                        }
                                    }
                                }
                                if picked == usize::MAX {
                                    break;
                                }
                                let (o0, o1, k0, k1) = self.merge_segs[picked];
                                self.merge_ids.push(out[o0]);
                                self.merge_segs[picked] = (o0 + 1, o1, k0 + 1, k1);
                            }
                            for (i, &id) in self.merge_ids.iter().enumerate() {
                                out[base_out + i] = id;
                            }
                        }
                    }
                    None => {
                        let ids = &self.cell_ids[cid];
                        let bx = &self.cell_box[cid];
                        simd::scan_cells(bx, ids, n, ox, oz, qx0, qx1, qz0, qz1, my0, my1, out);
                    }
                }
            }
        }
        let mut any_ov = false;
        for o in &self.oversize {
            if o.world == world && o.b.intersects(b) {
                out.push(o.id);
                any_ov = true;
            }
        }
        if any_ov {
            // Oversize entities live in vanilla sections interleaved with
            // the cell population; restore the exact global order (rare —
            // the oversize list is empty in every bench scenario). Multi-tile
            // quad merges already restore the vanilla order per cell.
            let mut keyed: Vec<(i64, i64, i64)> = out
                .iter()
                .map(|&id| {
                    let (ka, kb) = self.order_key_of(id).unwrap_or((i64::MAX, i64::MAX));
                    (ka, kb, id)
                })
                .collect();
            keyed.sort_unstable_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
            out.clear();
            out.extend(keyed.into_iter().map(|(_, _, id)| id));
        }
    }

    /// The two-long vanilla order key of `id` — ka = (chunkZ biased)<<22 |
    /// chunkX biased, kb = (sectionY biased)<<40 | seq — the same shape as
    /// CeHooks.ordKey. Used by the oversize merge in [`EntityIndex::query`].
    fn order_key_of(&self, id: i64) -> Option<(i64, i64)> {
        const B: i64 = 2097152;
        const M: i64 = 0x3F_FFFF;
        if let Some(slot) = self.idmap.find(id) {
            let home = self.homes[slot as usize] as usize;
            let key = self.cell_keys[home];
            let (cx, cz) = key_coords(key);
            let ka = ((((cz as i64) + B) & M) << 22) | (((cx as i64) + B) & M);
            let kb = ((((self.sy[slot as usize] as i64) + B) & M) << 40)
                | (self.seq[slot as usize] & 0xFFFF_FFFF);
            return Some((ka, kb));
        }
        if let Some(o) = self.oversize.iter().find(|o| o.id == id) {
            let cx = (o.b.center_x().floor() as i64) >> CELL_SHIFT;
            let cz = (o.b.center_z().floor() as i64) >> CELL_SHIFT;
            let ka = ((((cz) + B) & M) << 22) | (((cx) + B) & M);
            let kb = ((((o.sy as i64) + B) & M) << 40) | (o.seq & 0xFFFF_FFFF);
            return Some((ka, kb));
        }
        None
    }

    /// Public wrapper for the module-side row publisher: bring one cell's
    /// physical layout to the vanilla order before the row is copied to
    /// Java (rowQuery serves candidates in row order — the row must be
    /// vanilla-ordered for the merge/targeting tie-breaks to stay exact).
    pub fn ensure_order(&mut self, cell: u32) {
        self.ensure_cell_order(cell);
    }

    /// Re-sort one cell's physical blocked-SoA layout into the vanilla
    /// (sectionY, seq) order — no-op when the layout already matches the
    /// cell's current order version. Amortized: runs once per cell between
    /// actual order changes, not once per query.
    fn ensure_cell_order(&mut self, cell: u32) {
        let c = cell as usize;
        let v = self.cell_vers[c];
        if (v >> 32) == (v & 0xFFFF_FFFF) {
            return; // layout already sorted to the current order version
        }
        let n = self.cell_ids[c].len();
        if n > 1 {
            // (slot, keyB) rows; keyB = (sy biased)<<40 | seq — within one
            // cell chunkZ/chunkX are constant, so the key halves reduce to
            // sectionY + seq.
            let mut rows: Vec<(u32, i64)> = Vec::with_capacity(n);
            for p in 0..n {
                let slot = self.cell_slots[c][p] as usize;
                let kb = ((((self.sy[slot] as i64) + 2097152) & 0x3F_FFFF) << 40)
                    | (self.seq[slot] & 0xFFFF_FFFF);
                rows.push((slot as u32, kb));
            }
            rows.sort_unstable_by(|a, b| a.1.cmp(&b.1));
            let order: Vec<u32> = rows.into_iter().map(|(s, _)| s).collect();
            self.write_cell_layout(c, &order);
        }
        let v = self.cell_vers[c] & 0xFFFF_FFFF;
        self.cell_vers[c] = v | (v << 32);
        // (re)build the 2x2 XZ quadrant sub-stores for dense cells — the
        // amortized tail of the same lazy versioned rebuild the scan order
        // uses; small cells scan whole (a split cannot amortize there)
        if n >= QUAD_MIN {
            self.build_quad(c);
        } else {
            self.cell_quad[c] = None;
        }
    }

    /// Build the quadrant sub-stores of cell `c` from its CURRENT (sy,seq)
    /// sorted layout: stable partition by stored-f32 box center, so each
    /// 4x4-block tile run keeps the parent's vanilla order. O(n) copies.
    fn build_quad(&mut self, c: usize) {
        let bx = &self.cell_box[c];
        let ids = &self.cell_ids[c];
        let slots = &self.cell_slots[c];
        let n = ids.len();
        debug_assert!(n >= QUAD_MIN);
        let mut cnt = [0usize; 16];
        let mut mh = [0.0f64; 16];
        // pass 1: tile + per-tile max stored half-span
        for p in 0..n {
            let base = (p >> 3) * BLOCK_F32 + (p & 7);
            let x0 = bx[base] as f64;
            let z0 = bx[base + 16] as f64;
            let x1 = bx[base + 24] as f64;
            let z1 = bx[base + 40] as f64;
            // linear band index floor(center/4): the f32-sum x0+x1 is twice
            // the stored center, so band = floor(sum/8); clamp covers the
            // center==16.0 rounding edge (homedy keeps the f64 center in
            // [0,16), the f32 sum can round a 15.99… center to exactly 16)
            let tx = (((x0 + x1) * 0.125) as usize).min(3);
            let tz = (((z0 + z1) * 0.125) as usize).min(3);
            let qb = tx | (tz << 2);
            let hx = (x1 - x0) * 0.5;
            let hz = (z1 - z0) * 0.5;
            let h = if hx > hz { hx } else { hz };
            if h > mh[qb] {
                mh[qb] = h;
            }
            cnt[qb] += 1;
        }
        // pass 2: tile-major blocked-SoA sub-stores (exact f32 copies) with
        // the per-candidate vanilla order key for the no-probe merge
        let mut off = [0u32; 16];
        let mut boff = [0u32; 16];
        let mut run_id = 0usize;
        let mut run_blk = 0usize;
        for q in 0..16 {
            off[q] = run_id as u32;
            boff[q] = run_blk as u32;
            run_id += cnt[q];
            run_blk += (cnt[q] + 7) >> 3;
        }
        let mut qids = vec![0i64; run_id];
        let mut qkeys = vec![0i64; run_id];
        let mut qbx = vec![0.0f32; run_blk * BLOCK_F32];
        let mut wr = [0usize; 16];
        for p in 0..n {
            let base = (p >> 3) * BLOCK_F32 + (p & 7);
            let x0 = bx[base];
            let y0 = bx[base + 8];
            let z0 = bx[base + 16];
            let x1 = bx[base + 24];
            let y1 = bx[base + 32];
            let z1 = bx[base + 40];
            let cx2 = x0 as f64 + x1 as f64;
            let cz2 = z0 as f64 + z1 as f64;
            let tx = ((cx2 * 0.125) as usize).min(3);
            let tz = ((cz2 * 0.125) as usize).min(3);
            let qb = tx | (tz << 2);
            let slot = slots[p] as usize;
            let kb = ((((self.sy[slot] as i64) + 2097152) & 0x3F_FFFF) << 40)
                | (self.seq[slot] & 0xFFFF_FFFF);
            let i = wr[qb];
            wr[qb] += 1;
            let dbase = (boff[qb] as usize + (i >> 3)) * BLOCK_F32 + (i & 7);
            qbx[dbase] = x0;
            qbx[dbase + 8] = y0;
            qbx[dbase + 16] = z0;
            qbx[dbase + 24] = x1;
            qbx[dbase + 32] = y1;
            qbx[dbase + 40] = z1;
            let didx = off[qb] as usize + i;
            qids[didx] = ids[p];
            qkeys[didx] = kb;
        }
        // self-rejecting pad for the free lanes of each tile's tail block
        for q in 0..16 {
            let rem = cnt[q] & 7;
            if rem != 0 {
                let tb = (boff[q] as usize + ((cnt[q] + 7) >> 3) - 1) * BLOCK_F32;
                for l in rem..8 {
                    qbx[tb + l] = PAD_X0;
                    qbx[tb + 24 + l] = PAD_X1;
                }
            }
        }
        self.cell_quad[c] = Some(Box::new(Quad {
            mh,
            nb: [
                cnt[0] as u32, cnt[1] as u32, cnt[2] as u32, cnt[3] as u32,
                cnt[4] as u32, cnt[5] as u32, cnt[6] as u32, cnt[7] as u32,
                cnt[8] as u32, cnt[9] as u32, cnt[10] as u32, cnt[11] as u32,
                cnt[12] as u32, cnt[13] as u32, cnt[14] as u32, cnt[15] as u32,
            ],
            off,
            boff,
            ids: qids,
            keys: qkeys,
            bx: qbx,
        }));
    }

    /// Physically rebuild one cell's blocked-SoA arrays in the given slot
    /// order (shared by ensure_cell_order and sort_cell_by_keys).
    fn write_cell_layout(&mut self, c: usize, order: &[u32]) {
        let mut blocks: Vec<f32> = Vec::with_capacity(((order.len() + 7) >> 3) * BLOCK_F32);
        let old_box = std::mem::take(&mut self.cell_box[c]);
        let mut chunk: [f32; 6] = [0.0; 6];
        for (i, &slot) in order.iter().enumerate() {
            let lane = i & 7;
            if lane == 0 {
                let mut blk = [0.0f32; BLOCK_F32];
                for l in 0..8 {
                    blk[l] = PAD_X0;
                    blk[24 + l] = PAD_X1;
                }
                blocks.extend_from_slice(&blk);
            }
            let oldp = self.cpos[slot as usize] as usize;
            let oldbase = (oldp >> 3) * BLOCK_F32 + (oldp & 7);
            // blocked-SoA segments: x0@0, y0@8, z0@16, x1@24, y1@32, z1@40
            chunk[0] = old_box[oldbase];
            chunk[1] = old_box[oldbase + 8];
            chunk[2] = old_box[oldbase + 16];
            chunk[3] = old_box[oldbase + 24];
            chunk[4] = old_box[oldbase + 32];
            chunk[5] = old_box[oldbase + 40];
            let base = (i >> 3) * BLOCK_F32 + lane;
            let bl = &mut blocks[base..base + 41];
            bl[0] = chunk[0];
            bl[8] = chunk[1];
            bl[16] = chunk[2];
            bl[24] = chunk[3];
            bl[32] = chunk[4];
            bl[40] = chunk[5];
            self.cpos[slot as usize] = i as u32;
        }
        self.cell_box[c] = blocks;
        for (i, &slot) in order.iter().enumerate() {
            self.cell_ids[c][i] = self.ids[slot as usize];
            self.cell_slots[c][i] = slot;
        }
    }

    // ---- internals ----

    /// L12 push kernel: exact candidate scan of ONE cell (f32 AVX2
    /// prefilter + exact f64 re-check), appending ids in the cell's block
    /// order — the vanilla EntityLookup scan order via
    /// [`EntityIndex::sort_cell_by_keys`]. Same
    /// superset contract as [`EntityIndex::query`] (MARGIN-expanded bounds,
    /// no oversize walk — oversize entities are never pushable candidates
    /// here because the pushable filter classifies them separately).
    #[allow(clippy::too_many_arguments)]
    pub fn query_cell(
        &self,
        world: u16,
        cx: i32,
        cz: i32,
        x0: f64,
        y0: f64,
        z0: f64,
        x1: f64,
        y1: f64,
        z1: f64,
        out: &mut Vec<i64>,
    ) {
        let key = cell_key(world, cx, cz);
        let Some(cid) = self.cellmap.find(key) else {
            return;
        };
        let cid = cid as usize;
        let ids = &self.cell_ids[cid];
        let n = ids.len();
        if n == 0 {
            return;
        }
        let bx = &self.cell_box[cid];
        let (ox, oz) = cell_origin(cx, cz);
        // The prefilter + scan_cells' internal f64-promoted compare judge
        // the F32-STORED boxes, whose cell-relative rounding is up to
        // ULP(96) ~ 7.6e-6 off the exact f64 mirror. Expanding the bounds
        // by 1e-2 (the doc-21 EPSF) keeps this scan a provable SUPERSET of
        // the kernel's exact f64 test — no false negatives can escape.
        const QEPS: f64 = 1.0e-2;
        let qx0 = x0 - QEPS;
        let qx1 = x1 + QEPS;
        let qz0 = z0 - QEPS;
        let qz1 = z1 + QEPS;
        let my0 = (y0 - QEPS) as f32;
        let my1 = (y1 + QEPS) as f32;
        simd::scan_cells(bx, ids, n, ox, oz, qx0, qx1, qz0, qz1, my0, my1, out);
    }

    /// L12 push kernel: the public cell-key builder (module-side sort
    /// stamps are keyed by it).
    #[inline]
    pub fn cell_lookup(&self, world: u16, cx: i32, cz: i32) -> Option<u32> {
        self.cellmap.find(cell_key(world, cx, cz))
    }

    /// L12 density gate: the pushable-row population of ONE cell (0 if
    /// absent). The kernel sums this over the covered cells and routes
    /// vanilla below GATE_MIN — the kernel's fixed cost (JNI crossing +
    /// lock + scan setup) beats vanilla only when the local candidate
    /// density is high (measured break-even ~50-80 rows; the A/B spread
    /// legs lost +27ms/tick to the fixed cost at ~10-30 rows/cell).
    #[inline]
    pub fn cell_population(&self, world: u16, cx: i32, cz: i32) -> usize {
        let key = cell_key(world, cx, cz);
        match self.cellmap.find(key) {
            Some(cid) => self.cell_ids[cid as usize].len(),
            None => 0,
        }
    }

    /// L12 push kernel support: sort one cell's entries by a caller-supplied
    /// order key, in place, rebuilding the blocked-SoA box store and fixing
    /// every entry's cpos.
    ///
    /// WHY: vanilla's per-section candidate order is the EntityLookup scan
    /// order (chunkZ, chunkX, sectionY, section-insertion seq) — the push
    /// kernel serves its candidate loop in this order so the
    /// maxEntityCollisions cap picks the same neighbors and the f64 force
    /// accumulation runs in the same order (exact parity for dense piles,
    /// where the cap binds and merge/AI cascade on the order). Amortized:
    /// callers sort at most once per (cell, tick).
    pub fn sort_cell_by_keys<F>(&mut self, world: u16, cx: i32, cz: i32, mut key_of: F)
    where
        F: FnMut(i64) -> (i64, i64),
    {
        let key = cell_key(world, cx, cz);
        let Some(cid) = self.cellmap.find(key) else {
            return;
        };
        let cid = cid as usize;
        let n = self.cell_ids[cid].len();
        if n <= 1 {
            return;
        }
        let bx = &self.cell_box[cid];
        let mut rows: Vec<(i64, u32, [f32; 6], i64, i64)> = Vec::with_capacity(n);
        for p in 0..n {
            let base = (p >> 3) * BLOCK_F32 + (p & 7);
            let id = self.cell_ids[cid][p];
            let (ka, kb) = key_of(id);
            rows.push((
                id,
                self.cell_slots[cid][p],
                [
                    bx[base],
                    bx[base + 8],
                    bx[base + 16],
                    bx[base + 24],
                    bx[base + 32],
                    bx[base + 40],
                ],
                ka,
                kb,
            ));
        }
        rows.sort_unstable_by(|a, b| {
            (a.3, a.4).cmp(&(b.3, b.4))
        });
        let mut blocks: Vec<f32> = Vec::with_capacity(((n + 7) >> 3) * BLOCK_F32);
        let mut chunk: [f32; 6] = [0.0; 6];
        for (i, (_id, slot, b6, _, _)) in rows.iter().enumerate() {
            let lane = i & 7;
            if lane == 0 {
                // fresh block: self-rejecting padding for unallocated lanes
                let mut blk = [0.0f32; BLOCK_F32];
                for l in 0..8 {
                    blk[l] = PAD_X0;
                    blk[24 + l] = PAD_X1;
                }
                blocks.extend_from_slice(&blk);
            }
            let base = (i >> 3) * BLOCK_F32 + lane;
            chunk.copy_from_slice(b6);
            let bl = &mut blocks[base..base + 41];
            bl[0] = chunk[0];
            bl[8] = chunk[1];
            bl[16] = chunk[2];
            bl[24] = chunk[3];
            bl[32] = chunk[4];
            bl[40] = chunk[5];
            self.cpos[*slot as usize] = i as u32;
        }
        let ids: Vec<i64> = rows.iter().map(|r| r.0).collect();
        let slots: Vec<u32> = rows.iter().map(|r| r.1).collect();
        self.cell_ids[cid] = ids;
        self.cell_slots[cid] = slots;
        self.cell_box[cid] = blocks;
    }

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
        self.cell_vers.push(0);
        self.cell_quad.push(None);
        let id = (self.cell_keys.len() - 1) as u32;
        self.cellmap.insert(key, id);
        self.stats.cells += 1;
        let _ = world;
        id
    }

    /// Append an entity to a cell's contiguous arrays; fixes cpos.
    /// Blocked-SoA: a new block (every 8th candidate) is pre-filled with
    /// self-rejecting padding (x0=+INF, x1=-INF) so unallocated lanes of a
    /// partially-filled block can never match the prefilter.
    #[inline]
    fn cell_add(&mut self, cell: u32, slot: usize, b: &Aabb, cx: i32, cz: i32) {
        let ids = &mut self.cell_ids[cell as usize];
        let p = ids.len() as u32;
        if p & 7 == 0 {
            // new block: pad x0 with +INF (fails x0 < qx1) and x1 with -INF
            // (fails x1 > qx0); y/z pad values are never reached
            let mut blk = [0.0f32; BLOCK_F32];
            for lane in 0..8 {
                blk[lane] = PAD_X0;
                blk[24 + lane] = PAD_X1;
            }
            self.cell_box[cell as usize].extend_from_slice(&blk);
        }
        ids.push(self.ids[slot]);
        let base = (p >> 3) as usize * BLOCK_F32 + (p & 7) as usize;
        let box_ = &mut self.cell_box[cell as usize];
        box_[base] = (b.x0 - cx as f64 * CELL as f64) as f32;
        box_[base + 8] = b.y0 as f32;
        box_[base + 16] = (b.z0 - cz as f64 * CELL as f64) as f32;
        box_[base + 24] = (b.x1 - cx as f64 * CELL as f64) as f32;
        box_[base + 32] = b.y1 as f32;
        box_[base + 40] = (b.z1 - cz as f64 * CELL as f64) as f32;
        self.cell_slots[cell as usize].push(slot as u32);
        self.cpos[slot] = p;
    }

    /// Swap-remove position `pos` from a cell's arrays; fixes the moved
    /// entity's cpos. O(1). Blocked-SoA: the moved candidate's six values
    /// are copied segment-wise (block math per segment). The vacated lane
    /// keeps stale data but is past `n` — the scan masks it out.
    #[inline]
    fn cell_remove(&mut self, cell: u32, pos: usize) {
        let ids = &mut self.cell_ids[cell as usize];
        let last = ids.len() - 1;
        if pos != last {
            let moved_slot = self.cell_slots[cell as usize][last] as usize;
            ids[pos] = ids[last];
            let box_ = &mut self.cell_box[cell as usize];
            let pb = (pos >> 3) * BLOCK_F32 + (pos & 7);
            let lb = (last >> 3) * BLOCK_F32 + (last & 7);
            for &off in &[0usize, 8, 16, 24, 32, 40] {
                box_[pb + off] = box_[lb + off];
            }
            self.cell_slots[cell as usize][pos] = moved_slot as u32;
            self.cpos[moved_slot] = pos as u32;
        }
        ids.pop();
        // keep whole-block capacity (padded lanes are self-rejecting); only
        // release a block when it became fully empty
        let keep = ((ids.len() + 7) >> 3) * BLOCK_F32;
        if self.cell_box[cell as usize].len() > keep {
            self.cell_box[cell as usize].truncate(keep);
        }
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
            let moved_sy = self.sy[last];
            let moved_seq = self.seq[last];
            self.ids[slot] = moved_id;
            self.worlds[slot] = moved_world;
            self.homes[slot] = moved_home;
            self.cpos[slot] = moved_cpos;
            self.sy[slot] = moved_sy;
            self.seq[slot] = moved_seq;
            let mp = moved_cpos as usize;
            self.cell_slots[moved_home as usize][mp] = slot as u32;
            self.idmap.insert(moved_id, slot as u32);
        }
        self.ids.pop();
        self.worlds.pop();
        self.homes.pop();
        self.cpos.pop();
        self.sy.pop();
        self.seq.pop();
    }

    fn upsert_oversize_ord(&mut self, id: i64, world: u16, b: Aabb, sy: i32, seq: i64) -> UpsertEv {
        if let Some(slot) = self.idmap.find(id) {
            // was a normal entity: detach it first
            let home = self.homes[slot as usize];
            let v = &mut self.cell_vers[home as usize]; *v = (*v & 0xFFFF_FFFF_0000_0000) | ((*v).wrapping_add(1) & 0xFFFF_FFFF);
            self.cell_remove(home, self.cpos[slot as usize] as usize);
            self.idmap.remove(id);
            self.free_slot(slot as usize);
            self.stats.entities -= 1;
            if let Some(p) = self.oversize.iter().position(|o| o.id == id) {
                self.oversize[p] = Oversize { id, world, b, sy, seq };
                return UpsertEv::ToOversize { old: Some(home) };
            }
            self.oversize.push(Oversize { id, world, b, sy, seq });
            self.stats.oversize += 1;
            return UpsertEv::ToOversize { old: Some(home) };
        } else if let Some(p) = self.oversize.iter().position(|o| o.id == id) {
            self.oversize[p] = Oversize { id, world, b, sy, seq };
            return UpsertEv::OversizeSame;
        }
        self.oversize.push(Oversize { id, world, b, sy, seq });
        self.stats.oversize += 1;
        UpsertEv::ToOversize { old: None }
    }

    // ---- 4b.2a Java-row publish helpers (module-side publisher) ----

    /// The cell key of `cell`, or None if out of range.
    #[inline]
    pub fn cell_key_of(&self, cell: u32) -> Option<u64> {
        self.cell_keys
            .get(cell as usize)
            .copied()
            .filter(|k| *k != 0)
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
            let id = ids[i];
            let lo =
                (((simd::box_at(bx, i, 0) * 1024.0).floor() - 2.0).max(-32768.0) as i64) + 32768;
            let hi = (((simd::box_at(bx, i, 3) * 1024.0).ceil() + 2.0).min(32767.0) as i64) + 32768;
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
    /// ISO BENCH: walk phase only — the chunkZ-major cell walk with the
    /// cellmap probes, no order check, no scan. Returns cells visited.
    pub fn iso_walk_only(&mut self, world: u16, b: &Aabb) -> usize {
        let ex = self.max_half_span + MARGIN as f64;
        let cx0 = ((b.x0 - ex).floor() as i32) >> CELL_SHIFT;
        let cx1 = ((b.x1 + ex).floor() as i32) >> CELL_SHIFT;
        let cz0 = ((b.z0 - ex).floor() as i32) >> CELL_SHIFT;
        let cz1 = ((b.z1 + ex).floor() as i32) >> CELL_SHIFT;
        let mut visited = 0usize;
        for cz in cz0..=cz1 {
            for cx in cx0..=cx1 {
                if let Some(cid) = self.cellmap.find(cell_key(world, cx, cz)) {
                    let n = self.cell_ids[cid as usize].len();
                    if n != 0 {
                        visited += n;
                    }
                }
            }
        }
        visited
    }

    /// ISO BENCH: walk + versioned order check, no scan.
    pub fn iso_no_scan(&mut self, world: u16, b: &Aabb) -> usize {
        let ex = self.max_half_span + MARGIN as f64;
        let cx0 = ((b.x0 - ex).floor() as i32) >> CELL_SHIFT;
        let cx1 = ((b.x1 + ex).floor() as i32) >> CELL_SHIFT;
        let cz0 = ((b.z0 - ex).floor() as i32) >> CELL_SHIFT;
        let cz1 = ((b.z1 + ex).floor() as i32) >> CELL_SHIFT;
        let mut visited = 0usize;
        for cz in cz0..=cz1 {
            for cx in cx0..=cx1 {
                if let Some(cid) = self.cellmap.find(cell_key(world, cx, cz)) {
                    let cid = cid as usize;
                    let n = self.cell_ids[cid].len();
                    if n != 0 {
                        self.ensure_cell_order(cid as u32);
                        visited += n;
                    }
                }
            }
        }
        visited
    }

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
            // blocked-SoA: whole-block capacity; the tail block's unallocated
            // lanes hold self-rejecting padding
            assert_eq!(
                ((ids.len() + 7) >> 3) * BLOCK_F32,
                bx.len(),
                "cell {ci}: block capacity mismatch"
            );
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
            assert_eq!(
                self.idmap.find(self.ids[s]),
                Some(s as u32),
                "idmap: entry {s} id {} not mapped",
                self.ids[s]
            );
            assert!(seen_slots.contains(&s), "entry {s} not in any cell");
            let ci = self.homes[s] as usize;
            let p = self.cpos[s] as usize;
            assert_eq!(
                self.cell_ids[ci][p], self.ids[s],
                "cell {ci}[{p}] != ids[{s}]"
            );
        }
        let mut seen_keys: HS<u64> = HS::new();
        for ci in 1..self.cell_keys.len() {
            if self.cell_keys[ci] == 0 {
                continue;
            }
            assert!(
                self.cellmap.find(self.cell_keys[ci]) == Some(ci as u32),
                "cellmap does not map cell {ci}"
            );
            assert!(seen_keys.insert(self.cell_keys[ci]), "duplicate cell key");
        }
    }
}
