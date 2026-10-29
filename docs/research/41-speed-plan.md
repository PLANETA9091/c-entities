# 41. The -90% grind — fresh profiles and the ranked next moves

Date: 2026-10-29. Status: PLAN (profiles measured on 443b8dc baseline +
721c5de profile runs; the parity fix 5194968 rides in parallel).

## Where the four scenarios stand (baseline 40795889686, p50)

| scenario | vanilla | mod | delta | target (-90%) |
|---|---:|---:|---:|---:|
| mobs-25k | 509.2 | 190.7 | -62.6% | 50.9 |
| mixed-150k | 693.4 | 412.2 | -40.6% | 69.3 |
| items-100k | 58.9 | 24.4 | -58.6% | 5.9 |
| spread-100k | 322.1 | 150.8 | -53.2% | 32.2 |

Context for the doc-30-era comparison (L20b 152.9 / L22 129.5 on
mobs-25k): the parity hunt legitimately disabled or exactified several
floors — L21 rest dispatch (d35a309), crammed-push routing to vanilla
(1bc786a), the exact f64 AABB re-check (9f11b3c), aiFloor off
(3d298d1). Every re-gain from here must preserve the now-proven exact
semantics — no skip-ticks, no frozen entities, byte-faithful outcomes.

## The items-100k ON profile (profile run 40670289234, jfr self-time,
8724 samples; boot noise — bzip2/security provider frames ~2.5% —
dilutes but does not reorder the steady-state frames)

| % | frame | owner |
|---:|---|---|
| 13.96 | CeHooks.fastQuery | the merge scan's row-path query |
| 8.29 | ItemInstance.getMaxStackSize | the merge predicate per PAIR |
| 4.52 | Entity.getId | probe arena/hash lookups |
| 3.51 | CeHooks.itemTickFlush | the native batch flush |
| 3.01 | CeHooks.newTrackerTick$ | L30 tracker-loop floor |
| 2.60 | ChunkEntitySlices$BasicEntityList.indexOf | vanilla ticklist reads |
| 2.27 | CeHooks.tickListForEach | L13 raw walk |
| 2.11 | Unsafe.getObject | mirror/publish internals |
| 2.10 | Entity.getBoundingBox | probe path reads |
| 1.82 | AABB.intersects(DDD) | exact f64 re-check |

**~26% of the items server thread sits in THREE module-owned buckets:
fastQuery + the merge predicate + itemTickFlush.** The merge scan is
the one big lever left on items (the tracker/ticklist floors are
already landed and flat).

## The mobs-25k ON profile (baseline REPORT top-15, same structure as
mixed — no dominant bucket: top 2.99%)

Flat: AABB.intersects 3.0, Unsafe.getObject 2.9,
ChunkEntitySlices.getEntities 1.7, SetIterator 2.6 (two frames),
Entity.getId 1.6, GoalSelector.tick 1.4, SynchedEntityData 1.3,
tickListForEach 1.3, rowQuery 1.2, setTicksFrozen 1.2,
newTrackerTick 1.1, AttributeMap 1.1... The mob leg's remaining mass is
the VANILLA AI + attribute + tracker long tail — many 1-3% frames, each
needs its own floor. No single -30% move exists.

## Ranked next moves (updated 2026-10-29 after the L41 measurement)

0. **DONE — the merge max-stack pre-resolution (L41, 98483d2)**:
   `ItemInstance.getMaxStackSize` 8.29% → **0.85%** (mixed-150k profile
   40886854724). The merge predicate is no longer a top bucket.
1. **The EXACT native query (f64 boxes in the index)** — was move 2,
   promoted: the mixed-150k profile's new #1 is
   `AABB.intersects(DDD)` at **8.30%** — the L37d exact re-check of the
   native query's margin superset — with **marginExtra = 226.4M
   candidates per 300-tick measure window**. The whole remaining item
   tax is the f32 index's margin: the query returns ~5x the true
   candidate count and Java re-checks every one. Options, in order of
   preference:
   a. store f64 boxes in the query index (100k entities × 48B = 4.8MB
      — trivial) and drop the MARGIN entirely: the native query returns
      EXACTLY vanilla's set, the Java re-check disappears, and the
      marginExtra loop with its getBoundingBox calls (1.88%) goes with
      it. The f32 prefilter stays as the SIMD first pass (a range
      compare on the f32 copy, exact f64 confirm in Rust);
   b. SIMD the Java re-check (21's AVX2 pattern) — cheaper to ship,
      keeps the 5x candidate tax.
2. **fastQuery row-path tightening** — unchanged (the shared-scratch
   list for the merge-scan caller; the general getEntities keeps
   allocating).
3. **The vanilla-AI long tail (mobs)** — unchanged.
4. **AttributeMap snapshot** — unchanged.
5. **Unsafe.getObject 2.2%** — unchanged.

## Guardrails

- Every move ships with its parity knob (the CE_* env pattern) so a
  red parity gate pins the regression to the exact subsystem in one
  bisect run.
- The A/B stays honest: baseline.yml runs the SHIPPED config only; no
  leg may enable a flag the other leg doesn't see.
- The correctness floors from the parity hunt are NOT negotiable: the
  die-probe freshness discipline (5194968) is the template — a floor
  may only skip work whose OUTCOME is provably identical, and every
  mirror read must carry a freshness story (probe, version, or phase
  contract).
