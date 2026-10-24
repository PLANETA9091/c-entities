# 06 — Module v0.2 results: A/B benchmark on a live Paper 1.21.8 server

Date: 2026-10-24. Machine: shared 4-core sandbox (4 GB RAM, -Xmx2G, G1GC).
Every number is a full server run of the `ce-baseline` harness: boot → fill
(400 ticks) → warmup → measure → `baseline.json` → shutdown. `off` = vanilla
Paper 1.21.8, no agent. `on` = the same server with the CRUSSTY runtime
agent + the `entities` module (weave + JNI + Rust index), armed after boot.

## Headline (ms per tick, mean / p50)

| scenario   | off           | on            | Δ mean | Δ p50  |
|------------|---------------|---------------|--------|--------|
| control    | 0.290 / 0.233 | 0.234 / 0.206 | ~0 (noise) | ~0 |
| items-100k | 86.7 / 78.0   | 86.9 / 74.2   | +0.2 % (parity) | −5 % |
| mobs-25k   | 942.5 / 918.7 | 591.1 / 563.4 | **−37.3 %** | **−38.7 %** |
| mixed-150k | 1386.9 / 1380.7 | 854.7 / 791.5 | **−38.4 %** | **−42.7 %** |

Correctness: **2080 differential checks, 0 mismatches**. During every run's
warmup the harness executes random AABB queries twice — once through the
vanilla body (direct EntityLookup call), once through the woven bridge — and
compares the full entity-id sets. Both `getEntities` and `getEntitiesOfClass`
are covered. No tick is skipped, no entity is frozen: the module only
re-implements the *query* (`Level.getEntities`, `Level.getEntitiesOfClass`)
and mirrors vanilla exactly otherwise.

## Where the win comes from

1. **The index** (Rust, `centities-core` v0.2): 16-block XZ hash grid,
   array-of-structs packed cells (32 B/entry, one cache stream per cell),
   cell-relative f32 coordinates (exact within ±30 M blocks), per-world
   max-half-span tracking, exact-with-margin filtering (no false negatives,
   margin FPs vanish in the Java recheck).
2. **The weave**: `Level.getEntities(Entity,AABB,Predicate)` (push path —
   `getPushableEntities` routes through it) and
   `Level.getEntitiesOfClass(Class,AABB,Predicate)` (item merge, sensor
   scans) are replaced by straight-line bridge calls; mutation probes sit on
   `EntityLookup.addEntity/removeEntity` and `Entity.setPos/setBoundingBox`.
3. **Cost-aware fast path**: the native reply carries `[candidate count,
   entries walked]`. Wide sensor boxes in dense packs (thousands of true
   candidates) fall back to the vanilla slice walk — a Java-side id-resolve
   of that many candidates would lose. Population pre-estimate
   (`query_population`, O(cells)) skips the native walk itself for huge
   dense queries (marker reply −1). In mobs-25k: 11.8 M fast + 3.7 M
   correctly-fallback queries; mixed-150k: 15.7 M fast + 1.6 M fallback.

## What did NOT work (kept or reverted)

* Java-side setPos dedup (open-addressed long arrays): hit rate measured at
  **0 %** — vanilla items never call `setPos` without real movement. Kept
  (harmless, ~8 MB, protects static entities).
* Replacing bodies via the SDK's ASM helper: `defineClass` inside a
  ClassFileLoadHook during a retransform deadlocks the VM (class-loader lock
  held by a safepointed thread). All Level swapping is now pure Rust with a
  branch-free body — no StackMapTable needed at all.
* Retransforming before `MinecraftServer.isReady()`: deadlocks mid-boot
  (worldgen threads + class loading + safepoint). The module now waits for
  `isReady`, bootstraps the index from live world state (`EntityLookup.getAll()`),
  arms the probes, then swaps the queries — no stale window.

## Safety contract (unchanged)

* Kill switch at every level: patch failure → class untouched; any bridge
  exception → `ACTIVE=false` → vanilla mirror; desync (all candidates stale)
  → mirror for that query.
* False positives are filtered by the exact f64 recheck in Java; false
  negatives are impossible by construction (expansion ≥ max half-span +
  quantization margin) and are asserted by property tests and the live
  differential harness.

## Reproduce

```bash
bash scripts/run_ab.sh control     off ab/results.csv
bash scripts/run_ab.sh mobs-25k    on  ab/results.csv   # -37 %
bash scripts/run_ab.sh mixed-150k  on  ab/results.csv   # -38 %
```

Scenario knobs mirror `.github/workflows/baseline.yml`; measure/warmup
windows are shorter locally (shared box, 570 s cap per run).
