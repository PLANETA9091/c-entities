//! The native full-tick driver: ONE call ticks the whole managed
//! population — AI floor → travel slice → grid refresh → compaction. This
//! is the function the Java host will call once per tick through JNI (the
//! L111 bridge); in Rust it is a plain function call, so the profiler sees
//! EVERY frame.
//!
//! Tick shape (the Horizon contract, creatures.rs:1251): vanilla insertion
//! order, one mob at a time, AI before movement, later mobs observe earlier
//! mobs' results. The profile phases (ai/travel/grid/apply) split the same
//! body the way Horizon's spans do.
use swarm_sim::mob::{AiStat, MobAI, PackState, PlayerSnap};
use swarm_sim::mob::TravelMth as Mth;

use crate::profile::{ai_guard, apply_guard, grid_guard, travel_guard};
use crate::store::{MobSlot, MobStore, SeenGrid};
use crate::world::{OpenGround, PlayerTick, Snapshot, WorldRes};

use rayon::prelude::*;

// TEMP WAVE11 probe: CE_NO_AI=1 skips every ai_tick body (the executor's
// per-mob fixed overhead row). Measurement only — parity knobs never set it.
static NO_AI: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[inline]
pub fn no_ai_on() -> bool {
    NO_AI.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn set_no_ai(on: bool) {
    NO_AI.store(on, std::sync::atomic::Ordering::Relaxed);
}

/// WAVE14: the per-thread eval workspace (the scratch/out/pack triple the
/// serial executor owns once — the parallel arm needs one set per WORKING
/// thread; every field is pure workspace, written before read per mob, so
/// the per-thread copies are content-identical to the shared one).
struct WorkerBufs {
    scratch: Vec<i32>,
    out: Vec<i32>,
    pack: PackState,
}

impl WorkerBufs {
    #[inline]
    fn fresh() -> Self {
        WorkerBufs {
            scratch: Vec::new(),
            out: vec![0i32; swarm_sim::mob::out_len()],
            pack: PackState::default(),
        }
    }
}

/// The executor state: the persistent store + the grid + the shared travel
/// context (`Mth` — the sine/cosine table, one instance for the world).
pub struct NativeTick {
    pub store: MobStore,
    pub grid: SeenGrid,
    pub mth: Mth,
    /// tombstone compaction threshold (free slots) — the Horizon lazy
    /// compact (their slot lists rebuild when tombstones pile up)
    pub compact_at: usize,
    /// the splitmix stroll stream (owned here, handed to the resolver per
    /// mob — the stream advances across mobs within the tick, vanilla
    /// draws it from the mob's OWN random; the swarm-honesty note 1
    /// split stays until the L111 bridge replaces it with the real
    /// per-mob stream handoff)
    pub target_rng: u64,
    /// WAVE3: the per-tick AI accounting buffer — ONE allocation per
    /// executor (grown on demand), returned as a borrow from `tick`.
    stats_buf: Vec<AiStat>,
    /// WAVE2: the shared aiPack scratch — ONE buffer for the whole
    /// population (grown on demand, reused across every mob and round;
    /// the old per-round `vec![0i32; ..]` was a malloc+memset per ROUND
    /// per MOB — ~75k allocations per tick at 25k mobs).
    pack_scratch: Vec<i32>,
    /// WAVE2: the shared floor-output buffer — ONE per executor, reset
    /// per mob over just the read-before-write words (see mob.rs
    /// OUT_RESET; the 3 KiB per-mob memset is gone). Bit-identical per
    /// the `reused_out_buffer_matches_fresh_zero` gate.
    out_buf: Vec<i32>,
    /// WAVE4: the resident-pack state (fp + tick stamps live here — ONE
    /// per executor; the scratch's static/tick halves outlive any mob).
    pack: PackState,
    /// WAVE4: the per-tick PlayerSnap staging buffer (the old code
    /// allocated a fresh Vec every tick).
    players_buf: Vec<PlayerSnap>,
    /// WAVE6 twin hook: true = the resolver's fold contract is OFF — every
    /// request parks (the exact pre-fold semantics; the A/B gate drives
    /// both modes over the same population).
    fold_off: bool,
    /// WAVE9 twin hook: true = the pre-wave9 two-pass walk (the A/B arm).
    /// False = the fused vanilla-order walk (one DRAM stream per tick).
    split_travel: bool,
    /// WAVE10 measurement probe (NOT a semantics arm): skip the travel
    /// slice entirely — the AI-only decomposition row in the bench ledger.
    skip_travel: bool,
    /// WAVE12 twin hook: true = the pre-wave12 PRE-PASS shape (the whole
    /// approach-refresh walk runs before the main loop — one extra full
    /// population stream per tick). False (the production default) = the
    /// refresh FUSES into the main loop, immediately before each mob's own
    /// ai_tick — the only reader of the fields is the mob itself.
    prepass: bool,
    /// WAVE14: the parallel double-buffer (the shadow store). tick_parallel
    /// evals every mob from THIS buffer pair: slot A stays PRISTINE (the
    /// eval runs on a per-mob clone), the result lands in slot B; the
    /// drawers (stroll-draw mobs) re-eval from the pristine A with their
    /// serially pre-assigned stream value; the buffers SWAP at tick end.
    /// Sized to the store between ticks.
    pre_buf: Vec<MobSlot>,
    /// WAVE14: the alive slot list (slot order) — the stats stream
    /// contract is ONE entry per alive mob in slot order; the walk writes
    /// by POSITION through this map.
    alive_idx: Vec<u32>,
    /// WAVE14: the per-mob stroll-draw counts + stats staging (slot-indexed
    /// reuse — zero per-tick allocation) + the repair set (slot flags,
    /// assigned stream values, the (slot, stat-pos) drawer list) + the
    /// persistent per-worker eval workspaces.
    results_buf: Vec<(AiStat, u32)>,
    repair_buf: Vec<bool>,
    assign_buf: Vec<u64>,
    drawer_list: Vec<(u32, u32)>,
    bufs: Vec<WorkerBufs>,
    /// WAVE14: the parallel-arm switch — the PRODUCTION DEFAULT (the twin
    /// gate proved the arm bit-identical; the dispatch falls back to
    /// serial for the twin-only shapes). `set_parallel(false)` / CE_SER=1
    /// restores the serial fused walk (the A/B arm).
    parallel: bool,
}

impl NativeTick {
    pub fn new(target_seed: u64) -> Self {
        NativeTick {
            store: MobStore::new(),
            grid: SeenGrid::default(),
            mth: Mth::new(),
            compact_at: 256,
            target_rng: target_seed,
            pack_scratch: Vec::new(),
            out_buf: vec![0i32; swarm_sim::mob::out_len()],
            stats_buf: Vec::new(),
            pack: PackState::default(),
            players_buf: Vec::new(),
            fold_off: false,
            split_travel: false,
            skip_travel: false,
            prepass: false,
            pre_buf: Vec::new(),
            alive_idx: Vec::new(),
            results_buf: Vec::new(),
            repair_buf: Vec::new(),
            assign_buf: Vec::new(),
            drawer_list: Vec::new(),
            bufs: Vec::new(),
            parallel: true,
        }
    }

    /// WAVE14 twin hook: drive the PARALLEL executor (the double-buffer
    /// shadow-eval with the serial stream pre-assignment). The serial
    /// fused walk stays the default until the twin gate proves the arm;
    /// the dispatch falls back to serial for the twin-only shapes (the
    /// full-pack wire arm, the pre-pass arm, the split-travel arm, the
    /// no-AI probe) — those exist to arm the A/B gates, not to run fast.
    pub fn set_parallel(&mut self, on: bool) {
        self.parallel = on;
    }

    /// WAVE12 twin hook: restore the pre-wave12 approach-refresh PRE-PASS
    /// (the extra full population stream per tick). The production default
    /// is the fused refresh: no mob reads another mob (the folds answer
    /// every scan over the packed players; the snapshot is immutable during
    /// the tick), so refreshing mob i's approach fields right before ITS
    /// ai_tick lands the same two writes at the same point of ITS own
    /// timeline — the executor twin walks both arms bit-for-bit.
    pub fn set_prepass(&mut self, on: bool) {
        self.prepass = on;
    }

    /// WAVE10 measurement probe: skip the travel slice (the AI-only row).
    pub fn set_skip_travel(&mut self, on: bool) {
        self.skip_travel = on;
    }

    /// WAVE6 twin hook: force the parked (pre-fold) semantics.
    pub fn set_fold_off(&mut self, on: bool) {
        self.fold_off = on;
    }

    /// WAVE9 twin hook: restore the pre-wave9 SPLIT shape (all AI first,
    /// then all travel — the two-pass walk). The production default is the
    /// FUSED walk: vanilla runs each mob's movement inside the same
    /// serverAiStep body, so the fused order is the vanilla order — and
    /// the 25k × ~568 B population streams ONCE per tick instead of twice.
    pub fn set_split_travel(&mut self, on: bool) {
        self.split_travel = on;
    }

    /// WAVE4 twin hook: force the wave3 full-pack-every-round semantics
    /// (the resident-vs-full bit test drives one executor per mode).
    pub fn set_full_pack(&mut self, on: bool) {
        self.pack.full = on;
        if on {
            // the stamps describe the resident content — drop them so the
            // full path is also COLD
            self.pack.fp = None;
            self.pack.tick = None;
        }
    }

    /// Host push: replace the player snapshot (the COPY layer contract —
    /// the host serializes nothing else; players only, mobs live in the
    /// store permanently).
    pub fn push_players(&mut self, players: Vec<PlayerTick>) {
        self.store.snapshot.players = players;
    }

    /// Host push: one mob's persistent state created (spawn / handoff from
    /// Java at load-in). The caller supplies the full `MobAI` (the bridge
    /// packs the goal table, RNG mirror and physics fields verbatim).
    pub fn add_mob(&mut self, mob: MobAI) {
        self.store.add(mob);
        // WAVE8c lazy grid: a fresh slot's position lands at the next query
        self.grid.dirty = true;
    }

    pub fn remove_mob(&mut self, id: i64) {
        // WAVE8c lazy grid: the tombstone skips the immediate detach — the
        // next query rebuilds from live positions (the stale-listed id
        // would only be observable through candidates(), which rebuilds
        // first). One dirty store per remove instead of a map op + scan.
        if self.store.remove(id).is_some() {
            self.grid.dirty = true;
        }
    }

    /// The whole managed tick. Returns the per-mob AI accounting — a
    /// borrow of the executor's REUSED stats buffer (wave3: the old code
    /// allocated a fresh 25k-slot Vec — ~1.2 MB — every tick; the buffer
    /// now persists, the borrow ends at the next tick call).
    pub fn tick(&mut self, game_time: i32) -> &[AiStat] {
        // WAVE14: the parallel arm owns the production shape (the fused
        // refresh + the resident pack); the twin-only shapes stay serial.
        if self.parallel && !self.pack.full && !no_ai_on() && !self.prepass && !self.split_travel
        {
            self.tick_parallel(game_time)
        } else {
            self.tick_serial(game_time)
        }
    }

    /// The pre-wave14 SERIAL fused walk (the wave12b shape) — the A/B
    /// twin arm for the parallel gate and the fallback for the twin-only
    /// knobs. Bit-identical to the wave12b release by construction.
    fn tick_serial(&mut self, game_time: i32) -> &[AiStat] {
        // WAVE8c: the grid maintenance is LAZY — the native v0 path never
        // queries candidates() (the folds answer LOS/target scans natively
        // over the packed players), so the per-tick re-bucket walk was pure
        // overhead (~0.7ms/tick at 25k). The grid dirty-flag fires on the
        // first actual query (the JNI production scans route through
        // candidates()); see SeenGrid::candidates.
        // the phase bracket (the tick opener — arms the 1-in-17 sample;
        // WAVE8c FIX: `let _ = grid_guard` MOVED the fn item without
        // calling it — the whole ledger measured zero ticks since wave8c.
        // The guard drops IMMEDIATELY: the grid is lazy (wave8c), its
        // query-time cost lives inside the folds — a live bracket here
        // would book the WHOLE tick into the grid cell)
        drop(grid_guard());
        // players -> the pack snapshot (the floor borrows it per mob;
        // WAVE4: built into a REUSED buffer, not a fresh Vec per tick)
        let players: &[PlayerSnap] = {
            let _g = apply_guard();
            let pb = &mut self.players_buf;
            pb.clear();
            pb.extend(
                self.store
                    .snapshot
                    .players
                    .iter()
                    .map(|p: &PlayerTick| PlayerSnap {
                        x: p.x,
                        y: p.y,
                        z: p.z,
                        id: p.id,
                        flags: p.flags,
                    }),
            );
            &*pb
        };
        // WAVE4: the tick-level player fingerprint (computed ONCE per tick;
        // the stamp check in ai_tick is free per mob)
        let tfp = MobAI::tick_fp(players);
        // WAVE12a: the melee approach refresh FUSED into the main loop —
        // the floor reads approach_x/z live (they must track the target's
        // CURRENT position; vanilla reads getTarget().position() per round
        // through the pack). The old shape ran it as a PRE-PASS: a second
        // full population stream per tick (25k × ~360 B of L3/DRAM
        // traffic) for a per-mob two-field write. The write is independent
        // of every other mob — no mob-mob reads exist (the folds answer
        // every scan over the packed players) and the snapshot is immutable
        // during the tick — so landing it immediately before the mob's OWN
        // ai_tick is bit-identical (the prepass_matches_fused_refresh twin
        // drives both arms 400 ticks). CE_PREPASS=1 / set_prepass(true)
        // restores the pre-pass shape (the A/B arm).
        if self.prepass {
            let _g = apply_guard();
            for slot in self.store.mobs.iter_mut() {
                let tp = slot.mob.target_player;
                if tp >= 0 {
                    if let Some(p) = self.store.snapshot.players.get(tp as usize) {
                        let (px, pz) = (p.x, p.z);
                        slot.mob.approach_x = px;
                        slot.mob.approach_z = pz;
                    } else {
                        slot.mob.target_player = -1;
                    }
                }
            }
        }
        // THE LOOP — vanilla order (insertion), one mob at a time: the
        // mobs after this one observe its tick results through the same
        // store (Horizon creatures.rs:1468). The resolver borrows the
        // snapshot (immutable) while the mob mutates — disjoint fields,
        // the borrow splits cleanly.
        let stats: &mut Vec<AiStat> = &mut self.stats_buf;
        stats.clear();
        // disjoint field borrows: the snapshot is read-only during the
        // loop (it is only REPLACED between ticks through push_players),
        // the mobs mutate by slot.
        let snapshot: &crate::world::Snapshot = &self.store.snapshot;
        // WAVE2: the resolver is built ONCE per tick (the old code
        // constructed a fresh WorldRes per mob — a useless 5-word shuffle
        // x N) and its target stream advances through the tick.
        let mut res = WorldRes::open_ground_v0(snapshot, self.target_rng);
        res.fold_off = self.fold_off;
        for slot in self.store.mobs.iter_mut() {
            if !slot.alive {
                continue;
            }
            // WAVE12a: the fused approach refresh — the two-field write
            // rides the SAME population stream the AI is about to pay for
            // (the fields are L3-hot the moment the floor reads them).
            if !self.prepass {
                let tp = slot.mob.target_player;
                if tp >= 0 {
                    if let Some(p) = self.store.snapshot.players.get(tp as usize) {
                        let (px, pz) = (p.x, p.z);
                        slot.mob.approach_x = px;
                        slot.mob.approach_z = pz;
                    } else {
                        slot.mob.target_player = -1;
                    }
                }
            }
            res.target_rng = self.target_rng;
            let stat = if no_ai_on() {
                // TEMP WAVE11 probe (CE_NO_AI=1): skip ai_tick entirely —
                // measures the executor's per-mob fixed overhead. NOT a
                // semantics arm (the store stops ticking); parity knobs
                // never set this.
                AiStat::default()
            } else {
                let _g = ai_guard();
                slot.mob
                    .ai_tick(players, game_time, &mut res, &mut self.pack_scratch, &mut self.out_buf, &mut self.pack, tfp)
            };
            self.target_rng = res.target_rng;
            stats.push(stat);
            // WAVE9 FUSED TRAVEL: the movement runs in the SAME pass, right
            // after this mob's AI — the vanilla body order (serverAiStep:
            // AI then travel per mob, one mob at a time). No cross-mob reads
            // exist (the folds answer every scan over the packed players;
            // move_tick touches self fields only), so the fused order is
            // bit-identical to the split two-pass walk — proven by the
            // fused-vs-split executor twin. The population's ~14.2 MB
            // (25k × ~568 B) streams ONCE per tick instead of twice: the
            // second pass re-fetched every mob from DRAM/L3 after the AI
            // pass evicted it.
            if !self.split_travel && !self.skip_travel {
                let _g = travel_guard();
                slot.mob.move_tick(&self.mth);
            }
        }
        // the WAVE9 twin arm: the pre-wave9 SPLIT shape (all AI first, then
        // all travel) — kept for the interleaved A/B and the fused-vs-split
        // parity twin.
        if self.split_travel {
            for slot in self.store.mobs.iter_mut() {
                if !slot.alive {
                    continue;
                }
                let _g = travel_guard();
                slot.mob.move_tick(&self.mth);
            }
        }
        // tombstone compaction (between ticks, the Horizon lazy compact).
        // WAVE2: compact RENUMBERS slots — the grid's stored indices all
        // go stale, so it invalidates and the next rebuild re-inserts.
        if self.store.free.len() > self.compact_at {
            self.store.compact();
            self.grid.invalidate_all();
            self.grid.dirty = true;
            for m in &mut self.store.mobs {
                m.grid_cell = (i64::MAX, i64::MAX);
            }
        }
        self.store.game_time = game_time;
        &stats[..]
    }

    /// WAVE14 — THE PARALLEL TICK, v2 (the lean shape). The per-mob
    /// independence theorem made hardware: no mob reads another mob (the
    /// folds answer every scan over the packed players; move_tick touches
    /// self fields only — the fused-vs-split executor twin), the snapshot
    /// is immutable during the tick, and the ONE cross-mob channel — the
    /// shared stroll stream (`target_rng`) — is a PURE DRAW COUNTER:
    /// whether a mob draws is a function of its own state + its own seed
    /// LCG (the stroll gate runs BEFORE the draw and never reads the
    /// stream), at most one draw per ai_tick, and the stream advance is
    /// TWO 48-bit LCG steps per draw regardless of the values (the park
    /// arm draws 2x16 via lcg_next, the fold arm 2x16 via
    /// stroll_draw_lcg16 — the same LCG).
    ///
    /// So the serial mob order fixes WHICH mobs draw and HOW MANY; the
    /// VALUES chain as advance-per-draw. The tick decomposes:
    ///
    ///   pass 1 (parallel): each worker chunk CLONES its slots A→B
    ///     (clone_from — the pre-tick state stays pristine in A), evals
    ///     B in place with a placeholder stream value, and FUSES the
    ///     travel right after (the wave9 vanilla order, per mob).
    ///     Non-drawers are already CORRECT (they never touch the stream).
    ///     Drawers record their draw count; their B state is garbage and
    ///     gets replaced below.
    ///   walk (serial, one fused pass over the alive slots): copy the
    ///     stats into the contract buffer, assign each drawer its stream
    ///     value = the chain advanced by the draws before it; the
    ///     executor's post-tick stream = the chain end (exactly what the
    ///     serial arm leaves in self.target_rng).
    ///   pass 2 (the drawer repair): re-clone ONLY the drawers A→B and
    ///     re-eval with the assigned value + the fused travel (serial
    ///     below 1024 drawers — the thread spin dominates there — chunked
    ///     parallel above). ZERO allocations: the per-chunk results ride
    ///     pre-sliced regions of reused buffers, the worker bufs persist.
    ///   swap: the shadow buffer becomes the store; no copy pass.
    ///
    /// Every mob's result is a pure function of (its pre-state, the
    /// immutable snapshot, game_time, its assigned stream value) and the
    /// assignment is a deterministic serial scan — the parallel result is
    /// bit-identical to tick_serial for ANY thread interleaving. The twin
    /// test drives both arms 400 ticks and diffs the whole-store dumps.
    ///
    /// The v1→v2 lesson (measured): the v1 shape paid ~1.9 ms/tick at ONE
    /// thread — the stack shadow clones (A→stack→B double copy), the
    /// per-chunk result Vec allocations (mmap page faults every tick),
    /// the regrown scratch per map_init share, and the split-travel second
    /// stream. v2 clones A→B once (the eval mutates B in place), rides
    /// pre-sliced result regions, keeps persistent worker bufs, and fuses
    /// the travel — the parallel arm's overhead is now the clone stream
    /// itself (~0.25 ms at 25k), which the extra cores buy back many times.
    fn tick_parallel(&mut self, game_time: i32) -> &[AiStat] {
        drop(grid_guard());
        // players -> the pack snapshot (the same shape as the serial arm)
        let players: &[PlayerSnap] = {
            let _g = apply_guard();
            let pb = &mut self.players_buf;
            pb.clear();
            pb.extend(
                self.store
                    .snapshot
                    .players
                    .iter()
                    .map(|p: &PlayerTick| PlayerSnap {
                        x: p.x,
                        y: p.y,
                        z: p.z,
                        id: p.id,
                        flags: p.flags,
                    }),
            );
            &*pb
        };
        let tfp = MobAI::tick_fp(players);

        let n = self.store.mobs.len();
        // the shadow buffer mirrors the store shape (the resizes are rare:
        // the population changes between ticks, the compact shrinks it)
        if self.pre_buf.len() > n {
            self.pre_buf.truncate(n);
        }
        if self.pre_buf.len() < n {
            self.pre_buf.resize_with(n, || MobSlot {
                mob: MobAI::new(0, 0, 0.0, 0.0, 0.0),
                alive: false,
                grid_cell: (i64::MAX, i64::MAX),
                grid_pos: 0,
            });
        }
        // the alive map (slot order) + the reused per-tick work buffers
        self.alive_idx.clear();
        for (i, s) in self.store.mobs.iter().enumerate() {
            if s.alive {
                self.alive_idx.push(i as u32);
            }
        }
        let alive_len = self.alive_idx.len();
        if self.results_buf.len() != n {
            self.results_buf.resize(n, (AiStat::default(), 0u32));
        }
        self.repair_buf.clear();
        self.repair_buf.resize(n, false);
        self.assign_buf.clear();
        self.assign_buf.resize(n, 0);
        self.drawer_list.clear();
        self.stats_buf.clear();
        self.stats_buf.resize(alive_len, AiStat::default());

        let nthreads = rayon::current_num_threads().max(1);
        // the job-count knob: more, smaller jobs = better load balance (the
        // chaser mobs cost more than the idlers); the rayon overhead per
        // job stays in the noise down to ~1k mobs per job
        let jobs = std::env::var("CE_JOBS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(2);
        let chunk = (n / (nthreads * jobs)).clamp(256, 8192).max(1);
        let nchunks = n.div_ceil(chunk);
        if self.bufs.len() != nchunks {
            // the resize keeps the GROWN buffers (scratch/out/pack) alive
            // across ticks — zero per-tick allocation after the first
            self.bufs.resize_with(nchunks, WorkerBufs::fresh);
        }

        let Self {
            store,
            pre_buf,
            stats_buf,
            alive_idx,
            results_buf,
            repair_buf,
            assign_buf,
            drawer_list,
            bufs,
            target_rng,
            mth,
            fold_off,
            skip_travel,
            compact_at,
            grid,
            ..
        } = self;

        let snapshot: &Snapshot = &store.snapshot;
        let players_raw: &[PlayerTick] = &store.snapshot.players;
        let s0 = *target_rng;
        let fold_off = *fold_off;
        let skip_travel = *skip_travel;

        // ---- PASS 1: clone A→B (A stays pristine — pass 2's re-eval
        // source), eval B in place with the placeholder stream value, fuse
        // the travel. The per-chunk (stat, draws) land in the pre-sliced
        // results region — zero allocation. The tombstones ride along
        // (metadata mirror + mob copy) — the buffers stay index-aligned
        // for the swap and the free list. ----
        results_buf
            .par_chunks_mut(chunk)
            .zip(store.mobs.par_chunks_mut(chunk))
            .zip(pre_buf.par_chunks_mut(chunk))
            .zip(bufs.par_chunks_mut(1))
            .for_each(|(((res_chunk, a_chunk), b_chunk), b1)| {
                let bufs = &mut b1[0];
                let _g = ai_guard();
                for ((res, a), b) in res_chunk.iter_mut().zip(a_chunk).zip(b_chunk) {
                    b.alive = a.alive;
                    b.grid_cell = a.grid_cell;
                    b.grid_pos = a.grid_pos;
                    if !a.alive {
                        b.mob.clone_from(&a.mob);
                        *res = (AiStat::default(), 0);
                        continue;
                    }
                    let (stat, drew) = re_eval_mob(
                        &a.mob,
                        &mut b.mob,
                        players, players_raw, game_time, snapshot, fold_off, s0, tfp, bufs,
                    );
                    if !skip_travel {
                        let _t = travel_guard();
                        b.mob.move_tick(mth);
                    }
                    *res = (stat, drew);
                }
            });

        // ---- THE WALK (serial, ONE fused pass): the stats contract copy +
        // the drawer stream pre-assignment. TWO LCG-48 steps per draw. ----
        let mut stream = s0;
        let mut drawers = 0usize;
        for k in 0..alive_len {
            let slot = alive_idx[k] as usize;
            let (stat, drew) = results_buf[slot];
            stats_buf[k] = stat;
            if drew == 0 {
                continue;
            }
            assign_buf[slot] = stream;
            repair_buf[slot] = true;
            drawer_list.push((slot as u32, k as u32));
            drawers += 1;
            for _ in 0..(2 * drew) {
                stream = stream
                    .wrapping_mul(2521_4903_917)
                    .wrapping_add(11)
                    & 0xFFFF_FFFF_FFFF;
            }
        }
        // the executor's post-tick stream — exactly what the serial arm
        // accumulates across its per-mob handoffs
        *target_rng = stream;

        // ---- PASS 2: the drawer repair — re-clone A→B, re-eval with the
        // assigned value, re-travel. Small sets run serial (the thread
        // spin dominates there); big sets chunk parallel. ----
        if drawers > 0 {
            if drawers < 1024 {
                let mut bufs = WorkerBufs::fresh();
                for &(slot, k) in &*drawer_list {
                    let expect = results_buf[slot as usize].1;
                    let (stat, drew) = re_eval_mob(
                        &store.mobs[slot as usize].mob,
                        &mut pre_buf[slot as usize].mob,
                        players,
                        players_raw,
                        game_time,
                        snapshot,
                        fold_off,
                        assign_buf[slot as usize],
                        tfp,
                        &mut bufs,
                    );
                    debug_assert_eq!(
                        drew, expect,
                        "the stroll draw count must be stream-value-independent"
                    );
                    if !skip_travel {
                        let _t = travel_guard();
                        pre_buf[slot as usize].mob.move_tick(mth);
                    }
                    stats_buf[k as usize] = stat;
                }
            } else {
                let a_all: &[MobSlot] = &store.mobs;
                let base = results_buf.as_ptr() as usize;
                let unit = std::mem::size_of::<(AiStat, u32)>();
                results_buf
                    .par_chunks_mut(chunk)
                    .zip(a_all.par_chunks(chunk))
                    .zip(pre_buf.par_chunks_mut(chunk))
                    .zip(bufs.par_chunks_mut(1))
                    .for_each(move |(((res_chunk, a_chunk), b_chunk), b1)| {
                        let bufs = &mut b1[0];
                        let _g = ai_guard();
                        let cbase = (res_chunk.as_ptr() as usize - base) / unit;
                        for (j, ((res, a), b)) in
                            res_chunk.iter_mut().zip(a_chunk).zip(b_chunk).enumerate()
                        {
                            let slot = cbase + j;
                            if !repair_buf[slot] {
                                continue;
                            }
                            let (stat, _drew) = re_eval_mob(
                                &a.mob,
                                &mut b.mob,
                                players,
                                players_raw,
                                game_time,
                                snapshot,
                                fold_off,
                                assign_buf[slot],
                                tfp,
                                bufs,
                            );
                            if !skip_travel {
                                let _t = travel_guard();
                                b.mob.move_tick(mth);
                            }
                            *res = (stat, 0);
                        }
                    });
                // the repair stats scatter — the drawer list is slot order
                for &(slot, k) in &*drawer_list {
                    stats_buf[k as usize] = results_buf[slot as usize].0;
                }
            }
        }

        // ---- the swap: the shadow becomes the store (the post-tick
        // state), the old store buffer becomes the next tick's shadow ----
        std::mem::swap(&mut store.mobs, pre_buf);
        // tombstone compaction (between ticks, the Horizon lazy compact;
        // the same shape as the serial arm)
        if store.free.len() > *compact_at {
            store.compact();
            grid.invalidate_all();
            grid.dirty = true;
            for m in store.mobs.iter_mut() {
                m.grid_cell = (i64::MAX, i64::MAX);
            }
        }
        store.game_time = game_time;
        stats_buf
    }
}

/// WAVE14: ONE mob's eval from its pristine pre-tick state. The clone
/// lands DIRECTLY in `target` (the shadow slot's mob — the eval mutates
/// it in place; the v1 stack shadow's extra copy is gone). Pass 1 (the
/// placeholder stream) and pass 2 (the assigned value) run the SAME body
/// — the only input that differs is `assign`, and the non-drawers never
/// read it (their results are assign-independent).
#[inline]
fn re_eval_mob(
    a: &MobAI,
    target: &mut MobAI,
    players: &[PlayerSnap],
    players_raw: &[PlayerTick],
    game_time: i32,
    snapshot: &Snapshot,
    fold_off: bool,
    assign: u64,
    tfp: u64,
    bufs: &mut WorkerBufs,
) -> (AiStat, u32) {
    target.clone_from(a);
    // the WAVE12a fused approach refresh — rides the eval input (the
    // only reader of the fields is the mob itself; the write lands on
    // the SHADOW — the pristine slot A is pass-2's re-eval source)
    if target.target_player >= 0 {
        if let Some(p) = players_raw.get(target.target_player as usize) {
            let (px, pz) = (p.x, p.z);
            target.approach_x = px;
            target.approach_z = pz;
        } else {
            target.target_player = -1;
        }
    }
    let mut res: WorldRes<OpenGround> = WorldRes::open_ground_v0(snapshot, assign);
    res.fold_off = fold_off;
    res.draw_count = 0;
    let stat = target.ai_tick(
        players,
        game_time,
        &mut res,
        &mut bufs.scratch,
        &mut bufs.out,
        &mut bufs.pack,
        tfp,
    );
    (stat, res.draw_count)
}

impl Default for NativeTick {
    fn default() -> Self {
        Self::new(0x5EED_1A11_0B1C)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swarm_sim::mob::zombie_table;
    use swarm_sim::mob::MobAI;

    /// WAVE14: the executor twins + the global-knob probes SERIALIZE here.
    /// The suite runs tests concurrently; every twin compares two executor
    /// arms tick-by-tick, and a concurrent knob flip (odd-skip / z0 / the
    /// probes) landing BETWEEN the two arm ticks would diverge them — a
    /// pure test-harness race, not an executor bug. Lock + the knob tests
    /// below make the whole twin family deterministic.
    static EXEC_TWIN_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    macro_rules! twin_guard {
        () => {
            let _twin_guard = EXEC_TWIN_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        };
    }

    /// TEMP WAVE11 slice probe: the EXACT wall decomposition with minimal
    /// population drift (the bench scenario stabilized 400 ticks, then ≤16
    /// timed iterations per variant). cargo test -p centities-native
    /// slice_probe -- --nocapture
    #[test]
    fn slice_probe() {
        twin_guard!();
        let build = || -> NativeTick {
            let mut nt = NativeTick::new(0xC0FFEE);
            for i in 0..25000usize {
                let x = (i % 250) as f64 * 0.6 - 75.0 + 0.05 * ((i / 7) % 3) as f64;
                let z = (i / 250) as f64 * 0.6 - 75.0;
                let mut m = MobAI::new(
                    1000 + i as i64,
                    0xC0FFEE ^ (i as u64).wrapping_mul(0x9E37),
                    x,
                    64.0,
                    z,
                );
                m.goals = swarm_sim::mob::zombie_table_res();
                nt.add_mob(m);
            }
            nt.push_players(vec![PlayerTick {
                x: 0.0,
                y: 64.0,
                z: 0.0,
                id: 1,
                flags: 1,
            }]);
            nt
        };
        let iters = 16usize;
        // the variant runner: ONE instance, the knob flipped between
        // rounds (the drift over 16 iterations is small vs 400 stabilize)
        let run = |name: &str, skip_travel: bool, no_ai: bool, no_acts: bool| {
            let mut nt = build();
            for t in 0..400 {
                nt.tick(t);
            }
            nt.set_skip_travel(skip_travel);
            crate::exec::set_no_ai(no_ai);
            swarm_sim::ledger::set_no_acts(no_acts);
            let t0 = std::time::Instant::now();
            for t in 400..400 + iters as i32 {
                nt.tick(t);
            }
            let d = t0.elapsed().as_secs_f64() * 1000.0 / iters as f64;
            println!("slice {name}: {d:.3} ms/tick");
            crate::exec::set_no_ai(false);
            swarm_sim::ledger::set_no_acts(false);
        };
        run("full", false, false, false);
        run("no-travel", true, false, false);
        run("no-ai", false, true, false);
        run("no-travel+no-ai", true, true, false);
        run("no-travel+no-acts", true, false, true);
        crate::exec::set_no_ai(false);
        swarm_sim::ledger::set_no_acts(false);
    }

    fn mk(id: i64, x: f64, z: f64) -> MobAI {
        let mut m = MobAI::new(id, 0x1234_5678_9abc_def0 + id as u64, x, 64.0, z);
        m.goals = swarm_sim::mob::zombie_table_res();
        m
    }

    #[test]
    fn ticks_run_and_move_without_players() {
        let mut nt = NativeTick::new(42);
        for i in 0..64 {
            nt.add_mob(mk(i, i as f64 * 3.0, (i % 8) as f64 * 5.0));
        }
        let mut any_walked = false;
        for t in 0..200 {
            nt.tick(t);
            if t == 199 {
                for m in &nt.store.mobs {
                    if (m.mob.x - (m.mob.id % 64) as f64 * 3.0).abs() > 1e-6
                        || m.mob.nav_state != 0
                        || m.mob.goals.iter().any(|g| g.running)
                    {
                        any_walked = true;
                    }
                }
            }
        }
        assert!(any_walked, "peaceful mobs must stroll/look around");
        assert_eq!(nt.store.len(), 64);
    }

    #[test]
    fn target_acquisition_and_chase() {
        let mut nt = NativeTick::new(1337);
        for i in 0..32 {
            nt.add_mob(mk(i, i as f64 * 2.0 + 100.0, 100.0));
        }
        // one player inside follow range of several mobs
        nt.push_players(vec![PlayerTick {
            x: 100.0,
            y: 64.0,
            z: 96.0,
            id: 1,
            flags: 1, // PF_ALIVE
        }]);
        for t in 0..400 {
            nt.tick(t);
        }
        let chasing = nt
            .store
            .mobs
            .iter()
            .filter(|m| m.mob.target_player >= 0)
            .count();
        assert!(chasing > 0, "zombies must acquire the target: got {chasing}");
        // at least one mob materially closed distance toward the player
        let mut closed = false;
        for m in &nt.store.mobs {
            if m.mob.target_player >= 0 {
                let dx = m.mob.x - 100.0;
                let dz = m.mob.z - 100.0;
                if dx * dx + dz * dz < 900.0 {
                    closed = true;
                }
            }
        }
        assert!(closed, "a chasing mob must approach the player");
    }

    #[test]
    fn determinism_same_seed_same_endgame() {
        let run = |seed: u64| -> Vec<(u64, u64)> {
            let mut nt = NativeTick::new(seed);
            for i in 0..32 {
                nt.add_mob(mk(i, i as f64, 0.0));
            }
            for t in 0..120 {
                nt.tick(t);
            }
            nt.store
                .mobs
                .iter()
                .map(|m| (m.mob.x.to_bits(), m.mob.z.to_bits()))
                .collect()
        };
        assert_eq!(run(777), run(777), "same seed -> bitwise same positions");
        assert_ne!(run(777), run(778), "different stroll stream -> different path");
    }

    #[test]
    fn remove_and_readd_keeps_population_exact() {
        let mut nt = NativeTick::new(7);
        for i in 0..100 {
            nt.add_mob(mk(i, i as f64, 0.0));
        }
        for i in (0..100).step_by(2) {
            nt.remove_mob(i);
        }
        assert_eq!(nt.store.len(), 50);
        nt.tick(1);
        for i in (0..100).step_by(2) {
            nt.remove_mob(i); // double remove is a no-op
        }
        assert_eq!(nt.store.len(), 50);
        for i in (0..100).step_by(2) {
            nt.add_mob(mk(1000 + i, i as f64, 0.0));
        }
        assert_eq!(nt.store.len(), 100);
        nt.store.compact();
        assert_eq!(nt.store.len(), 100);
        assert_eq!(nt.store.mobs.len(), 100);
    }

    /// THE wave4 executor gate: the resident-pack executor (stamps, shared
    /// scratch) must be BIT-IDENTICAL to the wave3 full-pack executor over
    /// a chased, strolling, target-locked population — positions, seeds,
    /// goal states, nav, the whole store, word for word.
    #[test]
    fn resident_executor_matches_full_pack_executor() {
        twin_guard!();
        let build = |full: bool| -> NativeTick {
            let mut nt = NativeTick::new(0xC0FFEE);
            nt.set_full_pack(full);
            for i in 0..64 {
                let mut m = MobAI::new(
                    1000 + i,
                    0xC0FFEE ^ (i as u64).wrapping_mul(0x9E37),
                    (i % 8) as f64 * 3.0,
                    64.0,
                    (i / 8) as f64 * 3.0,
                );
                m.goals = swarm_sim::mob::zombie_table_res();
                nt.add_mob(m);
            }
            nt.push_players(vec![PlayerTick {
                x: 12.0,
                y: 64.0,
                z: 4.0,
                id: 1,
                flags: 1,
            }]);
            nt
        };
        let mut a = build(false); // resident (production)
        let mut b = build(true); // wave3 full-pack twin
        for t in 0..400 {
            a.tick(t);
            b.tick(t);
            assert_eq!(a.store.len(), b.store.len(), "tick {t}: population");
            for (ma, mb) in a.store.mobs.iter().zip(b.store.mobs.iter()) {
                assert_eq!(ma.mob.id, mb.mob.id, "tick {t}: id order");
                assert_eq!(ma.mob.x.to_bits(), mb.mob.x.to_bits(), "tick {t}: mob {} x", ma.mob.id);
                assert_eq!(ma.mob.y.to_bits(), mb.mob.y.to_bits(), "tick {t}: mob {} y", ma.mob.id);
                assert_eq!(ma.mob.z.to_bits(), mb.mob.z.to_bits(), "tick {t}: mob {} z", ma.mob.id);
                assert_eq!(ma.mob.seed, mb.mob.seed, "tick {t}: mob {} seed", ma.mob.id);
                assert_eq!(ma.mob.yaw.to_bits(), mb.mob.yaw.to_bits(), "tick {t}: mob {} yaw", ma.mob.id);
                assert_eq!(ma.mob.nav_state, mb.mob.nav_state, "tick {t}: mob {} nav", ma.mob.id);
                assert_eq!(ma.mob.target_player, mb.mob.target_player, "tick {t}: mob {} target", ma.mob.id);
                assert_eq!(ma.mob.dx.to_bits(), mb.mob.dx.to_bits(), "tick {t}: mob {} dx", ma.mob.id);
                assert_eq!(ma.mob.dz.to_bits(), mb.mob.dz.to_bits(), "tick {t}: mob {} dz", ma.mob.id);
                for (ga, gb) in ma.mob.goals.iter().zip(mb.mob.goals.iter()) {
                    assert_eq!(ga.running, gb.running, "tick {t}: mob {} goal running", ma.mob.id);
                    assert_eq!(ga.state, gb.state, "tick {t}: mob {} goal state", ma.mob.id);
                }
            }
        }
        // the resident twin must actually chase (the live-path exercise)
        let chasing = a
            .store
            .mobs
            .iter()
            .filter(|m| m.mob.target_player >= 0)
            .count();
        assert!(chasing > 0, "the twin scenario must acquire targets");
    }

    /// THE WAVE9 executor gate: the FUSED walk (AI + travel per mob, the
    /// vanilla body order, one DRAM stream) must be BIT-IDENTICAL to the
    /// SPLIT two-pass walk (the pre-wave9 shape) over the same chased/
    /// strolling/looking population — positions, seeds, goal states, nav,
    /// targets, the whole store, word for word, 400 ticks. The fusion is
    /// legal because no cross-mob reads exist (the folds answer every scan
    /// over the packed players; move_tick touches self fields only) — this
    /// twin PROVES that claim every CI run.
    #[test]
    fn fused_executor_matches_split_executor() {
        twin_guard!();
        let build = |split: bool| -> NativeTick {
            let mut nt = NativeTick::new(0xC0FFEE);
            nt.set_split_travel(split);
            for i in 0..96 {
                let mut m = MobAI::new(
                    1000 + i,
                    0xC0FFEE ^ (i as u64).wrapping_mul(0x9E37),
                    (i % 12) as f64 * 3.0,
                    64.0,
                    (i / 12) as f64 * 3.0,
                );
                m.goals = swarm_sim::mob::zombie_table_res();
                nt.add_mob(m);
            }
            nt.push_players(vec![PlayerTick {
                x: 12.0,
                y: 64.0,
                z: 4.0,
                id: 1,
                flags: 1,
            }]);
            nt
        };
        let mut split = build(true);
        let mut fused = build(false);
        let dump = |nt: &NativeTick| -> Vec<u64> {
            let mut h = 0xcbf2_9ce4_8422_2325u64;
            let mut pop = 0u64;
            for s in &nt.store.mobs {
                if !s.alive {
                    continue;
                }
                pop += 1;
                let m = &s.mob;
                for w in [
                    m.id as u64,
                    m.x.to_bits(),
                    m.y.to_bits(),
                    m.z.to_bits(),
                    m.seed,
                    m.yaw.to_bits() as u64,
                    m.dx.to_bits(),
                    m.dy.to_bits(),
                    m.dz.to_bits(),
                    m.nav_state as u64,
                    m.nav_x.to_bits(),
                    m.nav_z.to_bits(),
                    m.target_player as u64,
                    m.no_action as u64,
                    m.tick_count as u64,
                    m.rel_x.to_bits(),
                    m.rel_z.to_bits(),
                    m.nav_x.to_bits(),
                    m.nav_z.to_bits(),
                    m.pend_x.to_bits(),
                    m.pend_z.to_bits(),
                    m.approach_x.to_bits(),
                    m.approach_z.to_bits(),
                ] {
                    h = (h ^ w).wrapping_mul(0x1000_0000_01b3);
                }
                for g in &m.goals {
                    h = (h ^ (g.running as u64)).wrapping_mul(0x1000_0000_01b3);
                    h = (h ^ (g.state as u64)).wrapping_mul(0x1000_0000_01b3);
                }
            }
            [h, nt.target_rng, pop].to_vec()
        };
        for t in 0..400 {
            split.tick(t);
            fused.tick(t);
            assert_eq!(
                dump(&split),
                dump(&fused),
                "tick {t}: the fused executor diverged from the split twin"
            );
        }
        // the scenario must move (the fused travel actually runs)
        let moved = fused
            .store
            .mobs
            .iter()
            .filter(|m| m.mob.x != m.mob.x.trunc() || m.mob.goals.iter().any(|g| g.running))
            .count();
        assert!(moved > 0, "the fused twin must walk/stroll: {moved}");
    }

    /// THE WAVE12 gate: the fused approach refresh (the two-field write
    /// inside the main loop, right before each mob's own ai_tick) must be
    /// BIT-IDENTICAL to the pre-wave12 pre-pass (the separate whole-store
    /// walk before the loop) over the same chased/strolling/looking
    /// population — the whole-store dump per tick, 400 ticks. The write is
    /// independent of every other mob (no mob-mob reads: the folds answer
    /// every scan over the packed players; the snapshot is immutable during
    /// the tick), so both arms must land the same words every tick.
    #[test]
    fn prepass_matches_fused_refresh() {
        twin_guard!();
        let build = |prepass: bool| -> NativeTick {
            let mut nt = NativeTick::new(0xC0FFEE);
            nt.set_prepass(prepass);
            for i in 0..96 {
                let mut m = MobAI::new(
                    1000 + i,
                    0xC0FFEE ^ (i as u64).wrapping_mul(0x9E37),
                    (i % 12) as f64 * 3.0,
                    64.0,
                    (i / 12) as f64 * 3.0,
                );
                m.goals = swarm_sim::mob::zombie_table_res();
                nt.add_mob(m);
            }
            nt.push_players(vec![PlayerTick {
                x: 12.0,
                y: 64.0,
                z: 4.0,
                id: 1,
                flags: 1,
            }]);
            nt
        };
        let mut pre = build(true);
        let mut fus = build(false);
        let dump = |nt: &NativeTick| -> Vec<u64> {
            let mut h = 0xcbf2_9ce4_8422_2325u64;
            let mut pop = 0u64;
            for s in &nt.store.mobs {
                if !s.alive {
                    continue;
                }
                pop += 1;
                let m = &s.mob;
                for w in [
                    m.id as u64,
                    m.x.to_bits(),
                    m.y.to_bits(),
                    m.z.to_bits(),
                    m.seed,
                    m.yaw.to_bits() as u64,
                    m.dx.to_bits(),
                    m.dy.to_bits(),
                    m.dz.to_bits(),
                    m.nav_state as u64,
                    m.nav_x.to_bits(),
                    m.nav_z.to_bits(),
                    m.target_player as u64,
                    m.no_action as u64,
                    m.tick_count as u64,
                    m.approach_x.to_bits(),
                    m.approach_z.to_bits(),
                ] {
                    h = (h ^ w).wrapping_mul(0x1000_0000_01b3);
                }
                for g in &m.goals {
                    h = (h ^ (g.running as u64)).wrapping_mul(0x1000_0000_01b3);
                    h = (h ^ (g.state as u64)).wrapping_mul(0x1000_0000_01b3);
                }
            }
            [h, nt.target_rng, pop].to_vec()
        };
        for t in 0..400 {
            pre.tick(t);
            fus.tick(t);
            assert_eq!(
                dump(&pre),
                dump(&fus),
                "tick {t}: the fused refresh diverged from the pre-pass twin"
            );
        }
        // the scenario must acquire targets (the refresh actually fires)
        let chasing = fus
            .store
            .mobs
            .iter()
            .filter(|m| m.mob.target_player >= 0)
            .count();
        assert!(chasing > 0, "the refresh twin must chase: {chasing}");
    }

    /// THE WAVE14 GATE: the PARALLEL executor (the double-buffer shadow
    /// eval + the serial stroll-stream pre-assignment + the drawer repair
    /// + the split travel + the buffer swap) must be BIT-IDENTICAL to the
    /// serial fused walk over the same chased/strolling/looking
    /// population — the whole-store dump per tick, 400 ticks, PLUS the
    /// per-tick stats stream and the executor's post-tick stroll stream.
    /// The independence theorem: every mob's result is a pure function of
    /// (its pre-state, the immutable snapshot, game_time, its assigned
    /// stream value); the assignment is a deterministic serial scan — so
    /// ANY thread interleaving lands the same words.
    #[test]
    fn parallel_executor_matches_serial_executor() {
        twin_guard!();
        let build = |parallel: bool, px: f64, pz: f64| -> NativeTick {
            let mut nt = NativeTick::new(0xC0FFEE);
            nt.set_parallel(parallel);
            for i in 0..96 {
                let mut m = MobAI::new(
                    1000 + i,
                    0xC0FFEE ^ (i as u64).wrapping_mul(0x9E37),
                    (i % 12) as f64 * 3.0,
                    64.0,
                    (i / 12) as f64 * 3.0,
                );
                m.goals = swarm_sim::mob::zombie_table_res();
                nt.add_mob(m);
            }
            nt.push_players(vec![PlayerTick {
                x: px,
                y: 64.0,
                z: pz,
                id: 1,
                flags: 1,
            }]);
            nt
        };
        let dump = |nt: &NativeTick| -> Vec<u64> {
            let mut h = 0xcbf2_9ce4_8422_2325u64;
            let mut pop = 0u64;
            for s in &nt.store.mobs {
                if !s.alive {
                    continue;
                }
                pop += 1;
                let m = &s.mob;
                for w in [
                    m.id as u64,
                    m.x.to_bits(),
                    m.y.to_bits(),
                    m.z.to_bits(),
                    m.seed,
                    m.yaw.to_bits() as u64,
                    m.dx.to_bits(),
                    m.dy.to_bits(),
                    m.dz.to_bits(),
                    m.nav_state as u64,
                    m.nav_x.to_bits(),
                    m.nav_z.to_bits(),
                    m.target_player as u64,
                    m.no_action as u64,
                    m.tick_count as u64,
                    m.approach_x.to_bits(),
                    m.approach_z.to_bits(),
                    m.et_running as u64,
                    m.ar_running as u64,
                    m.pend_x.to_bits(),
                    m.pend_z.to_bits(),
                    m.lookat_player as u64,
                ] {
                    h = (h ^ w).wrapping_mul(0x1000_0000_01b3);
                }
                for g in &m.goals {
                    h = (h ^ (g.running as u64)).wrapping_mul(0x1000_0000_01b3);
                    h = (h ^ (g.state as u64)).wrapping_mul(0x1000_0000_01b3);
                    h = (h ^ (g.kind as u64)).wrapping_mul(0x1000_0000_01b3);
                    h = (h ^ (g.prio as u64)).wrapping_mul(0x1000_0000_01b3);
                    h = (h ^ (g.flags as u64)).wrapping_mul(0x1000_0000_01b3);
                }
            }
            [h, nt.target_rng, pop].to_vec()
        };
        // arm A — the CHASED shape: the player inside the follow range (the
        // refresh/target/attack paths ride the parallel eval)
        let mut ser = build(false, 12.0, 4.0);
        let mut par = build(true, 12.0, 4.0);
        for t in 0..400 {
            let ss = ser.tick(t).to_vec();
            let ps = par.tick(t).to_vec();
            assert_eq!(ss, ps, "tick {t}: the stats stream diverged (chase)");
            assert_eq!(
                dump(&ser),
                dump(&par),
                "tick {t}: the parallel executor diverged from the serial twin (chase)"
            );
        }
        let chasing = par
            .store
            .mobs
            .iter()
            .filter(|m| m.mob.target_player >= 0)
            .count();
        assert!(chasing > 0, "the chase twin must chase: {chasing}");

        // arm B — the STROLL shape: the player far outside the follow range
        // (nobody targets -> the stroll gates fire -> the DRAWER path — the
        // stream pre-assignment + the repair pass — is exercised HARD)
        let mut ser = build(false, 100000.0, 100000.0);
        let mut par = build(true, 100000.0, 100000.0);
        for t in 0..400 {
            let ss = ser.tick(t).to_vec();
            let ps = par.tick(t).to_vec();
            assert_eq!(ss, ps, "tick {t}: the stats stream diverged (stroll)");
            assert_eq!(
                dump(&ser),
                dump(&par),
                "tick {t}: the parallel executor diverged from the serial twin (stroll)"
            );
        }
        assert_ne!(
            par.target_rng, 0xC0FFEE,
            "the stroll twin must draw (the pre-assignment path must be live)"
        );
        let strolling = par
            .store
            .mobs
            .iter()
            .filter(|m| m.mob.nav_state != 0)
            .count();
        assert!(strolling > 0, "the stroll twin must stroll: {strolling}");
    }

    /// THE WAVE11 executor gate: the STRAIGHT-LINE zombie walk (ai_floor_z0
    /// behind the fp/running/even/fold gate) must be BIT-IDENTICAL to the
    /// generic engine over the same chased/strolling/looking population —
    /// the whole-store dump per tick, 400 ticks. The straight arm engages
    /// on round 0 of every eligible tick; parks resume through the generic
    /// arm — both arms must land the same words every tick.
    #[test]
    fn zstrait_matches_generic_executor() {
        twin_guard!();
        let build = || -> NativeTick {
            let mut nt = NativeTick::new(0xC0FFEE);
            for i in 0..96 {
                let mut m = MobAI::new(
                    1000 + i,
                    0xC0FFEE ^ (i as u64).wrapping_mul(0x9E37),
                    (i % 12) as f64 * 3.0,
                    64.0,
                    (i / 12) as f64 * 3.0,
                );
                m.goals = swarm_sim::mob::zombie_table_res();
                nt.add_mob(m);
            }
            nt.push_players(vec![PlayerTick {
                x: 12.0,
                y: 64.0,
                z: 4.0,
                id: 1,
                flags: 1,
            }]);
            nt
        };
        let dump = |nt: &NativeTick| -> Vec<u64> {
            let mut h = 0xcbf2_9ce4_8422_2325u64;
            let mut pop = 0u64;
            for s in &nt.store.mobs {
                if !s.alive {
                    continue;
                }
                pop += 1;
                let m = &s.mob;
                for w in [
                    m.id as u64,
                    m.x.to_bits(),
                    m.y.to_bits(),
                    m.z.to_bits(),
                    m.seed,
                    m.yaw.to_bits() as u64,
                    m.dx.to_bits(),
                    m.dy.to_bits(),
                    m.dz.to_bits(),
                    m.nav_state as u64,
                    m.nav_x.to_bits(),
                    m.nav_z.to_bits(),
                    m.target_player as u64,
                    m.no_action as u64,
                    m.tick_count as u64,
                    m.rel_x.to_bits(),
                    m.rel_z.to_bits(),
                    m.pend_x.to_bits(),
                    m.pend_z.to_bits(),
                    m.approach_x.to_bits(),
                    m.approach_z.to_bits(),
                ] {
                    h = (h ^ w).wrapping_mul(0x1000_0000_01b3);
                }
                for g in &m.goals {
                    h = (h ^ (g.running as u64)).wrapping_mul(0x1000_0000_01b3);
                    h = (h ^ (g.state as u64)).wrapping_mul(0x1000_0000_01b3);
                }
            }
            [h, nt.target_rng, pop].to_vec()
        };
        // the arms run SEQUENTIALLY (the z0 switch is a process-global);
        // the executor is deterministic per seed (the determinism gate)
        let mut runs: Vec<Vec<Vec<u64>>> = Vec::new();
        for z0 in [true, false] {
            swarm_sim::mob::set_z0(z0);
            let mut nt = build();
            let mut dumps = Vec::with_capacity(400);
            for t in 0..400 {
                nt.tick(t);
                dumps.push(dump(&nt));
            }
            runs.push(dumps);
        }
        swarm_sim::mob::set_z0(true);
        assert_eq!(
            runs[0], runs[1],
            "the straight-line walk diverged from the generic twin"
        );
        // the scenario must actually chase (the z0 target path is exercised)
        let mut nt = build();
        for t in 0..120 {
            nt.tick(t);
        }
        let chasing = nt
            .store
            .mobs
            .iter()
            .filter(|m| m.mob.target_player >= 0)
            .count();
        assert!(chasing > 0, "the zstrait twin must acquire targets");
    }

    /// THE WAVE10 executor gate: the odd-tick no-op SKIP (the maintained
    /// every-tick bit) must be BIT-IDENTICAL to the full-floor walk over the
    /// same chased/strolling/looking population — the whole-store dump per
    /// tick, 400 ticks. The skip arm returns from ai_tick before the floor
    /// on every odd tick where no requiresUpdateEveryTick goal runs; any
    /// et_running bit drift diverges the dump streams and fails this gate.
    #[test]
    fn odd_skip_matches_full_floor_executor() {
        twin_guard!();
        let build = || -> NativeTick {
            let mut nt = NativeTick::new(0xC0FFEE);
            for i in 0..96 {
                let mut m = MobAI::new(
                    1000 + i,
                    0xC0FFEE ^ (i as u64).wrapping_mul(0x9E37),
                    (i % 12) as f64 * 3.0,
                    64.0,
                    (i / 12) as f64 * 3.0,
                );
                m.goals = swarm_sim::mob::zombie_table_res();
                nt.add_mob(m);
            }
            nt.push_players(vec![PlayerTick {
                x: 12.0,
                y: 64.0,
                z: 4.0,
                id: 1,
                flags: 1,
            }]);
            nt
        };
        let dump = |nt: &NativeTick| -> Vec<u64> {
            let mut h = 0xcbf2_9ce4_8422_2325u64;
            let mut pop = 0u64;
            for s in &nt.store.mobs {
                if !s.alive {
                    continue;
                }
                pop += 1;
                let m = &s.mob;
                for w in [
                    m.id as u64,
                    m.x.to_bits(),
                    m.y.to_bits(),
                    m.z.to_bits(),
                    m.seed,
                    m.yaw.to_bits() as u64,
                    m.dx.to_bits(),
                    m.dy.to_bits(),
                    m.dz.to_bits(),
                    m.nav_state as u64,
                    m.nav_x.to_bits(),
                    m.nav_z.to_bits(),
                    m.target_player as u64,
                    m.no_action as u64,
                    m.tick_count as u64,
                    m.rel_x.to_bits(),
                    m.rel_z.to_bits(),
                    m.pend_x.to_bits(),
                    m.pend_z.to_bits(),
                    m.approach_x.to_bits(),
                    m.approach_z.to_bits(),
                ] {
                    h = (h ^ w).wrapping_mul(0x1000_0000_01b3);
                }
                for g in &m.goals {
                    h = (h ^ (g.running as u64)).wrapping_mul(0x1000_0000_01b3);
                    h = (h ^ (g.state as u64)).wrapping_mul(0x1000_0000_01b3);
                }
            }
            [h, nt.target_rng, pop].to_vec()
        };
        // the arms run SEQUENTIALLY (the skip switch is a process-global):
        // the executor is deterministic per seed (the determinism gate), so
        // the per-tick dump streams must match arm-for-arm.
        let mut runs: Vec<Vec<Vec<u64>>> = Vec::new();
        for skip in [true, false] {
            swarm_sim::ledger::set_odd_skip(skip);
            let mut nt = build();
            let mut dumps = Vec::with_capacity(400);
            for t in 0..400 {
                nt.tick(t);
                dumps.push(dump(&nt));
            }
            runs.push(dumps);
        }
        swarm_sim::ledger::set_odd_skip(true);
        assert_eq!(
            runs[0], runs[1],
            "the odd-skip walk diverged from the full-floor twin"
        );
        // the scenario must actually chase (the melee every-tick path is
        // exercised: chasing mobs keep their odd-tick floor)
        let mut nt = build();
        for t in 0..120 {
            nt.tick(t);
        }
        let chasing = nt
            .store
            .mobs
            .iter()
            .filter(|m| m.mob.target_player >= 0)
            .count();
        assert!(chasing > 0, "the odd-skip twin must acquire targets");
    }

    /// THE WAVE6 executor gate: the FOLDED executor (find_target/LOS/stroll/
    /// RLA-start/cc resolved IN PLACE, zero park round-trips) must be
    /// BIT-IDENTICAL to the PARKED executor (every request staged + resumed —
    /// the pre-fold semantics) over the same chased/strolling/looking
    /// population: positions, seeds, goal states, nav, targets — the whole
    /// store, word for word, 400 ticks.
    #[test]
    fn folded_executor_matches_parked_executor() {
        twin_guard!();
        let build = |fold_off: bool| -> NativeTick {
            let mut nt = NativeTick::new(0xC0FFEE);
            nt.set_fold_off(fold_off);
            for i in 0..96 {
                let mut m = MobAI::new(
                    1000 + i,
                    0xC0FFEE ^ (i as u64).wrapping_mul(0x9E37),
                    (i % 12) as f64 * 3.0,
                    64.0,
                    (i / 12) as f64 * 3.0,
                );
                m.goals = swarm_sim::mob::zombie_table_res();
                nt.add_mob(m);
            }
            nt.push_players(vec![PlayerTick {
                x: 12.0,
                y: 64.0,
                z: 4.0,
                id: 1,
                flags: 1,
            }]);
            nt
        };
        let mut parked = build(true);
        let mut folded = build(false);
        let dump = |nt: &NativeTick| -> Vec<u64> {
            let mut h = 0xcbf2_9ce4_8422_2325u64;
            let mut pop = 0u64;
            for s in &nt.store.mobs {
                if !s.alive {
                    continue;
                }
                pop += 1;
                let m = &s.mob;
                for w in [
                    m.id as u64,
                    m.x.to_bits(),
                    m.y.to_bits(),
                    m.z.to_bits(),
                    m.seed,
                    m.yaw.to_bits() as u64,
                    m.dx.to_bits(),
                    m.dy.to_bits(),
                    m.dz.to_bits(),
                    m.nav_state as u64,
                    m.target_player as u64,
                    m.no_action as u64,
                    m.tick_count as u64,
                    m.rel_x.to_bits(),
                    m.rel_z.to_bits(),
                    m.nav_x.to_bits(),
                    m.nav_z.to_bits(),
                    m.pend_x.to_bits(),
                    m.pend_z.to_bits(),
                ] {
                    h = (h ^ w).wrapping_mul(0x1000_0000_01b3);
                }
                for g in &m.goals {
                    h = (h ^ (g.running as u64)).wrapping_mul(0x1000_0000_01b3);
                    h = (h ^ (g.state as u64)).wrapping_mul(0x1000_0000_01b3);
                }
            }
            let _ = pop;
            [h, nt.target_rng, pop].to_vec()
        };
        for t in 0..400 {
            parked.tick(t);
            folded.tick(t);
            assert_eq!(
                dump(&parked),
                dump(&folded),
                "tick {t}: the folded executor diverged from the parked twin"
            );
        }
        // the scenario must exercise the folds: goals running + targets held
        let chasing = folded
            .store
            .mobs
            .iter()
            .filter(|m| m.mob.target_player >= 0)
            .count();
        let running = folded
            .store
            .mobs
            .iter()
            .filter(|m| m.mob.goals.iter().any(|g| g.running))
            .count();
        assert!(chasing > 0, "the folded twin must acquire targets: {chasing}");
        assert!(running > 0, "the folded twin must run goals: {running}");
    }
}
