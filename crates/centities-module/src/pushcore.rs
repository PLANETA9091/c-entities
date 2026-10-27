//! L12 — the native push kernel.
//!
//! The doc-21 CI profile (ab-paired #12, mixed-150k WITH the bench bot) put
//! **79.95% of all server-thread samples** inside
//! `LivingEntity.pushEntities() -> Level.getPushableEntities() ->
//! CeHooks.getEntities -> CeHooks.mirror -> EntityLookup.getEntities`: every
//! mob's push is a section scan whose cost is O(section population), and in
//! the bench the anchor section holds ~100k items + 30k stands + the pile.
//! Both paths (mirror AND the native index scan) are O(population) there —
//! the EMA gate honestly prefers the mirror and the win evaporates.
//!
//! The kernel removes the query AND the per-pair Java floor:
//!
//! * a second, PUSHABLE-ONLY index (this file's `pidx` view) — push queries
//!   stop paying for 130k non-pushable items/stands;
//! * the whole `pushEntities` body computed natively per pusher, from the
//!   exact 26.3 bytecode (`javap` dumps in docs/research/23):
//!   `isPushable` gate, team rule (routed vanilla), cramming RNG/audit,
//!   `numCollisions` reset + `maxEntityCollisions` cap, `Entity.push` force
//!   math (absMax metric, 0.009999999776482582 threshold, 0.05f quantum),
//!   mutual delta writes, `needsSync`.
//! * candidates are served in the EXACT vanilla EntityLookup.getEntities
//!   scan order — (chunkZ, chunkX, sectionY, section-insertion seq), the
//!   seq mirrors Java's ORD_SEQ stamps (addEntity / moveEntity re-append);
//!   the kernel's per-cell `sort_cell_by_keys` keeps each cell sorted at
//!   most once per tick and the cell walk is chunkZ-major, so the merged
//!   stream is bit-identical to vanilla's for the cap and force math.
//!
//! State mirrors (the arena): exact f64 box6 (probeBox), exact x/z, delta
//! xyz (probeDelta + kernel writes), flag bits, `numCollisions`. Java reads
//! the mirrors back at the entity's own kernel call — the accumulation
//! window between two writebacks is exactly the vanilla between-ticks
//! window, so intra-tick causality matches vanilla for every entity that
//! pushes (all of them run pushEntities each tick).

use std::collections::HashMap;

// Flag bits — must match CeHooks.PF_*.
pub const PF_PUSHABLE: u8 = 1;
pub const PF_SPECTATOR: u8 = 2;
pub const PF_NOPHYSICS: u8 = 4;
pub const PF_PASSENGER: u8 = 8;
pub const PF_VEHICLE: u8 = 16;
pub const PF_PLAYER: u8 = 32;
pub const PF_NEEDSSYNC: u8 = 64;

const PS_CAP: usize = 1 << 22;
const PS_MASK: usize = PS_CAP - 1;

/// Vanilla constants (javap-verified, Entity.push / pushEntities).
pub const PUSH_EPS: f64 = 0.009999999776482582;
pub const PUSH_QUANTUM: f64 = 0.05000000074505806;

pub const R_MIRROR: i64 = 0; // kernel declined — vanilla body must run
pub const R_OK: i64 = 1;

/// The density gate (see centities-core::cell_population): below this many
/// pushable rows in the covered cells the vanilla body is cheaper than the
/// kernel's fixed cost. Measured: break-even ~50-80 rows; the A/B spread
/// legs (10-30 rows/cell) lost +27ms/tick without the gate.
pub const GATE_MIN: usize = 96;

