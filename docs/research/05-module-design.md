# 05 — c-entities module design (v0.1), driven by the baseline data

This is the first optimization that touches the server. Scope is strictly
the lever with the largest measured headroom (findings doc 04, L1): make
**entity→entity proximity queries** fast without changing a single bit of
observable behavior. Everything else (static fast-path L2, tiles L3,
pipeline batching L4, mirror tick L6) is deliberately out of scope for
v0.1 and will land incrementally, each with its own A/B benchmark.

## 0. Hard constraints (owner requirements)

1. 100 % vanilla output: every query must return **exactly** the same
   entity set as vanilla, in a compatible way for the caller's predicate.
2. No extra RAM/CPU pressure: the index must cost less than it saves
   (budget: ≤ 64 bytes/entity for the index itself).
3. Freeze/tiering is NOT part of v0.1. No tick is skipped, ever, in v0.1.
4. Kill switch: if anything fails to weave/verify → module idles, vanilla
   paths run untouched.

## 1. What v0.1 replaces, and why it is safe

Vanilla hotspot chain (measured, doc 04 §3):

```
LivingEntity.pushEntities()  ─┐
ItemEntity.tick() merge scan ─┴─> Level.getEntities(Entity, AABB, Predicate)
                                     └> Moonrise EntityLookup.getEntities
                                          └> ChunkEntitySlices scan (25-43% of CPU, quadratic)
```

v0.1 re-implements **only the scan** (candidate discovery) natively.
The predicate — the part with real game semantics (`canCollideWith`,
merge rules) — keeps running in Java exactly as vanilla wrote it.
So even if the index returns a superset, the result set is still decided
by vanilla's own predicate. The only correctness requirement on the Rust
side is **no false negatives**: every entity whose AABB intersects the
query box must be offered to the predicate. That is an exact-geometry
requirement, which the index satisfies by construction (§2).

Injection mechanics (cplug-sdk, verified in doc 03):

- `redirect_calls`: every call site of
  `Level.getEntities(Entity, AABB, Predicate)` inside
  `LivingEntity.pushEntities()` / `ItemEntity.tick()` (and later, per
  benchmark evidence, other hot callers) is redirected to a static shim
  `CeHooks.getEntitiesFast(Level, Entity, AABB, Predicate)`.
- The shim asks Rust: `ce_index_query(world_id, minx, miny, minz, maxx,
  maxy, maxz) -> long[] entity ids` (exact, deduped), then resolves ids
  to entities in Java, applies the vanilla `Predicate`, returns the list.
  Fallback: if the index is disabled/desynced → call the original
  vanilla method (zero risk).
- Mirror maintenance hooks:
  - `EntityLookup.addEntity / removeEntity` (add/delete index entry),
  - `Entity.setPos(double,double,double)` tail (update entry; ~20 ns
    no-change early-out for statics — resting items call setPos every
    4th tick with identical coordinates),
  - `Entity.setBoundingBox(AABB)` tail (rare; teleports/refresh go
    through setPos too, but this closes the gap),
  - `ServerLevel` unload hook (bulk eviction per world stop).
- `main_thread` discipline: the index is only ever mutated from the
  server tick thread (vanilla calls these methods on the tick thread);
  reads happen only inside the same thread's query shims. No locking on
  the hot path; a seqlock counter exposes the index generation for the
  (debug-only) verifier thread.

## 2. The index (centities-core)

Exact, allocation-free, cache-ordered:

- **XZ hash grid**, cell size = 64 blocks (≥ max query box we redirect:
  push ~1.9×2.1, merge ~2.6; 64 gives ~1 cell probes for typical cases
  and bounds worst-case cell occupancy at server densities).
  Open-addressed table (`long2slot`, power-of-two, linear probing,
  lazy-deleted slots), no per-cell Vec allocations — cells own a
  slab-allocated dense array of entries (u32 slots), compacted lazily.
- Each entity entry (SoA, index = slot):
  `entity id (i64)`, `world u16`, AABB (6×f32: x0,y0,z0,x1,y1,z1),
  `next` for cell chaining. ~64 B/entity — within budget.
  f32 vs vanilla f64: query inserts a **guard margin of 0.0293**
  (≈ 2^(-15.3)·(1+|x|) is not needed — we use absolute margin 0.5 m and
  then re-test in f64 in the shim; see below), so rounding can only
  produce false positives, never misses.
- **Y filtering inside cells**: entries store y0..y1; query skips
  entries outside the Y range before touching anything else. Vertical
  queries tall boxes (e.g. 320-height selector scans) degenerate to a
  cell scan — still faster than the vanilla slice walk because entries
  are contiguous.
- **Insert/update**: setPos hook computes the new cell(s); if the cell
  set didn't change → update AABB in place (2 stores). Else remove+insert
  (slab swap-remove + chain update). Typical moving entity: 1–2 probe
  reads + 2–6 stores.
- **Query**: walk overlapped cells (≤ 4 for our redirects), iterate
  dense entry arrays, test AABB overlap in f64-converted values, emit
  ids into a thread-local scratch `long[]` reused across queries (zero
  allocation steady-state).
- **Verifier (debug builds)**: every 128 ticks, sample 1/128 of entities,
  re-derive their cell membership from Java ground truth, count
  mismatches; any mismatch > 0 → auto-disable index (kill switch) and
  log. Mirrors the "verification" principle from doc 03.

## 3. JNI surface (minimal, all primitives)

```
native long[]  ceQuery(long world, double x0,y0,z0,x1,y1,z1)   // exact, may return superset? NO: exact
native void    ceUpsert(long entity, int world, double ...9)   // id + AABB
native void    ceRemove(long entity)
native void    ceClearWorld(int world)
```

Return arrays come from a reused per-thread direct buffer exposed as a
`long[]` view (single copy), or `LongBuffer` — decision at implementation
time by microbenchmark; steady-state allocation = 0.

## 4. Validation plan (before any benchmark claim)

1. **Unit (Rust)**: exactness property test — random boxes/entities,
   index query vs brute force, 10^6 cases, zero misses tolerated.
2. **Unit (Rust)**: update storm test — 10^6 random setPos, queries stay
   exact; memory ceiling check.
3. **E2E (CI, CRUSSTY e2e harness)**: boot Paper 1.21.8 with module,
   scripted loads: spawn/move/remove storm + query storm; assert Java
   `getEntities` results identical with module on vs off (same seed).
4. **A/B benchmark**: the existing baseline workflow, same scenarios +
   `MODULE=off/on` matrix leg; publish comparison table into
   `reports/ab/`.

## 5. Out of scope for v0.1 (recorded, not forgotten)

- L2 static fast-path, L3 tile fast paths, L4 pipeline batching,
  L5 activation scan index, L6 Rust mirror tick (doc 03 architecture).
- Multi-world: index is per-world keyed from day one, but only the main
  overworld is redirected until E2E covers all dimension types.
- Folia/scheduler API: not targeted (CRUSSTY base is Paper 1.21.8).

## 6. Repo layout (from v0.1 commit on)

```
crates/centities-core/    pure Rust: index + tests (no JVM deps)
crates/centities-module/  cplug-sdk module: weave, JNI bridge, kill switch
manifest/module.json      CRUSSTY module manifest
.github/workflows/module.yml  cargo test + cargo build + e2e (later legs)
```
