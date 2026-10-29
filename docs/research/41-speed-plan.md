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

## Ranked next moves

1. **Merge-scan pre-resolved predicate (doc-30's redesign, items +
   mixed + spread legs)**. `tryToMerge`'s conditions split into a
   STATIC per-item-type part (maxStackSize, damage>0, enchantments/
   component class — all type-level constants) and a DYNAMIC part
   (count comparison, age/pickup). The static part is pre-resolved per
   item TYPE into the kernel's mirror (a flag byte per entity, stamped
   by the existing probe path — zero JNI at scan time); the dynamic
   count compare moves into the Rust scan. The MH predicate stays the
   fallback for any item whose static resolution is uncertain
   (components present, non-vanilla stack). Expected: -8.3% direct
   (getMaxStackSize) plus a cut of fastQuery's 14% (the predicate's
   MH dispatch and per-pair Java work).
   PARITY CONTRACT: the merge scan order and the survivor semantics
   are PROVEN green (4242a2d + 5194968) — the specialization must
   select the SAME survivor in the SAME order; the pre-resolved bit is
   only allowed to replace the per-pair PREDICATE EVALUATION, never to
   reorder or re-pair.
2. **fastQuery row-path tightening (items/mixed)**. The row path pays
   a per-candidate live-box filter + the exact AABB re-check in Java
   for every merge scan. Two cheap cuts: (a) skip the f32-prefilter
   double-filter when the row was published from the same-tick boxes
   (the pending revision check already exists — reuse it as a skip
   bit); (b) hoist the Entity.getId/hash lookups out of the per-row
   loop (4.5% Entity.getId rides partly on this path).
3. **The vanilla-AI long tail (mobs)**: GoalSelector.tick + tickRunning
   (~2.4%) — the L15 microkernel is shipped; the remaining mass is the
   vanilla goal BODY. The pre-resolved-predicate goal redesign (doc-30
   item 4) only pays after 1-2 land.
4. **AttributeMap.getValue/getInstance (~1.1-1.2% mobs/mixed)** — the
   movement floors re-read attributes per tick; a per-tick attribute
   snapshot in the existing mob mirror (stamped at ingest, refreshed
   on updateAttributes) removes the Map lookups.
5. **Unsafe.getObject 2.1-2.9%** — the arena ARENA[id&AMASK] /
   ORD_SEQ reads inside probes. A collapsed long-keyed table (id →
   packed (slot|rev)) could halve the probe cost; measure first — the
   frames may be the publish path's ROW writes, which are already
   minimal.

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