/// Per-entity push mirror. Slots are addressed `id & PS_MASK` with the full
/// id stored for exactness (the Java ARENA pattern). A slot is live iff
/// `key[slot] == id` — removals null the key synchronously, so a removed
/// entity can never be pushed as a candidate.
#[derive(Default)]
pub struct PushCore {
    pub key: Vec<i64>,
    /// exact AABB: [x0,y0,z0,x1,y1,z1] per slot
    pub box6: Vec<f64>,
    /// exact getX/getZ per slot
    pub px: Vec<f64>,
    pub pz: Vec<f64>,
    /// delta mirror per slot
    pub dx: Vec<f64>,
    pub dy: Vec<f64>,
    pub dz: Vec<f64>,
    pub flags: Vec<u8>,
    /// direct vehicle id (0 = none) — for the isPassengerOfSameVehicle check
    pub vid: Vec<i64>,
    /// numCollisions mirror (the ONLY reader/writer outside is the vanilla
    /// pushEntities body — javap-verified — so the mirror is authoritative)
    pub ncol: Vec<i32>,
    /// L37 vanilla-order key halves: the clamped section Y and the global
    /// insertion seq (Java bumps at addEntity / moveEntity re-append). With
    /// px/pz (chunkZ/X at sort time) the kernel sorts cell candidates into
    /// the exact EntityLookup.getEntities scan order — merge partner choice
    /// and the maxEntityCollisions cap depend on it.
    pub sy: Vec<i32>,
    pub seq: Vec<i32>,
    /// L37b: the entity's home-cell key (world+cell of the box center) at
    /// the last upsert — structural invalidation: when an entity's order
    /// key (sy/seq) or home cell CHANGES, the affected cells' cached sorts
    /// are stale and must be cleared (vanilla re-appends on every section
    /// change — the per-tick sort cache would otherwise serve the old
    /// order to later pushes in that cell).
    pub home_key: Vec<u64>,
    /// pending kernel delta additions (see the MERGE STATE note)
    pub kadd_dx: Vec<f64>,
    pub kadd_dz: Vec<f64>,
    /// L12 MERGE STATE: the kernel's pending delta additions per slot.
    /// mirror.dx == java_value + kernel_add_dx. Java-side delta events
    /// (probeDelta/probeBox, which always see the PRE-kernel Java field)
    /// rebaze the mirror onto the new Java value while PRESERVING the
    /// pending kernel pushes; the self writeback resets kernel_add to zero
    /// (the Java field then equals the mirror exactly).
    /// cell key -> tick of the last id-sort (the kernel sorts at most once
    /// per (cell, tick))
    pub sorted_at: HashMap<u64, u64>,
    pub population: u64,
}

impl PushCore {
    pub fn new() -> Self {
        PushCore {
            key: vec![0; PS_CAP],
            box6: vec![0.0; PS_CAP * 6],
            px: vec![0.0; PS_CAP],
            pz: vec![0.0; PS_CAP],
            dx: vec![0.0; PS_CAP],
            dy: vec![0.0; PS_CAP],
            dz: vec![0.0; PS_CAP],
            flags: vec![0; PS_CAP],
            vid: vec![0; PS_CAP],
            ncol: vec![0; PS_CAP],
            sy: vec![0; PS_CAP],
            seq: vec![0; PS_CAP],
            home_key: vec![0; PS_CAP],
            kadd_dx: vec![0.0; PS_CAP],
            kadd_dz: vec![0.0; PS_CAP],
            sorted_at: HashMap::new(),
            population: 0,
        }
    }

    #[inline]
    pub fn slot_of(&self, id: i64) -> Option<usize> {
        let s = (id as usize) & PS_MASK;
        if self.key[s] == id {
            Some(s)
        } else {
            None
        }
    }

    /// Apply one upsert event (from the Java probe batch). `pushable == 0`
    /// removes the entity from the push view (it stays in the main index).
    #[allow(clippy::too_many_arguments)]
    pub fn upsert(
        &mut self,
        id: i64,
        wid: u16,
        b: &centities_core::Aabb,
        px: f64,
        pz: f64,
        dx: f64,
        dy: f64,
        dz: f64,
        flags: u8,
        vid: i64,
        sy: i32,
        seq: i32,
    ) -> Option<i64> {
        let s = (id as usize) & PS_MASK;
        let mut evicted = None;
        let mut structural = false;
        if self.key[s] != id {
            if self.key[s] != 0 {
                // slot collision (id & MASK): the previous owner loses its
                // arena slot — the caller must drop it from the push index
                // too, or its candidates would silently miss the mirror
                evicted = Some(self.key[s]);
            } else {
                self.population += 1;
            }
            self.key[s] = id;
            self.ncol[s] = 0; // a fresh id starts the counter at zero
            self.kadd_dx[s] = 0.0;
            self.kadd_dz[s] = 0.0;
            structural = true; // a fresh member changes the cell list
        } else {
            // the ORDER key changes only when the seq/section-Y changes or
            // the position crosses a CHUNK border (cz/cx in the key) — a
            // move within the chunk keeps both the vanilla list position
            // and the key, so the cached sort stays valid (per-move
            // invalidation would kill the once-per-tick amortization)
            let ocx = ((self.px[s].floor() as i64) >> 4) as i32;
            let ocz = ((self.pz[s].floor() as i64) >> 4) as i32;
            let ncx = ((px.floor() as i64) >> 4) as i32;
            let ncz = ((pz.floor() as i64) >> 4) as i32;
            structural = self.seq[s] != seq || self.sy[s] != sy
                || ocx != ncx || ocz != ncz;
        }
        let o = s * 6;
        self.box6[o] = b.x0;
        self.box6[o + 1] = b.y0;
        self.box6[o + 2] = b.z0;
        self.box6[o + 3] = b.x1;
        self.box6[o + 4] = b.y1;
        self.box6[o + 5] = b.z1;
        self.px[s] = px;
        self.pz[s] = pz;
        // Java-side delta view rebased onto the fresh Java field; the
        // kernel's pending pushes survive (mirror = java + kadd)
        self.dx[s] = dx + self.kadd_dx[s];
        self.dz[s] = dz + self.kadd_dz[s];
        self.dy[s] = dy;
        self.flags[s] = flags;
        self.vid[s] = vid;
        self.sy[s] = sy;
        self.seq[s] = seq;
        // L37b structural invalidation: clear the cached sorts of BOTH the
        // old and the new home cell (position cell too — the center-based
        // home can sit one cell from the position near a border).
        if structural {
            let old_home = self.home_key[s];
            if old_home != 0 {
                self.sorted_at.remove(&old_home);
            }
            let ccx = (((b.x0 + b.x1) * 0.5).floor() as i32) >> centities_core::CELL_SHIFT;
            let ccz = (((b.z0 + b.z1) * 0.5).floor() as i32) >> centities_core::CELL_SHIFT;
            let new_home = centities_core::cell_key_pub(wid, ccx, ccz);
            self.sorted_at.remove(&new_home);
            let pcx = (((px.floor() as i32)) >> centities_core::CELL_SHIFT) as i64;
            let pcz = (((pz.floor() as i32)) >> centities_core::CELL_SHIFT) as i64;
            let pos_key = centities_core::cell_key_pub(wid, pcx as i32, pcz as i32);
            if pos_key != new_home {
                self.sorted_at.remove(&pos_key);
            }
            self.home_key[s] = new_home;
        }
        evicted
    }

