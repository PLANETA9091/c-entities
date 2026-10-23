# 19 — L8: rayon-parallel flush + the merge-parity investigation

Owner directive (binding): the tick is COMPUTED in Rust, at maximum speed.
L8 lands the declared next lever (parallel flush) and closes the
merge-parity question with instrumentation instead of theory.

## 1. The merge-parity investigation (docs 18 open item)

Symptom (items-100k CI): vanilla entity count plateaus at 63,050 and never
moves again; the mod drains 21,411 → 5,461 over the measured window. Doc 18
hypothesized "the fast path absorbs more neighbours per scan" + "1.5M stale
arena candidates".

### What was verified against the shipped 1.21.8-60 jar (javap)

* `mergeWithNeighbours()` — the scan loop CONTINUES until `this` is removed;
  there is no one-merge-per-scan in vanilla. Doc 18's proposed fix was wrong.
* `tryToMerge(other)` — direction: `other.count < this.count` → this absorbs
  other; else OTHER absorbs THIS (equal counts kill the scanner). Any
  absorption chain therefore converges to "the biggest candidate eats the
  rest" regardless of candidate ORDER — the pile equilibrium is
  order-INDEPENDENT. The Rust index's hash order cannot change it.
* The vanilla MOVE GATE exists and was ported exactly: a resting item
  (onGround && horiz² ≤ 1e-5f32 && (tickCount+id)%4 ≠ 0) skips move + friction
  + bounce and accumulates dy = −0.04·k until its gate tick zeroes it —
  the core's trace test is bit-exact against the bytecode, including the
  −0.04·k oscillation. The `maxD4e-2` in the flush log is THIS oscillation,
  not drift.
* Constants: GATE_HORZ_SQR = (1e-5f32) as f64 = 9.999999747378752E-6 ✓,
  Y_DRAG = 0.98 (double literal) ✓, air friction = 0.98f32 ✓, gravity 0.04 ✓.
* `getOnPos(0.999999f)` (friction block) — the mainSupportingBlockPos branch
  is ported exactly (fences route vanilla).
* `EntityCallback.onRemove → EntityLookup.removeEntity` — EVERY removal
  (merge discards included) fires the probe prefix; probeRemove clears the
  ARENA slot synchronously, so a removed entity can never resolve as a merge
  candidate. The 1.5M stale candidates are the normal probe-batch window,
  correctly SKIPPED (not merged).

### Conclusion

Every per-scan semantic and the candidate SET are vanilla-exact. What the
run cannot show is whether MASS is conserved — the bench counts ENTITIES,
not stacks. If mass is conserved, the count gap is pure merge dynamics
(order/chaining across 40-tick waves); if not, something duplicates or eats
items. That question now gets answered by measurement:

* `BaselinePlugin` samples now carry `item_mass` (sum of stack counts of all
  ItemEntities) and `item_count` next to `entities`. Mass is the hard
  vanilla-compat gate: entity counts may diverge through merge order, mass
  may not.
* `CeHooks.mergeOps` counts successful tryToMerge operations (either side
  discarded) and is printed in the hooks line (`merge=.../ops`). Next run:
  compare per-window merge rates vanilla-expected (live/40 per tick) vs
  actual, with mass as the conservation reference.

## 2. L8 — the rayon-parallel flush

The flush was the last single-threaded floor of the native core
(~2.4ms/flush at n≈5.3k, ≈215ns/item). The item loop shards cleanly by id:
every item's tick is independent, and the flush output is indexed.

### Thread-safe section cache (centities-core/sections.rs)

`SectionGrid` no longer mutates on read:

* 16 shards of `RwLock<HashMap<u64, Arc<SectionData>>>` (fast u64 hasher —
  keys are already-mixed sec_keys);
* READS: shard read-lock only to clone an Arc — the block scan runs
  LOCK-FREE on the immutable Arc payload;
* MISS: the provider up-call runs with NO lock held (a JNI call must never
  sit on a mutex other workers need), then one write-lock insert with a
  double-check (a concurrent hydrate wins cheaply via the entry API);
* invalidation (setBlockState probe) takes one shard write-lock;
* `populated`/`lookups` became AtomicU64.

