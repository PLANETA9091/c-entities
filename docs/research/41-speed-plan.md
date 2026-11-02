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
0d. **DONE — L57 (isViewAfter part 2, 684231c)**: the view lookup is a
   flat open-addressing int table (vtabGet/vtabPut, 2048 slots,
   Fibonacci spread) — zero boxing, zero hashing allocations. Effect
   reads from the post-L57 baseline profiles.
0e. **DONE — L58/L58b (721c5de-era + de9cb75)**: the vanilla scan order
   lives IN the index (upsert_ord stamps, per-cell versions, lazy
   ensure_cell_order; (0,0)=unknown preserves stored stamps; stable
   sorts). The per-query ordSortVanilla is dead work now.
0f. **NEXT — move 1a (the exact f64 index boxes)**: still gated on the
   freshness families; the MSIB dispatch (run 41295183606) pairs
   idxMin vs liveMin per reject IN the divergence window (270-330) —
   the stale side per reject id lands with that run.
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

## The move-1a decision tree (pre-staged 2026-11-01, pending MSIB run 41295493035)

The 84% Java-side rejection question (the index box vs the live box)
resolves by pairing MSIB idxMin against MSR liveMin per reject id in the
270-330 window. The outcomes and their pre-committed fixes:

- **A. idxMin == liveMin, reject is pure margin geometry** (the candidate
  sits inside the index's MARGIN superset but outside the exact box):
  the f64 index (move 1a proper) shrinks the survivor set to exact —
  the Java marginExtra loop dies, AABB.intersects (8.09% #1 mixed)
  collapses with it. Implementation: widen the cell box store to f64
  (100k × 48B = 4.8MB), the AVX2 f32 prefilter stays as the first pass
  against a truncated copy, the Rust exact confirm promotes both sides
  to f64, the MARGIN expansion drops to 0 — the query result becomes
  set-exact, the Java re-check becomes an assert (knob-gated, then
  deleted).
- **B. idxMin stale (PRE-move) vs liveMin (POST-move)**: the probe
  batch's box for the candidate predates its body's move this tick —
  the R-B interleave (vanilla moves+merges per item in tick order; the
  flush batches them). Fix surface: the L44 view already models PRE
  boxes for RESYNC entries — extend the VIEW contract to every
  move-gated entry the scanner can see BEFORE its body (the
  freshness-story template: probe, version, or phase contract), or
  serve those candidates from the row path with PRE boxes (the L49
  pending-fresh mechanism already does this for covered rows).
- **C. idxMin newer than liveMin (off-thread nativeBoxDirect wrote a
  POST box the vanilla scan has not reached yet)**: the async-spawn
  mirror ran ahead. Fix: gate nativeBoxDirect writes behind the same
  IN_ITEM_APPLY phase discipline (serve stale-correct boxes during the
  apply pass; commit the async truth at flush).
- **D. idxMin = NaN (the id was not indexed at sample time)**: a
  vanished candidate (merged/picked-up) the index still serves — the
  ghost family. Fix: the remove-probe must ride the same batch as the
  merge apply (probeDie-template: batch upsert + drain in the same
  tick), or the row publisher must tombstone removed ids within the
  flush cycle.

The TL stream (run 41295158850, trace 202, window 270-330) independently
witnesses the push-timing family's first-divergence tick order — its
tick-list order stream names WHICH entity's list position moved first
(the B/C class boundary).

## The MSIB verdict + L60 (2026-11-01, runs 41295493035 / 41295158850)

**The exact-f64 open question is CLOSED — the index boxes are FRESH.**
The MSIB x MSR pairing (376k pairs, items) shows idx x0/z0/y1 == live
EXACTLY; the apparent 0.25 y-offset was a printer artifact
(nativeIndexBox packs [x0,y0,z0,x1,y1,z1]; the sampler printed slots
0/2/4 = (x0,z0,y1) vs the live sampler's (x0,y0,z0)).

**The real root of the #1 buckets: the query expansion leaked into the
SCAN bounds.** `EntityIndex::query` fed box ± (max_half_span + MARGIN)
into scan_cells — every merge scan pulled the max_half_span
neighbourhood (reject 2067 sat 20.6 blocks from a 1.25-wide query —
the world's max_half_span ratchet: ONE giant box poisons every query
forever), and the Java live-box trim rejected ~80% (marginExtra=70M).
AABB.intersects 7.11% #1 (items) / fastQuery 13.29% #1 (mixed) were
mostly this tax.

**L60 (ba1b10e)**: the scan bounds are the TRUE box; the expansion
covers only the cell WALK. Parity-neutral by construction (the Java
trim already exactified the answer). The two exactness tests that
pinned the margin-leak semantics moved to the vanilla-strict contract.
The f64-box move 1a is NOT NEEDED for the margin tax; the freshness
gate is resolved as class A (margin geometry) + the printer lesson.

## The 2ed4aba refresh (2026-11-02): the L59-L62 era + the ranked frontier

Baseline 2ed4aba (run 41422148952, p50): items 34.8 (-45.5% vs the
fresh 63.9 vanilla leg), mobs 239.5 (-23.8% vs 314.4), mixed 248.7
(-54.2% vs 542.5), spread 332.0 (-38.5% vs 539.7). The vanilla legs
swung hard between runs again (mobs 490 -> 314, spread 632 -> 540) —
the deltas read against the PAIRED legs only.

The landed floors this era (parity-gated, all 12/12 green):
- L59a/L61: the tracker fuse extends — the per-pass chunk memo + the
  syncVelocity consumption (addendum 20/21; javap bc 1116-1246).
- L60: the ds/dwrite DELTA_SAME census rides the tree.
- L62: the empty-equipment floors (enchTickEffects$ +
  equipUpdates$) — the mobs profile's #1 leaf family decoded via the
  JFR parent analysis: ImmutableList$ListItr.next 6.32% (mobs) =
  87.7% inside tickEffects (47.5%) + collectEquipmentChanges (39.3%).

### The ranked next moves (2026-11-02, after L62)

0. **The AI engine RNG protocol (THE mobs lever)** — the L22a native
   AI engine measured mobs-25k 193.2 vs the shipped 239.5 (e774d00)
   but diverges level.random at t=143 (the staged-request protocol
   consumes the RNG in a different order; SR/LR probes, run
   40771017055). Parity is now green and the freshness discipline is
   proven — the engine's pre-resolved-predicate redesign (doc 30 item
   4) must make the Rust goal engine consume random values in the
   EXACT vanilla call order. ~-19% on mobs, the single biggest
   module-owned move left. The vanilla-goal long tail (GoalSelector
   4.05 + tickRunningGoals 2.11 + WrappedGoal 1.11 + flags/goal
   predicates ~3%) is what it replaces; the shipped aiFloor stays off
   until the engine is exact (the L15 gate mirror has the same
   freshness problem at smaller scale).
1. **The native travel/collide port (doc 28)** — LivingEntity.travel
   cluster (travel 2.07 + getBlockState 1.84 + fluid interaction ~2.5
   + collide internals ~3) ≈ 10% of the mobs leg; the quiet-travel
   gate catches only 18% (2.85M/15.6M on mobs-25k) — moving mobs pay
   full vanilla travel. tick_item's move/collide machinery is the
   template; the port must be byte-exact on friction variants
   (block-below slipperiness), water/lava branches and the collision
   epsilon.
2. **The tracker share on mobs** — the ttlDecl counters (this era)
   name the fuse decline site; if seen/se, the TrackedEntity-shape
   probe is the next floor; if syncVelocity, the L61 fuse already
   covers the bench mobs and the sendChanges family (tryEncode 2.35
   on spread, sendChanges internals) collapses with it.
3. **The ds census verdict (L60)** — dwrite ~ item count per tick
   means the resting fixed point misses widely and the L16 gate needs
   re-derivation (the setDeltaMovement caller 5.89 + callee 5.7 on
   items is the largest items bucket left after the tracker).
4. **The per-leg long tails** (each 1-3%, no single move):
   AttributeMap cluster (2.10+1.56+1.21 — attrCache covers part),
   NaturalSpawner.createState (1.63 mixed), setOldPos, applyEffectsFromBlocks.

### The L62 era verdicts (2026-11-02, runs 41449046210 / 41449061017 / 41453326369)

The 46e2f3a baseline (L62a armed, paired legs): items 34.5 vs 76.1
(-54.6%), mobs 309.9 vs 537.4 (-42.3%), mixed 275.8 vs 681.7 (-59.6%),
spread 340.9 vs 561.8 (-39.3%). The mobs leg moved -23.8% -> -42.3%
with the ench/equip floors actually armed (the L62a ReplaceBody bug
kept them SOE-ing the whole mobs population in the red run before it).

**Frontier item 2 (the tracker share on mobs) is RESOLVED — L62b.**
The ttlDecl verdict named site 5 (interp) = 8,075,257 on mobs-25k,
every other site 0. Root cause, a 26.3 shape change:
LivingEntity.createInterpolationHandler() routes SERVER-side to
SteppedInterpolationTracker (create() picks the handler client-side,
the tracker server-side), so ServerEntity.interpolationTracker is
NEVER InterpolationTracker.NO_OP for a mob and the fuse's gate 5
declined every living entity. L62b (2c92b3a) consumes the tracker
exactly: on EMPTY trackedSteps with syncPosition FALSE the per-tick
sendChanges effect is exactly {predictedDelta=ZERO; ticksSinceLastStep++}
(addStep packs the counter into PositionStep(vec, ticks) on a later
teleport, so the ++ is replicated, never skipped). Both parity runs on
2c92b3a are 12/12 bit-identical with ttlDecl ALL ZERO — the fuse now
covers the full bench population. The 2c92b3a baseline quantifies the
mobs share this freed.

**Frontier item 3 (the ds census) has its verdict: ds=0/34.5M on
items.** The DELTA_SAME gate NEVER hits — every setDeltaMovement call
changes the value; a same-value skip is worthless. The setDeltaMovement
bucket (caller 5.89 + callee 5.7 on items) must be removed by NOT
calling it: the item tick floor's move writeback should land the delta
straight into the mirror fields (the L44 view contract), with the
vanilla call kept only on the declined/vanilla-routed population.

## The L63 verdicts (2026-11-02, run 41466481613 on 08d3af8)

**L63a + L63b are 12/12 PASS bit-identical.** The two items-module
floors landed together:

- **L63b — the TTL shape cache** (newTrackerTick$ 13.97% items): the
  ttl fuse covered ~100% of the bench population (ttl=7374/85.38M/
  85.38M) — the residual cost IS the per-entity probe chain re-verifying
  verdicts that cannot change without a player. The cache stores the
  TrackedEntity identity + the structural verdicts per mirror slot;
  the per-pass `ChunkMap.level.players().isEmpty()` read (ONE per pass)
  proves the seenBy/trackedPlayers/nearby-players lists cannot gain
  entries, `hardColliding` is ctor-fixed, and the per-tick volatile
  inputs (syncVelocity, syncPosition, trackedSteps emptiness for SIT)
  stay per-entity reads. The telemetry: ttlc=9,423,970/20,000/0 —
  20k fills, then ~100% hits, zero player-blocked passes.
- **L63a — the flush delta direct write** (setDeltaMovement 3.93%
  items): javap 26.3 — the whole vanilla body is the isFinite gate +
  the posLock-guarded store (NO flag writes; markHurt owns syncVelocity).
  The woven chain for a dispatch-owned item added three dead frames
  (the deltaNoOp guard the ds=0 verdict says never fires, the probeDelta
  capture dead for items, the virtual dispatch). putObjectVolatile is
  stronger than the uncontended monitor-guarded store; the fresh Vec3
  preserves the per-write identity.

The L63b-era profile reading for the next items moves: the walk's
floor is now (te read + identity compare + per-tick bytes + tickCount
write) — the next structural cut is the tracker MIRROR (the tickCount
in the mirror array, the se writeback only on the player-arrival
epoch) and the itemTickFlush pass-2 loop remains the biggest single
Java block.

## L64a — the L29 direct-read layer was silently dead (2026-11-02)

The 2c92b3a baseline telemetry: `froz=15.59M/15.58M/0 fd=0 cd=0` —
the L25/L29 direct entityData walk caught ZERO across all three gates;
the chain layer (getTicksFrozen() = a virtual call + the boxed get)
served 100% of 15.6M calls/tick — the 2.01% setTicksFrozen$ self-time.

**Root structure**: EDV_OK was a SHARED boolean requiring BOTH
IDX_FROZEN (initFrozenHandles' timeline) and IDX_FLAGS (initFlagHandles'
timeline — FLAGS_ID may not be captured yet) plus the O_ED trio; one
missed precondition at either init moment killed the whole direct layer
with NO telemetry (the edv probe writes lastErrorText, but the line
prints it only when lastError > 0). L64a (764dcd0): per-layer indices,
per-call validity in entityDataValueIdx, `edvskip:<reason>` telemetry on
every capture skip, initFlagHandles runs the idempotent initDataOffsets
itself, and the indices keep the L28d record verdict (the accessor id()
METHOD — EntityDataAccessor is a record; Unsafe refuses its fields, so
O_ACC_ID can never resolve).

Reading rules: lastErr= empty or `edvskip:<reason>` names the layer
state; fd/cd must move off zero with frozenGate unchanged; a red parity
gate pins L64a (the frozenGate/convGate/flagGate knobs bisect in one
run).

## The L15 re-test (2026-11-02, run 41467393454, ai_floor=1 on 08d3af8)

The R-A root (the L15 GoalSelector floor desyncing the RNG at t=143-145)
predates the L25/L26 v2 floor — the bisect verdict (40773723794) was
taken when the floor had the STAGED-request protocol. The v2 floor
replaced staging with the shadow walk whose predicate call sequence
matches the vanilla set walk structurally (cleanup over the running
shadow in set order, update over the full set, the same short-circuit
order) — but it was never re-bisected. This run re-arms it on the
current tree (gs= must move off zero on the mobs leg; the gate must
stay 12/12 PASS). GREEN → re-ship (-Dcentities.aiFloor=false stops
being passed) and the GoalSelector cluster (~8% mobs) floors; RED →
the PH_DUMP bisect with ai_floor=1 isolates the tick.

## The L63 baseline verdict (2026-11-02, run 41466473668 on 08d3af8, paired legs)

| scenario | vanilla | mod | delta | prev delta |
|---|---:|---:|---:|---:|
| items-100k | 76.9 | 34.7 | **-54.9%** | -43.2% |
| mobs-25k | 520.0 | 291.3 | **-44.0%** | -43.2% |
| mixed-150k | 539.6 | 224.8 | **-58.3%** | -49.0% |
| spread-100k | 418.0 | 231.1 | **-44.7%** | -39.4% |

L63a+L63b delivered +11.7pp items, +9.3pp mixed, +5.3pp spread,
+0.8pp mobs (no item flush there — the tracker cache alone). The
items profile post-L63: newTrackerTick$ GONE from the top (13.97% ->
ttlCachedPath 3.68%), the #1 leaf is Unsafe.getObject 17.35% and the
JFR parent analysis pins 78.9% of it INSIDE ttlCachedPath's own hops —
L65 (51c0be2) caches the final-field refs (serverEntity /
interpolationTracker / trackedSteps — the te identity pins the chain)
to cut exactly those. itemTickFlush 9.97% next (L64c's scan skip
shaves the two full-batch loops), guardEntityTick 4.26% (the vanilla
tick-loop frame — the L7 dispatcher's deeper redesign), the merge
cadence.

### The refreshed frontier (2026-11-02, after L63-L65)

0. **The L15 re-test** — the ai_floor=1 parity with the FIXED input
   wiring (CE_AI_FLOOR export, a69fd58; the first attempt 41467393454
   never armed the floor — run-baseline.sh passed aiFloor=false on its
   own). GREEN → re-ship (run-baseline.sh stops passing aiFloor=false;
   the parity input default flips) → the GoalSelector cluster ~8% mobs
   floors. RED → the PH_DUMP bisect with ai_floor=1.
1. **The tracker mirror (L66, the items endgame)** — the cached walk's
   floor is (te read + identity + per-tick bytes + tickCount RMW +
   updateDataBeforeSync) per entity per tick ≈ 4-5% items. Moving the
   tickCount mirror-side (the se writeback rides the players-arrival
   epoch; the per-pass gate hands the state back) collapses the walk
   into a Rust array pass. The freshness story: se.tickCount is read
   ONLY by sendChanges (players present → the gate is off → the mirror
   hands the state back on the transition).
2. **The travel/collide port (doc 28)** — unchanged, ~10% mobs.
3. **The L7 dispatcher batch-tick** — guardEntityTick + the per-entity
   lambda + the itemSkip chain ≈ 7-8% items; one weave at the
   EntityTickList level replaces the per-entity frames for owned items.
4. The per-leg long tails — unchanged.

## The L69 verdict (2026-11-02, run 41507709894 on d02b03c): the L15 re-test is GREEN — the floor re-ships

**Frontier item 0 is CLOSED — but not the way the bisects implied.** The
"parity window" L67..L68 was FALSE: run 41489839532 (79632c2) passed
12/12 bit-identical BETWEEN the 15:xx reds and the L67 reds, with the
same code. The runs differed by DISPATCH INPUTS, not code: the reds were
curl-dispatched with an EMPTY ai_floor input (workflow_dispatch yaml
defaults do NOT apply to curl dispatches — `${{ inputs.x }}` is the empty
string, and parity.yml's `[ '' = '0' ] || echo 1` computed CE_AI_FLOOR=1),
so every "L67+ regression" run had the L15 AI floor ARMED while every
green run to date had it dead or '0'. The floor was never gate-proven.

The floor bug (javap 26.3 GoalSelector.tick + tickRunningGoals(true)):

- vanilla's cleanup walk and its tickRunningGoals tail iterate
  availableGoals — ObjectLinkedOpenHashSet, INSERTION order; two live
  goals of one mob tick/stop in the set's order;
- the floor's flat path kept the running shadow in START order: the
  update pass appended fresh starters (sh.goals[n++]); once goal X
  (set pos 0) started after goal Y (set pos 1) was already running the
  shadow was [Y, X] while vanilla walked [X, Y] — navigation/
  lookControl/pathfinding writes interleave in the wrong sequence and
  the mob diverges. 2500 mobs x 382 ticks = the T382 yaw=718 chaos and
  the type-mismatch storms in every mobs-bearing scene (items never
  diverged — no GoalSelector);
- fullPassTick additionally never appended its own update's starters to
  the shadow — a stale shadow that re-triggered fullPass forever.

L69 (d02b03c): the update merge-walk rebuilds the running array in SET
order (AI_RUN_TABLE scratch; survivors and fresh starters placed as the
walk reaches them = the walk IS the set order; same-pass stops compacted
afterwards), and fullPassTick rebuilds the shadow after its tail. Run
41507709894: 12/12 PASS bit-identical, gs=999514/472780/7429/6709 (the
floor ran at full scale). The re-ship (3c7e345): run-baseline.sh's
CE_AI_FLOOR defaults to 1, parity.yml's ai_floor default flips to '1'.

**Lesson for the ledger: a bisect run's KNOBS are part of the evidence —
an empty-string dispatch input is not a yaml default. The gate config
must be echoed into the job log (it is: the `CE_AI_FLOOR=$([ ... ]...)`
line names the arm state of every run).**

## L70 (be17c27): the tickCount mirror — frontier item 1 landed

The items-100k #1 leaf (Unsafe.putInt 9.79%) was the L63b cached path's
per-tick ServerEntity.tickCount RMW (~100k/tick; the counter's ONLY
reader is sendChanges' interval gate, unobservable while nobody tracks
the entity). The mirror: TT_TC[m] on the ARENA slot (identity-pinned by
TT_TE[m] == te). While the ChunkMap is ARMED the cached path increments
the mirror; the fill stamps it from the real field right after
ttlFastPath's real RMW; every decline tick re-captures post-sendChanges;
the armed->disarmed transition hands the mirror back into the real field
and disarmed->armed re-captures (one list walk per transition, a tiny
per-ChunkMap armed memo). Any reconcile failure sets TT_TC=null and the
real RMW fallback resumes (exact from the next tick). The bench has no
players — no transitions, no readers; prod keeps the interval counter
exact across join/leave.

### The frontier after L69/L70 (2026-11-03)

0. **The fresh baseline read** — the running baseline on be17c27 is the
   first one with the floor shipped + L70; its REPORT re-ranks this list
   (expect: GoalSelector.tick 1.92 + tickRunningGoals 1.70 collapse into
   gsFloorTicks; putInt 9.79 collapses on items; the ttlCachedPath walk
   floor stays).
1. **The travel/collide port (doc 28)** — ~10% mobs; the biggest
   module-owned move left.
2. **The L7 dispatcher batch-tick** — guardEntityTick 6.85% + the
   per-entity lambda ≈ 7-8% items.
3. **The itemTickFlush 5473 guard** — `e.isRemoved()` 9.28% self on the
   flush's per-entry removal check; a mirror-bit freshness story is
   needed (removals between dispatch and flush ride probeRemove —
   verify batchRemove's IB handling first).
4. The per-leg long tails — unchanged.