    /// Delta-only refresh (probeDelta events without a box change). The
    /// event value is the JAVA field (pre-kernel): rebase the mirror onto
    /// it and keep the kernel's pending additions.
    pub fn set_delta(&mut self, id: i64, dx: f64, dy: f64, dz: f64) {
        if let Some(s) = self.slot_of(id) {
            self.dx[s] = dx + self.kadd_dx[s];
            self.dz[s] = dz + self.kadd_dz[s];
            self.dy[s] = dy;
        }
    }

    pub fn remove(&mut self, id: i64) {
        let s = (id as usize) & PS_MASK;
        if self.key[s] == id {
            // L37b: a removal reshuffles the cell's list (vanilla arraycopy-
            // shifts) — the cached per-cell sort is stale
            let hk = self.home_key[s];
            if hk != 0 {
                self.sorted_at.remove(&hk);
            }
            self.home_key[s] = 0;
            self.key[s] = 0;
            self.flags[s] = 0;
            self.ncol[s] = 0;
            self.kadd_dx[s] = 0.0;
            self.kadd_dz[s] = 0.0;
            self.population -= 1;
        }
    }

    pub fn clear(&mut self) {
        for k in self.key.iter_mut() {
            *k = 0;
        }
        for f in self.flags.iter_mut() {
            *f = 0;
        }
        for n in self.ncol.iter_mut() {
            *n = 0;
        }
        for v in self.kadd_dx.iter_mut() {
            *v = 0.0;
        }
        for v in self.kadd_dz.iter_mut() {
            *v = 0.0;
        }
        self.sorted_at.clear();
        self.population = 0;
    }
}