Determinism: a published section payload is immutable — block changes
re-publish a NEW Arc — so a gather never observes torn data. The Java
provider is off-thread-safe (`getChunkNow`, no load for gather paths; the
server thread is blocked inside the flush scope, so no concurrent block
mutation can race the reads; `load=true` paths attach and may block on a
chunk load exactly like vanilla's synchronous getBlockState).

### Three-phase flush (centities-module/itemcore.rs)

* **Phase 1 (sequential):** parse the batch + hydrate persistent parts from
  the store. The store is touched only here and in phase 3. Resync/force
  entries are ingested here (unchanged semantics), no tick.
* **Phase 2 (rayon):** chunks of 512 items across the pool. Per chunk: own
  `Scratch` + own section provider (`with_attached` per up-call — rayon
  workers attach once and stay attached). Outputs (od/oi/commit-records)
  are written by disjoint per-chunk slots — bit-identical results
  regardless of scheduling.
* **Phase 3 (sequential):** commit tick results into the store in batch
  order (== vanilla tick order), the abort-seatbelt counters, despawn-rate
  refresh for collector entries, and the diagnostic fold (state extremes
  from the writeback buffers).

Kill switch: `CE_FLUSH_PARALLEL=0` falls back to... nothing special — the
same rayon driver runs one chunk (the parallel driver degenerates to the
sequential loop with ~2µs of pool overhead; small batches are far below the
JNI+flush toll anyway).

Removed: `ItemWorld.scratch` (per-chunk scratch now), the dead `SecMap`
(open-addressed map) — the sharded cache replaces it.

### What Java still pays per item

Unchanged: the dispatch guard (~40–70ns), the applier writeback (~60–100ns),
the merge scans (L3 fast path), the 256-tick belt.

### Verification

* centities-core: 8 trace/property tests + 9 exactness property tests green
  (the exactness suite re-runs fully against the sharded grid).
* centities-module: patcher tests on the real Level.class green.
* CI: module build + baseline A/B matrix (items-100k paired legs) — watch
  the flush `avg` line: expect ~2.4ms → ~0.8–1.2ms at n≈5.3k on 4 vCPU, and
  RSS flat (the sharded cache holds the same sections).

## 3. Hygiene found on the way

* `module.yml`/`baseline.yml`/`e2e.yml` push filters: cosmetic output
  artifact only — the filters are `[main]` (shell output strips `[m`);
  verified byte-level.
* CI clippy is not gated for the module (pre-existing raw-pointer JNI
  pattern in the extern wrappers); the core is warning-clean of new issues.

## Open items

* Merge parity: DECIDE by measurement (item_mass + mergeOps, first CI run).
* Java floor levers: tracker (ChunkMap$TrackedEntity.updatePlayer ~3.9%),
  EntityTickList.forEach (~2.5%), NaturalSpawner.createState — separate
  weaves, each vanilla-faithful, each behind its own kill switch.


## 4. The abort storm — root cause found and fixed (post-first-CI)

The first L8 CI leg froze the world (mass pinned at 100000, `maxD=0e0`, zero
merges): every item dispatch-owned and natively ticked, yet no commit ever
reached Java. Three compounding defects, all fixed:

1. **The writeback bit contract was shifted** (Rust ABORTED=1<<10 /
   VANILLA_NEXT=1<<9 / LAND=1<<11 vs the applier's 1<<13 / 1<<11 / 1<<9).
   An aborted item was applied as a normal one — the applier wrote zero
   deltas/age/pickupDelay/onGround into the entity every tick. The
   constants now mirror the Java applier exactly, and a property test
   (`flush_zip`) proves the chunk-zip commits every tickable item without
   slot crossing.
2. **The L7 abort seatbelt was lost in the three-phase split** — the abort
   streak is carried (phase 1 → tick → phase 3) again; 4 consecutive aborts
   route the item vanilla.
3. **THE ~48% ABORT STORM — `overrides()` anchored at the wrong base.**
   In 1.21.8 `stepOn` / `fallOn` / `updateEntityMovementAfterFallOn` MOVED
   from `BlockBehaviour` into the `Block` base class (javap-verified). The
   walk-to-BlockBehaviour check therefore matched EVERY block:
   `fall_on_special=1` everywhere, and the tick core's swept-gather gate
   (`fall_special -> ABORTED`) aborted every item whose sweep touched any
   block — i.e. every resting item, every tick. The anchor is now `Block`
   (the defaults' new home); only genuine overriders below Block (slime,
   honey, cobweb, ...) flag.

**Consequence for the L7 verdict:** the same storm existed before L8 — the
old seatbelt cycled every item 4-aborts-then-vanilla, i.e. 4 of 5 ticks the
item was FROZEN (the forbidden "not tick"), and the first CI leg of this
series (ok/ab counters) exposed it. The −86.7% p50 in doc 18 was measured
against that behavior. With the anchor fixed the core computes REAL native
physics every tick; the next paired A/B is the first honest measurement of
the native core doing actual work (plus the correct LAND events, which the
shifted bit had silently dropped).
