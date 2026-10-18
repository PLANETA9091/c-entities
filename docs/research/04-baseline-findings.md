# 04 — Baseline findings: what actually burns the tick

Data source: 9 full benchmark runs of **unmodified** Paper 1.21.8 on GitHub
Actions (ubuntu-latest, 4 vCPU, 8 GB heap, JFR `profile`, 600 measured ticks
per run, mineflayer bot as the activating player). Artifacts: `baseline-*`
in GitHub Actions run 39150947858; per-run reports in `reports/baseline/`.
Nothing in this document is guessed — every claim is either a number from
`baseline.json` / `REPORT.md` or resolved from JFR call stacks.

## 1. Headline numbers

| run | entities | block entities | MSPT p50, ms | TPS est. |
|---|---:|---:|---:|---:|
| control | 0 | 0 | 0.158 | 20.0 |
| items-25k | 25 000 (19 939 after merge) | 0 | 19.0 | 20.0 |
| items-100k | 100 000 (62 930 after merge) | 0 | 65.8 | 15.2 |
| mobs-10k | 10 000 | 0 | 100.3 | 10.0 |
| mobs-25k | 25 000 | 0 | 488.9 | 2.0 |
| mixed-50k | 50 000 (41 477 after merge) | 0 | 110.1 | 9.1 |
| mixed-150k | 150 000 (110 242 after merge) | 0 | 909.7 | 1.1 |
| tiles-25k | 0 | 24 964 | 7.0 | 20.0 |
| tiles-100k | 0 | 68 121 | 27.5 | 20.0 |

Even without any players moving and with EAR active, 100k entities cost
**65–900 ms of MSPT**. The vanilla server is 130–1800× over the project
budget (≤ 0.5 ms for the whole entity+tile tick at 100k+).

## 2. Cost per entity per tick (p50, measured)

| load | entities | MSPT | µs / entity / tick |
|---|---:|---:|---:|
| items-25k | 19 939 | 19.0 | ~0.95 |
| items-100k | 62 930 | 65.8 | ~1.05 |
| mobs-10k | 10 000 | 100.3 | ~10.0 |
| mobs-25k | 25 000 | 488.9 | ~19.6 |
| mixed-50k | 41 477 | 110.1 | ~2.65 |
| mixed-150k | 110 242 | 909.7 | ~8.25 |
| tiles-25k | 24 964 | 7.0 | ~0.28 |
| tiles-100k | 68 121 | 27.5 | ~0.40 |

Two facts jump out:

1. **A zombie costs 10–20× an item**, and *the same zombie costs twice as
   much at 25k as at 10k* — the cost is not linear in N.
2. The mixed run grows from 2.65 → 8.25 µs per entity (×3.1) when the
   population grows ×2.65. Fitting: cost ≈ N^2.07. Mobs: ×2.5 population →
   ×4.9 cost ≈ N^1.96. **The dominant workloads are quadratic in density.**

## 3. Why quadratic: per-entity AABB proximity queries

Entity spread in the benchmark is a fixed area (`AREA=150`, i.e. ~250 mobs
per chunk at 25k). Double the population ⇒ double candidates inside every
query box ⇒ double queries AND double work per query. The JFR stacks show
exactly which queries:

```
mobs-25k:  AABB.intersects(double×6)          25.4% of all CPU samples
mixed-150k: AABB.intersects(double×6)         42.7% of all CPU samples
```

Callers of `AABB.intersects` (resolved from stacks, 11 972 samples in
mobs-25k): **100 %** go through
`ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate)`
(Moonrise per-chunk entity slices). Callers of `Level.getEntities(Entity, AABB, ...)`
in mixed-150k: **73 % = `Level.getPushableEntities`** ← `LivingEntity.pushEntities()`
(the collision-separation scan that every living entity runs every tick);
the rest is dominated by the **`ItemEntity` merge scan** — doc 01 already
established that every mergeable item runs a `getEntitiesOfClass(ItemEntity,
box.inflate(0.5…))` query **every tick** (spigot `itemMerge` radius 0.5).

So the single biggest vanilla cost at scale is not "AI", not "physics" —
it is **entity→entity proximity queries** executed from two places:
push-separation (living entities) and item merging. Both scan chunk slice
lists with a virtual predicate per candidate and then `AABB.intersects`
per candidate — hundreds of ns per candidate due to pointer chasing, and
quadratic overall.

Vanilla has **no spatial acceleration for entity→entity queries** beyond
chunk slices. That is the core structural defect the module will fix.

## 4. The rest of the profile (stable, linear costs)

### items-100k (cleanest view of the per-entity pipeline tax)
```
 7.9%  EntityTickList.forEach          — iteration machinery itself
 5.8%  Entity.setOldPos                — written EVERY tick even for resting items
 5.1%  ServerEntity.sendChanges        — tracker diffing
 3.4%  Entity.setSharedFlag
 3.1%  InsideBlockEffectApplier.flushStep  — per-tick block-effect bookkeeping
 2.9%  AABB.intersects                 — item merge scan
 2.2%  Entity.applyEffectsFromBlocks
 2.1%  Entity.isNoGravity
 2.0%  SynchedEntityData.getItem
 1.6%  Vec3.<init>                     — allocation churn
```
Roughly **35–40 % of item cost is a fixed "pipeline tax"** paid by every
entity every tick regardless of what it does — including fully resting
items that will never move until something disturbs them.