/// One `LivingEntity.pushEntities()` computed natively.
///
/// `pidx` is the pushable-only index (already flushed with the caller's
/// pending probe batches). `tick` stamps cell sorts. Outputs ride back via
/// `&mut Out`.
#[allow(clippy::too_many_arguments)]
pub fn push_tick(
    pc: &mut PushCore,
    pidx: &mut centities_core::EntityIndex,
    world: u16,
    self_id: i64,
    self_slot: usize,
    self_vehicle: bool,
    self_player: bool,
    only_players_collide: bool,
    player_collisions: bool,
    max_ec: i32,
    cramming: i32,
    gate_min: usize,
    tick: u64,
    out: &mut PushOut,
) {
    out.ok = false;
    out.cramming = false;
    out.non_passengers = 0;
    out.ncol = 0;
    out.ddx = 0.0;
    out.ddy = 0.0;
    out.ddz = 0.0;

    let sc = self_slot * 6;
    let sx0 = pc.box6[sc];
    let sy0 = pc.box6[sc + 1];
    let sz0 = pc.box6[sc + 2];
    let sx1 = pc.box6[sc + 3];
    let sy1 = pc.box6[sc + 4];
    let sz1 = pc.box6[sc + 5];
    let spx = pc.px[self_slot];
    let spz = pc.pz[self_slot];
    let self_pushable = pc.flags[self_slot] & PF_PUSHABLE != 0;
    let self_nophys = pc.flags[self_slot] & PF_NOPHYSICS != 0;

    // Density gate FIRST (before any scanning work): sum the covered
    // cells' pushable-row populations; sparse worlds route vanilla.
    let ex_gate = pidx.max_half_span() + centities_core::MARGIN as f64;
    let gx0 = ((sx0 - ex_gate).floor() as i32) >> centities_core::CELL_SHIFT;
    let gx1 = ((sx1 + ex_gate).floor() as i32) >> centities_core::CELL_SHIFT;
    let gz0 = ((sz0 - ex_gate).floor() as i32) >> centities_core::CELL_SHIFT;
    let gz1 = ((sz1 + ex_gate).floor() as i32) >> centities_core::CELL_SHIFT;
    let mut rows = 0usize;
    for cz in gz0..=gz1 {
        for cx in gx0..=gx1 {
            rows += pidx.cell_population(world, cx, cz);
        }
    }
    if rows < gate_min {
        return; // out.ok stays false -> the caller routes vanilla
    }

    // numCollisions reset (vanilla: max(0, ncol - maxEC) BEFORE the loop).
    let mut ncol_self = pc.ncol[self_slot];
    ncol_self = if ncol_self > max_ec {
        ncol_self - max_ec
    } else {
        0
    };

    // Candidate scan over the pushable-only index. L37: cells are visited
    // CZ-MAJOR (vanilla iterates regions Z outer/X inner, chunks Z outer/
    // X inner — chunkZ-major), and each cell's entries are sorted at most
    // once per (cell, tick) by the EXACT vanilla scan key (chunkZ, chunkX,
    // sectionY, insertion seq — mirrors of the Java ORD_SEQ stamps). The
    // merged stream is therefore bit-identical to vanilla's
    // EntityLookup.getEntities order: the maxEntityCollisions cap and the
    // f64 force accumulation see the same neighbors in the same sequence.
    // `query_cell` is a provable superset (f32 prefilter + margin); the
    // exact f64 intersect below restores the vanilla set.
    let mut cell_hits: Vec<i64> = Vec::new();

    let ex = pidx.max_half_span() + centities_core::MARGIN as f64;
    let cx0 = ((sx0 - ex).floor() as i32) >> centities_core::CELL_SHIFT;
    let cx1 = ((sx1 + ex).floor() as i32) >> centities_core::CELL_SHIFT;
    let cz0 = ((sz0 - ex).floor() as i32) >> centities_core::CELL_SHIFT;
    let cz1 = ((sz1 + ex).floor() as i32) >> centities_core::CELL_SHIFT;

    let mut hits_count: i32 = 0;
    let mut non_pass: i32 = 0;
    let mut ddx = 0.0f64;
    let mut ddz = 0.0f64;
    let mut base_dx = pc.dx[self_slot];
    let mut base_dy = pc.dy[self_slot];
    let mut base_dz = pc.dz[self_slot];

    for cz in cz0..=cz1 {
        for cx in cx0..=cx1 {
            let key = centities_core::cell_key_pub(world, cx, cz);
            if pc.sorted_at.get(&key).copied().unwrap_or(0) != tick {
                pidx.sort_cell_by_keys(world, cx, cz, |id| match pc.slot_of(id) {
                    Some(s) => {
                        // px/pz are POSITIONS: >> 4 first (chunk coords,
                        // ±2M), then the 22-bit bias — the same key shape
                        // as CeHooks.ordKey (which reads the already-chunk-
                        // sized moonrise section fields directly).
                        let czk = (((pc.pz[s].floor() as i64) >> 4) + 2097152) & 0x3F_FFFF;
                        let cxk = (((pc.px[s].floor() as i64) >> 4) + 2097152) & 0x3F_FFFF;
                        let syk = (pc.sy[s] as i64 + 2097152) & 0x3F_FFFF;
                        let seq = pc.seq[s] as i64 & 0xFFFF_FFFF;
                        ((czk << 22) | cxk, (syk << 40) | seq)
                    }
                    // unknown slot (index-only id): never a kernel candidate
                    // (the pushable filter needs the mirror); park it last.
                    None => (i64::MAX, i64::MAX),
                });
                pc.sorted_at.insert(key, tick);
            }
            cell_hits.clear();
            pidx.query_cell(world, cx, cz, sx0, sy0, sz0, sx1, sy1, sz1, &mut cell_hits);
            for &id in cell_hits.iter() {
                if id == self_id {
                    continue;
                }
                let Some(hs) = pc.slot_of(id) else {
                    continue;
                };
                let fl = pc.flags[hs];
                if fl & PF_PUSHABLE == 0 || fl & PF_SPECTATOR != 0 || fl & PF_NOPHYSICS != 0 {
                    continue;
                }
                let o = hs * 6;
                // exact vanilla AABB.intersects(AABB) — strict compares
                if !(pc.box6[o] < sx1
                    && pc.box6[o + 3] > sx0
                    && pc.box6[o + 1] < sy1
                    && pc.box6[o + 4] > sy0
                    && pc.box6[o + 2] < sz1
                    && pc.box6[o + 5] > sz0)
                {
                    continue;
                }
                hits_count += 1;
                let is_pass = fl & PF_PASSENGER != 0;
                if !is_pass {
                    non_pass += 1;
                }
                // numCollisions cap (vanilla `return`s the whole body)
                if ncol_self >= max_ec {
                    // keep counting hits for the cramming audit, skip the rest
                    continue;
                }
                // ---- vanilla loop body: numCollisions bookkeeping happens
                // BEFORE doPush — an Entity.push early return (same vehicle,
                // noPhysics, eps) never undoes it (javap 295-319) ----
                pc.ncol[hs] += 1;
                ncol_self += 1;
                // Entity.push pair gates (all live INSIDE push, after ncol++)
                if is_pass && pc.vid[hs] == self_id {
                    continue; // isPassengerOfSameVehicle
                }
                if self_nophys || fl & PF_NOPHYSICS != 0 {
                    continue;
                }
                if only_players_collide && !self_player && fl & PF_PLAYER == 0 {
                    continue;
                }
                if fl & PF_PLAYER != 0 && !player_collisions {
                    continue;
                }
                // ---- Entity.push force math (exact constants) ----
                let opx = pc.px[hs];
                let opz = pc.pz[hs];
                let mut dx = opx - spx;
                let mut dz = opz - spz;
                let d0 = abs_max(dx, dz);
                if d0 < PUSH_EPS {
                    continue;
                }
                let d = d0.sqrt();
                dx /= d;
                dz /= d;
                let mut f = 1.0 / d;
                if f > 1.0 {
                    f = 1.0;
                }
                dx *= f;
                dz *= f;
                dx *= PUSH_QUANTUM;
                dz *= PUSH_QUANTUM;
                // this.push(-dx, 0, -dz) when not a vehicle and pushable
                if !self_vehicle && self_pushable {
                    base_dx += -dx;
                    base_dz += -dz;
                    ddx += -dx;
                    ddz += -dz;
                }
                // other.push(dx, 0, dz) when other is not a vehicle — the
                // addition lands in the mirror AND the pending-kadd ledger
                if fl & PF_VEHICLE == 0 {
                    pc.dx[hs] += dx;
                    pc.dz[hs] += dz;
                    pc.kadd_dx[hs] += dx;
                    pc.kadd_dz[hs] += dz;
                    pc.flags[hs] |= PF_NEEDSSYNC;
                }
            }
        }
    }

    pc.ncol[self_slot] = ncol_self;
    pc.dx[self_slot] = base_dx;
    pc.dy[self_slot] = base_dy;
    pc.dz[self_slot] = base_dz;
    // the Java writeback (the guard's setDeltaMovement with these exact
    // values) makes the Java field equal the mirror — the pending ledger
    // for self resets. The probeDelta event of that writeback rebases onto
    // the same value (idempotent).
    pc.kadd_dx[self_slot] = 0.0;
    pc.kadd_dz[self_slot] = 0.0;
    pc.flags[self_slot] |= PF_NEEDSSYNC;

    out.ok = true;
    out.cramming = cramming > 0 && hits_count > cramming - 1;
    out.non_passengers = non_pass;
    out.ncol = ncol_self;
    out.ddx = ddx;
    out.ddy = 0.0;
    out.ddz = ddz;
    // the FULL current delta rides back for the Java writeback
    out.dx = base_dx;
    out.dy = base_dy;
    out.dz = base_dz;
}

