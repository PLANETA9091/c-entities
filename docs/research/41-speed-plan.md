# 41. The -90% grind — fresh profiles and the ranked next moves

Date: 2026-10-31 (refresh). Status: PLAN (the ce5a471 baseline +
profile runs 41236402326 / 41236766183; the parity families gate the
grind — addendum 14's belt dust and the push-timing family).

## Where the four scenarios stand (baseline ce5a471, p50 — 2026-10-31)

| scenario | vanilla | mod | delta | target (-90%) |
|---|---:|---:|---:|---:|
| mobs-25k | 494.2 | 242.5 | -50.9% | 49.4 |
| mixed-150k | 452.0 | 227.9 | -49.6% | 45.2 |
| items-100k | 76.1 | 37.2 | -51.0% | 7.6 |
| spread-100k | 581.2 | 260.8 | -55.1% | 58.1 |

(older baseline 2e7986c: mobs -44.8 / mixed -60.1 / items -58.0 /
spread -36.1 — the vanilla legs re-measured: mixed 732->452 and spread
438->581 are RUN-VARIANCE-scale swings, the machine mix changed; the
deltas move with them. The grind reads the fresh table only.)

## Where the four scenarios stand (baseline 2e7986c, p50 — the L49-L52 era)

| scenario | vanilla | mod | delta | target (-90%) |
|---|---:|---:|---:|---:|
| mobs-25k | 494.6 | 273.0 | -44.8% | 49.5 |
| mixed-150k | 732.3 | 292.1 | -60.1% | 73.2 |
| items-100k | 63.5 | 26.7 | -58.0% | 6.3 |
| spread-100k | 438.3 | 279.9 | -36.1% | 43.8 |

(older baseline 40795889686: mobs -62.6 / mixed -40.6 / items -58.6 /
spread -53.2 — the parity exactness re-gains since then COST mobs
-17.8pp and spread -17.1pp and GAINED mixed +19.5pp: L21-rest
re-enablement, the exact gates, the probe tax — the grind below must
reclaim them without weakening the now-proven semantics.)

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

### The fresh items-100k profile (lean run 41112622574, bd5eef6)
mod p50 **29.1 ms** (vanilla 58.9 → **-50.6%**; target 5.9). fastQuery is
GONE from the top-15 (was 13.96% — L44c's row publish + the L46 scratch
changed the shape). New top: newTrackerTick$ 12.04, itemTickFlush
11.16+1.38, guardEntityTick 5.02, Unsafe.getObject 4.52, ttlFastPath
3.97, **isViewAfter 3.13 (the L48 hoist targets exactly this — items
carries a bigger view population than mixed)**, appendDispatch 3.11,
Entity.level() 2.37. The merge/query lever is spent; the tracker and
the flush are the next floors.

### The FRESH mixed-150k profile (lean run 41236766183, ce5a471, 27778
### samples, mod p50 227.9)
1. **AABB.intersects(DDD) 8.09%** — the L37d exact re-check STILL owns
   the #1 slot (the marginExtra tax: the index's margin superset
   rejected by the LIVE-box compare). Move 1a (the exact f64 index
   boxes) is the kill — gated on the freshness families (addendum 14).
2. **CeHooks.isViewAfter 4.96%** — REGROWN to #2 (the L48 viewN==0 gate
   does NOT hold on mixed: the view population there is real, the map
   is hit in the steady state). NEW LEVER: the view-after map itself —
   an int-array stamp per slot (id-based, cleared per query) instead of
   the boxed HashMap, or a per-query epoch bit that skips the map read
   entirely for ids whose entry is provably absent (a Bloom-ish
   generation counter).
3. ChunkEntitySlices$BasicEntityList.indexOf 2.04 (vanilla ticklist
   reads — the L13 raw walk's counterpart), Unsafe.getObject 2.01 (the
   mirror internals), ImmutableCollections$ListItr.next 1.99,
   Entity.getBoundingBox 1.92, newTrackerTick$ 1.88, SynchedEntityData
   1.25, HashMap.getNode 1.22, tickListForEach 1.15,
   getMaxStackSize 1.02 (the L41 pre-resolution holds).

## Ranked next moves (updated 2026-10-31 after the fresh mixed-150k profile)
0b. **DONE — L46 (move 2), f142684-pre c8747b6**: the merge-scan caller's
   shared scratch answer list (FQ_SCRATCH, depth-guarded, knob
   CE_FQ_SCRATCH=0). The general getEntities keeps allocating.
0c. **DONE — L48**: the isViewAfter boxing HashMap.get hoisted behind a
   viewN==0 gate (0.91% self-time on the fresh profile; the steady state
   never touches the map now). INVALIDATED 2026-10-31: mixed re-grows
   the frame to 4.96% — the gate holds only for view-free scenarios.
0d. **NEW — L57 (isViewAfter part 2)**: replace the per-id boxed map
   with a primitive stamp array (slot -> last-view-query epoch); the
   common "no view ever" path reads one int, the map only on a stamp
   hit.
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

   OPEN QUESTION (needs a diagnostic): the Rust scan already filters
   all 3 axes exactly (AVX2 prefilter + exact_keep in Rust — the
   survivors ARE tight against the INDEX box), so an 84% Java-side
   rejection means the INDEX box disagrees with the LIVE box for 84%
   of the surviving candidates. Bench geometry context: the profile
   config is `area=0, inner_radius=28` — ALL 105k items pile in a
   57x57 zone (~42 items/block²), so every merge scan's expanded box
   covers thousands of co-located candidates and even a small
   index-vs-live disagreement (probe lag within the flush cycle?
   falling items during fill?) multiplies into hundreds of millions.
   The native path serves only ~741 queries in the window (fast−row)
   — each carries ~363k candidates — the pile cells. A marginExtra
   sample diagnostic (index box + live box for the first N rejects)
   settles which side is stale.
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

## Session note (2026-10-29, L42): the push-mirror delta fix landed

The parity hunt's remaining 4 PH divergences were re-rooted: the
first-move class (mobs t=183, spread) is the L12 push mirror's stale
delta capture + the deferred candidate writes (doc 40 addendum 6) —
fixed in bff37c3 (the value-carrying probeDelta + the immediate pair
apply). The items/mixed merge class (t=159/t=152) keeps its MSQ
diagnosis track: the fresh 6abd10f log shows the MSQ/MSR telemetry
fires only in the 448-499 dump window while the divergence starts at
152/159 — a PH_DUMP run at 150-170 (40922735974) re-arms it at the
divergence tick.

Profile (mixed-150k ON, 64f49f7, stackdepth=5 — now fixed to 64):
AABB.intersects 8.3% (97% inside the vanilla/moonrise section scans,
NOT our query paths — the vanilla query volume is the lever), the
tracker floor ~2.8%, the attribute cluster ~2.2%, fastQuery 4.4%.
The next profile run attributes the vanilla query roots (the
74.85% bucket) with the full stacks.