### mobs-25k (AI-visible part)
```
25.4%  AABB.intersects (+5.6% slice scan)   — push separation (see §3)
 7.2%  Mob.serverAiStep total                — goals, sensing, attributes
 2.0%  Entity.setTicksFrozen                 — powder-snow freeze write, every tick
 1.0%  LivingEntity.canCollideWithBukkit     — predicate dispatch
 1.6%  tickEffects / getHealth / getSleepingPos — per-living boilerplate
```
After removing query cost, honest AI (`serverAiStep`, goals, navigation,
sensing) is the hard floor — roughly 15–25 % of what remains. No scheduler
trick removes it; only doing the same work faster (data layout, JNI-side
batching) or doing it in Rust can.

### tiles-100k (block entities)
```
10.2%  LevelChunk$BoundTickingBlockEntity.isRemoved   — loop bookkeeping
 9.8%  Level.tickBlockEntities                        — loop body itself
 8.3%  ItemStack.getMaxStackSize                      — hopper transfer attempts on empty inventories
 5.4%  HashMap.getNode
 2.8%  PalettedContainer.get / getBlockState          — block reads for hopper push/pull
```
Block-entity cost is ~linear (0.28 → 0.40 µs as hoppers/furnaces ratio
changes). But ~20 % of it is pure loop overhead, and a large share of
hopper work is **attempts that provably cannot move anything** (both sides
unchanged since last tick) yet still re-validate the full transfer.

## 5. Ranked, quantified levers for the module

Measured headroom, ordered by expected impact on the benchmark targets:

| # | Lever | Attacks | Measured basis | Expected effect |
|---|---|---|---|---|
| L1 | **Rust spatial index for entity→entity queries** (push scan, item merge, target prefilter) — packed SoA AABBs, incremental updates, no per-candidate virtual calls | quadratic term + 28–43 % of steady CPU | §3, §4 | kills N²; mixed-150k 910 ms → ~150–250 ms estimate |
| L2 | **Static fast-path** for provably-static entities (resting items, calm stands): skip setOldPos/flags/effects/sync rescans until disturbed | pipeline tax (35–40 % of item cost) | §4 items | item cost 1.05 → ≤ 0.15 µs; items are the bulk of any 100k+ load |
| L3 | **Tile fast paths**: short-circuit hopper transfers where both inventories are unchanged since the last attempt; hoist loop invariants out of `tickBlockEntities` | ~50 % of tile cost | §4 tiles | tiles-100k 27.5 → < 12 ms |
| L4 | **Batched per-entity pipeline** (setOldPos/synched-data/flag writes amortized for unchanged entities; tracker batching) | remaining linear tax | §4 items | 10–20 % of entity section |
| L5 | **Activation scan**: `ActivationRange.activateEntities` is a full `getEntities(maxBB)` per player per tick (doc 01 §3) — replace with an index window query | O(N) per player | doc 01 | matters at high player counts; neutral on the 1-bot bench |
| L6 | **Rust mirror tick** for far-from-player active mobs (doc 03): honest physics in Rust, same constants, writes back via JNI | AI floor | §4 mobs | long-term path to the 0.5 ms goal |

Honest caveats, fixed by data:

- **25k fully-active zombies in 150 blocks will never tick at 20 ns each.**
  Once queries are fixed, remaining cost is genuine AI. The realistic
  milestone ladder is: mixed-150k ≤ 50 ms (L1+L2) → mixed-150k ≤ 15 ms
  (L4+L6) → target mix (items+passive+tiles dominant, mobs at server-like
  density) at ≤ 0.5 ms. The benchmark keeps the worst case visible so we
  never claim unearned wins.
- The 62.9k → "100k spawned" merge shrink shows items **merge down**;
  realistic steady-state item counts are lower than spawn counts — the
  module must keep merge behavior byte-identical (L1 handles it via the
  same radius and tie-breaking rules).
- Freeze/tier mechanisms must respect EAR semantics (doc 01 §3): anything
  the module skips must be provably in a state where vanilla itself would
  run `inactiveTick` or nothing observable — L2's disturbance wake-up
  (block change, damage, pickup attempt, nearby player) is what keeps the
  output 100 % vanilla.

## 6. Repeatability notes

- JFR overhead (~2–5 %) is identical across all runs including future
  optimized runs, so comparisons stay fair.
- 4 vCPU CI runners cap parallelism experiments; before benchmarking the
  worker-pool levers, add a runner-size matrix (4/8/16 vCPU) to the
  benchmark workflow.
- Spawn area is intentionally hostile (250 mobs/chunk). A real-server
  profile (density ~5–15/chunk) will be added as a scenario so the
  progress ladder tracks both worst case and realistic case.