#[inline]
fn abs_max(a: f64, b: f64) -> f64 {
    // Mth.absMax: Math.max(Math.abs(a), Math.abs(b))
    let aa = a.abs();
    let bb = b.abs();
    if aa > bb {
        aa
    } else {
        bb
    }
}

/// Outputs of one kernel call (Java reads them back through statics).
#[derive(Default)]
pub struct PushOut {
    pub ok: bool,
    pub cramming: bool,
    pub non_passengers: i32,
    pub ncol: i32,
    /// full post-tick delta (for the Java writeback)
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
    /// the delta change contributed this call (diagnostics)
    pub ddx: f64,
    pub ddy: f64,
    pub ddz: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use centities_core::Aabb;

    // All test entities use the zombie box (0.6 x 1.95 x 0.6). Boxes
    // INTERSECT iff |dx| < 0.6 on X (strict) — tests place candidates
    // within that window.

    fn zbox(x: f64, z: f64) -> Aabb {
        Aabb::new(x - 0.3, 0.0, z - 0.3, x + 0.3, 1.95, z + 0.3)
    }

    fn core_with(id: i64, x: f64, z: f64) -> PushCore {
        let mut pc = PushCore::new();
        pc.upsert(id, 0, &zbox(x, z), x, z, 0.0, 0.0, 0.0, PF_PUSHABLE, 0, 0, 0);
        pc
    }

    fn idx_with(id: i64, x: f64, z: f64) -> centities_core::EntityIndex {
        let mut idx = centities_core::EntityIndex::new();
        idx.upsert(id, 0, &zbox(x, z));
        idx
    }

    /// Entity.push force math from the 26.3 bytecode: entity 2 at x=0.5
    /// (boxes overlap 0.1): dx=0.5, dz=0 -> d0=0.5 -> d=sqrt(0.5);
    /// dx' = 0.5/sqrt(0.5) = sqrt(0.5); f = min(1/d, 1) = 1 (d<1);
    /// force = sqrt(0.5) * 1 * 0.05000000074505806.
    #[test]
    fn push_force_matches_bytecode_math() {
        // replicate the exact vanilla op sequence: dx = 0.5/sqrt(0.5)
        // (division, not sqrt — one ULP apart), f capped to 1.0
        let sq = 0.5f64 / 0.5f64.sqrt();
        let mut pidx = idx_with(2, 0.5, 0.0);
        let mut pc = core_with(1, 0.0, 0.0);
        pc.upsert(2, 0, &zbox(0.5, 0.0), 0.5, 0.0, 0.0, 0.0, 0.0, PF_PUSHABLE, 0, 0, 0);
        let mut out = PushOut::default();
        let s1 = pc.slot_of(1).unwrap();
        push_tick(
            &mut pc, &mut pidx, 0, 1, s1, false, false, false, true, 8, 24, 0, 1, &mut out,
        );
        assert!(out.ok);
        let want = sq * 0.05000000074505806;
        assert!(
            (out.ddx - (-want)).abs() < 1e-18,
            "ddx={} want -{want}",
            out.ddx
        );
        assert_eq!(out.ddz, 0.0);
        // the OTHER gained +force on X
        let s2 = pc.slot_of(2).unwrap();
        assert!((pc.dx[s2] - want).abs() < 1e-18);
        // numCollisions advanced on BOTH sides (before doPush)
        assert_eq!(pc.ncol[s2], 1);
        assert_eq!(out.ncol, 1);
        assert!(pc.flags[s2] & PF_NEEDSSYNC != 0);
        assert!(pc.flags[s1] & PF_NEEDSSYNC != 0);
    }

    /// At the SAME position (d0 = 0 < eps) vanilla returns from Entity.push
    /// BEFORE the force, but the loop's numCollisions++ already ran, and the
    /// entity IS a list candidate (the cramming audit counts it).
    #[test]
    fn same_position_eps_return_keeps_counters() {
        let mut pidx = idx_with(2, 0.0, 0.0);
        let mut pc = core_with(1, 0.0, 0.0);
        pc.upsert(2, 0, &zbox(0.0, 0.0), 0.0, 0.0, 0.0, 0.0, 0.0, PF_PUSHABLE, 0, 0, 0);
        let mut out = PushOut::default();
        let s1 = pc.slot_of(1).unwrap();
        push_tick(
            &mut pc, &mut pidx, 0, 1, s1, false, false, false, true, 8, 24, 0, 1, &mut out,
        );
        assert_eq!(out.non_passengers, 1);
        assert_eq!(out.ncol, 1, "ncol++ happens before doPush");
        let s2 = pc.slot_of(2).unwrap();
        assert_eq!(pc.ncol[s2], 1);
        assert_eq!(out.ddx, 0.0, "no force below eps");
        assert!(!out.cramming, "1 hit <= cramming-1=23");
    }

    /// The maxEntityCollisions cap: 5 candidates, maxEC=2 -> the two LOWEST
    /// ids get pairs (ID order), all five counted for the audit.
    #[test]
    fn cap_limits_pairs_but_not_audit() {
        let mut pidx = centities_core::EntityIndex::new();
        let mut pc = core_with(1, 0.0, 0.0);
        for k in 2..7i64 {
            let x = 0.05 * (k - 1) as f64; // 0.05..0.25 — all intersect
            pidx.upsert(k, 0, &zbox(x, 0.0));
            pc.upsert(k, 0, &zbox(x, 0.0), x, 0.0, 0.0, 0.0, 0.0, PF_PUSHABLE, 0, 0, 0);
        }
        let mut out = PushOut::default();
        let s1 = pc.slot_of(1).unwrap();
        push_tick(
            &mut pc, &mut pidx, 0, 1, s1, false, false, false, true, 2, 24, 0, 1, &mut out,
        );
        assert_eq!(out.non_passengers, 5, "all hits audited despite the cap");
        assert_eq!(out.ncol, 2, "the cap stops pair processing at maxEC");
        for k in 2..7i64 {
            let s = pc.slot_of(k).unwrap();
            let want = if k <= 3 { 1 } else { 0 };
            assert_eq!(pc.ncol[s], want, "id {k}");
        }
    }

    /// The cramming audit: 30 hits > cramming-1=23 -> the Java side runs
    /// nextInt(4) + hurtServer. The cap (maxEC=8) does NOT affect the count.
    #[test]
    fn cramming_flag_needs_vanilla_condition() {
        let mut pidx = centities_core::EntityIndex::new();
        let mut pc = core_with(1, 0.0, 0.0);
        for k in 2..32i64 {
            let x = 0.01 * (k - 1) as f64; // 0.01..0.29 — all intersect
            pidx.upsert(k, 0, &zbox(x, 0.0));
            pc.upsert(k, 0, &zbox(x, 0.0), x, 0.0, 0.0, 0.0, 0.0, PF_PUSHABLE, 0, 0, 0);
        }
        let mut out = PushOut::default();
        let s1 = pc.slot_of(1).unwrap();
        push_tick(
            &mut pc, &mut pidx, 0, 1, s1, false, false, false, true, 8, 24, 0, 1, &mut out,
        );
        assert_eq!(out.non_passengers, 30);
        assert!(out.cramming);
    }

    /// Same-vehicle candidates: numCollisions++ runs (loop body), then
    /// Entity.push returns at isPassengerOfSameVehicle — no force.
    #[test]
    fn same_vehicle_candidate_no_force_but_counters() {
        let mut pidx = idx_with(2, 0.5, 0.0);
        let mut pc = core_with(1, 0.0, 0.0);
        pc.upsert(
            2,
            0,
            &zbox(0.5, 0.0),
            0.5,
            0.0,
            0.0,
            0.0,
            0.0,
            PF_PUSHABLE | PF_PASSENGER,
            1,
            0,
            0,
        );
        let mut out = PushOut::default();
        let s1 = pc.slot_of(1).unwrap();
        push_tick(
            &mut pc, &mut pidx, 0, 1, s1, false, false, false, true, 8, 24, 0, 1, &mut out,
        );
        assert_eq!(out.non_passengers, 0, "passengers excluded from the audit");
        assert_eq!(out.ncol, 1, "the loop counted the pair");
        assert_eq!(out.ddx, 0.0, "no force for same-vehicle pairs");
    }

    /// Removed entities can never be pushed (the slot key check).
    #[test]
    fn removed_candidate_never_pushed() {
        let mut pidx = idx_with(2, 0.5, 0.0);
        let mut pc = core_with(1, 0.0, 0.0);
        pc.upsert(2, 0, &zbox(0.5, 0.0), 0.5, 0.0, 0.0, 0.0, 0.0, PF_PUSHABLE, 0, 0, 0);
        pc.remove(2);
        let mut out = PushOut::default();
        let s1 = pc.slot_of(1).unwrap();
        push_tick(
            &mut pc, &mut pidx, 0, 1, s1, false, false, false, true, 8, 24, 0, 1, &mut out,
        );
        assert_eq!(out.non_passengers, 0);
        assert_eq!(out.ncol, 0);
    }

    /// Cells are served in the VANILLA scan order: (chunkZ, chunkX,
    /// sectionY, insertion seq). With maxEC=2 and ten same-spot candidates
    /// whose seqs run DESCENDING against their ids, the cap keeps the two
    /// LOWEST SEQs (the vanilla first-candidates), not the lowest ids.
    #[test]
    fn candidates_served_in_vanilla_order() {
        let mut pidx = centities_core::EntityIndex::new();
        let mut pc = core_with(1, 0.0, 0.0);
        for k in (2..12i64).rev() {
            let x = 0.02 * (k - 1) as f64; // 0.02..0.18 — all intersect
            // seq REVERSE of id: vanilla re-appends make seq, not id, the
            // order authority
            let seq = (14 - k) as i32;
            pidx.upsert(k, 0, &zbox(x, 0.0));
            pc.upsert(k, 0, &zbox(x, 0.0), x, 0.0, 0.0, 0.0, 0.0, PF_PUSHABLE, 0, 0, seq);
        }
        let mut out = PushOut::default();
        let s1 = pc.slot_of(1).unwrap();
        push_tick(
            &mut pc, &mut pidx, 0, 1, s1, false, false, false, true, 2, 24, 0, 1, &mut out,
        );
        assert_eq!(out.ncol, 2);
        for k in 2..12i64 {
            let s = pc.slot_of(k).unwrap();
            // lowest seqs = ids 11, 10 (seq 3, 4)
            let want = if k >= 10 { 1 } else { 0 };
            assert_eq!(pc.ncol[s], want, "id {k}");
        }
    }

    /// THE MERGE SCHEME: a Java-side delta event (probeDelta) arriving
    /// AFTER the kernel pushed a candidate must NOT erase the pending push.
    /// mirror = java + kadd; the event rebases, the push survives.
    #[test]
    fn java_delta_event_preserves_pending_kernel_push() {
        let mut pidx = idx_with(2, 0.5, 0.0);
        let mut pc = core_with(1, 0.0, 0.0);
        pc.upsert(2, 0, &zbox(0.5, 0.0), 0.5, 0.0, 0.0, 0.0, 0.0, PF_PUSHABLE, 0, 0, 0);
        let mut out = PushOut::default();
        let s1 = pc.slot_of(1).unwrap();
        push_tick(
            &mut pc, &mut pidx, 0, 1, s1, false, false, false, true, 8, 24, 0, 1, &mut out,
        );
        let s2 = pc.slot_of(2).unwrap();
        let pushed = pc.dx[s2];
        assert!(pushed > 0.0);
        // Java writes its (pre-kernel) view of entity 2's delta: 0.0
        pc.set_delta(2, 0.0, 0.0, 0.0);
        assert_eq!(
            pc.dx[s2], pushed,
            "the pending kernel push survives the rebase"
        );
        // and the ledger matches the mirror-minus-java
        assert_eq!(pc.kadd_dx[s2], pushed);
        // self: the writeback resets the ledger
        assert_eq!(pc.kadd_dx[s1], 0.0);
        // a Java change on TOP of the pending push: java 0.1 -> mirror 0.1+pushed
        pc.set_delta(2, 0.1, 0.0, 0.0);
        assert!((pc.dx[s2] - (0.1 + pushed)).abs() < 1e-18);
    }

    /// The density gate: below gate_min rows the kernel declines (ok=false)
    /// without touching any state; at/above it, the tick runs.
    #[test]
    fn density_gate_routes_sparse_worlds() {
        let mut pidx = idx_with(2, 0.5, 0.0);
        let mut pc = core_with(1, 0.0, 0.0);
        pc.upsert(2, 0, &zbox(0.5, 0.0), 0.5, 0.0, 0.0, 0.0, 0.0, PF_PUSHABLE, 0, 0, 0);
        let mut out = PushOut::default();
        let s1 = pc.slot_of(1).unwrap();
        // gate_min=96 with 2 rows -> decline, NO state changes
        push_tick(
            &mut pc, &mut pidx, 0, 1, s1, false, false, false, true, 8, 24, 96, 1, &mut out,
        );
        assert!(!out.ok, "sparse world must route vanilla");
        assert_eq!(out.ncol, 0);
        let s2 = pc.slot_of(2).unwrap();
        assert_eq!(pc.ncol[s2], 0, "declined call must not touch mirrors");
        // gate_min=0 -> the tick runs
        push_tick(
            &mut pc, &mut pidx, 0, 1, s1, false, false, false, true, 8, 24, 0, 1, &mut out,
        );
        assert!(out.ok);
        assert_eq!(out.ncol, 1);
    }

    /// The dy axis is untouched by pushes (y force is always 0).
    #[test]
    fn dy_untouched() {
        let mut pidx = idx_with(2, 0.5, 0.0);
        let mut pc = core_with(1, 0.0, 0.0);
        let s1 = pc.slot_of(1).unwrap();
        pc.dy[s1] = -0.0784;
        pc.upsert(
            2,
            0,
            &zbox(0.5, 0.0),
            0.5,
            0.0,
            0.03,
            -0.08,
            0.0,
            PF_PUSHABLE,
            0,
            0,
            0,
        );
        let mut out = PushOut::default();
        let s1 = pc.slot_of(1).unwrap();
        push_tick(
            &mut pc, &mut pidx, 0, 1, s1, false, false, false, true, 8, 24, 0, 1, &mut out,
        );
        assert_eq!(out.ddy, 0.0);
        let s2 = pc.slot_of(2).unwrap();
        assert_eq!(pc.dy[s2], -0.08);
    }
}
