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

## The L70 verdict + L71/L72 (2026-11-03)

**L70 is parity-proven**: run 41509676180 (be17c27) — 12/12 PASS
bit-identical with the mirror armed. The fresh baseline's items-100k-mod
artifacts read: Unsafe.putInt GONE from the top-15 (was 9.79% #1);
items-100k paired legs 69.18 vanilla vs 28.98 mod = **-58.1%** (was
-55.4%). The cost MOVED, it did not vanish: the JFR parent analysis on
the run's own rec.jfr (artifact, JDK jfr print + a 2-line attribution —
no CI round-trip needed) pins 71% of the new #1 (Unsafe.getObject
11.45%) to the per-tick te read in newTrackerTick$.

- **L71 (5b56ed8) — the travel floor's input gate widens to
  vertical-only.** mobTravelNative declined ANY nonzero travel input,
  which made SHAPE C unreachable for every actively-steering mob (the
  dominant walking population) and pinned the quiet-travel catch at 18%.
  The steering math (getInputVector: the 1e-7 gate, the >1 normalize,
  the f64 speed scale, the f32 Mth sin/cos yaw rotation) was ALREADY
  coded in the SHAPE C delta step and pinned by mobtravel.rs; the hull
  proof is swept-volume-generic. input.y stays declined (a positive dmY
  extends the swept hull's TOP — the proof's box covers the falling
  dmY only). SHAPE B now REQUIRES zero horizontal input: a mob steering
  into a wall leaves the steering residue in the vanilla delta after
  the collision, which the (0, -g*vf, 0) writeback would erase.
- **L72 (f5156eb) — the tracker cache's identity key is TT_E[m] == ent.**
  One array read replaces the per-tick te Unsafe hop on the ~100%-hit
  cached path (probeRemove clears TT_E in the same unconditional block
  as the other TT_ arrays; the te is 1:1 with (entity, world)). The te
  read and the chunk read both move behind the cached-path check.

### The frontier (2026-11-03, after L70/L71/L72)

0. **The gate runs on f5156eb** (parity + baseline in flight) — the
   verdicts quantify the steering widening (expect travelWalk to jump:
   the walking population finally routes) and the identity swap.
1. **The itemTickFlush pass-2 apply** (10.78% items at 5473 + the
   field-write cluster) — the structural cut is the flush writeback
   into Rust (the engine already computes everything; the Java loop is
   a handful of field writes per entry). The L8.2 compact-batch shape
   needs the writeback contract re-read first.
2. **The guardEntityTick + appendDispatch chain** (~11% items) — the
   L7 dispatcher's next step.
3. **The AI engine RNG protocol** (~-19% mobs, doc 30 item 4) — the
   big one, unchanged.
4. The per-leg long tails — unchanged.

## L73 (c663d5c): the per-mob floor maps drop their weak references

The fresh mobs-25k-mod profile (be17c27 artifacts, JFR attributed
locally) bills Reference.get 3.90% (#1) across the two weak-keyed
identity tables the per-mob floors probe per mob per tick:

- shadowOf/aiShadowPeek (48%+22% of the bucket) — the AI floor's shadow
  map (AIKey extends WeakReference). L73: STRONG keys + explicit removal
  — probeRemove drops the removed mob's selector through its
  goalSelector field (O_MOB_GOALSEL) via aiDrop's backward-shift
  deletion (Knuth 6.4R: the following cluster entries whose home does
  not lie cyclically between the hole and their own slot shift back, so
  the probe chains stay intact). probeRemove is the proven universal
  removal choke point; a missed drop leaks one bounded entry.
- attrCache (30%) — the travel floor's per-mob AttributeInstance[] cache
  is now DIRECT-MAPPED by entity id (AT_E[m] == e, 32k slots,
  probeRemove clears). One array read replaces the weak probe + the
  hash walk; id-slot collisions evict (the identity check rejects, the
  next travel re-fills).

The mobs legs' next floors (the fresh mobs-mod top, post-L70/floor):
WrappedGoal.getFlags 2.59% (the AI floor's own per-goal flag reads —
the flags CAN change, so a stamp needs the collide-check story),
AttributeMap.getValue 2.97% (the instance cache is landed; the value
walk needs a freshness story), setTicksFrozen$ 2.27% (the L68 memo —
its residual is the getter+setter counter arithmetic), SynchedEntityData
.getValue 2.16% (the L15/L29 direct-read layer's residual).

## The L71 verdict (run 41511972780 on f5156eb): the steering widening diverges — knob-gated (L71b)

The widening FAILED the gate: mobs/mixed/spread all red while items went
12/12 bit-identical (L72's TT_E identity swap is CLEAN). The failure
shape is NOT structural: same counts, no type mismatches, the worst
deltas are SMALL drifts (mixed T389: yaw 9.4deg, x 0.070, z -0.115,
vx -0.0136) — a tiny per-tick difference in the steering math compounding
through the AI's look/navigation feedback loop. The coasting shapes
(input==0, the pre-L71 proven 12/12) are untouched.

L71b (385a32d) puts the widening behind centities.travelSteer (default
OFF — the shipped gate keeps the proven input==0 contract) and re-arms
the hunt: the shipped-config parity run must go back to 12/12; the
traced bisect run (travelSteer=true + trace_id + PH_DUMP 240-400)
captures the TTR enter/exit lines for the traced mob — the exact shape,
inputs and computed delta at the first divergent tick, which names the
broken float op (candidates: the fis normalize/scale f32-vs-f64 seams,
the Mth table vs the JIT's intrinsic, the input-rotation operand order,
or the delta.y gate interacting with a steering tick's gravity write).

The rule stands: a widening ships ONLY after the traced divergence is
pinned to a single float op and the mobtravel.rs mirror pins it.

## L74 (f22fb53): the move() setPos condition — the near-stationary band

The traced travelSteer bisect (run 41514292000 on 385a32d, trace_id=78,
PH_DUMP 240-400) failed the gate exactly as the L71 verdict predicted:
mobs T382 / mixed T389 / spread T389 — same counts, no type mismatches,
worst deltas TINY (mobs T382: x 1.67e-4, z 4.90e-4 across entities 652 /
975 / 2336 — the drift magnitudes sit inside a ~3.2e-4 envelope). The
TTR lines (id=78, spawned t≈672 — NOT a T382 witness) showed the shape-C
routing itself works: input=(0,0,0) coasters enter with dm_horiz
0.025-0.07 and exit dv=dm×0.546 as designed.

The pin came from the DECOMPILED Entity.move (26.3, line 1173+):

```java
Vec3 movement = this.collide(delta);
double movementLength = movement.lengthSqr();
if (movementLength > 1.0E-7 || delta.lengthSqr() - movementLength < 1.0E-7) {
   addMovementThisTick(...); this.setPos(pos.add(movement)); recordMovement(...); }
```

The y-clip makes movement == (dmX, 0.0, dmZ); with the gated
dmY = -g*vf (~ -0.078) the second term is (dmX²+dmY²)+dmZ² minus
(dmX²+0.0)+dmZ² ≈ 6.1e-3 ≥ 1e-7 regardless of the sum roundings — the
condition reduces to movementLength > 1e-7. A NEAR-STATIONARY mob
(dm_horiz² ≤ 1e-7, |dm_horiz| ≤ 3.16e-4) does NOT setPos in vanilla.
The SHAPE C writeback moved it every tick: a per-tick drift of up to
~3.2e-4 exactly in the decelerating-coaster / wall-residue population —
the drift envelope matches the T382 dump to the digit. The rest of the
move() body (the collision flags, setOnGroundWithMovement, the delta
tail with the getBlockSpeedFactor multiply) runs OUTSIDE the condition —
the writeback now reproduces the condition bit-exactly (both branches),
declines the fall-damage clip (fallDistance != 0) and declines
getBlockSpeedFactor() != 1.0F (honey/soul sand feet blocks). mobtravel.rs
pins the condition (move_sets_position + the band-edge test — note the
threshold compares the COMPUTED lengthSqr: sqrt(1e-7)² re-rounds ABOVE
1e-7, so the exact-threshold pin uses representable squares).

The same-body read also VERIFIED op-exactness of everything else the
widening touches: travelInAir's blockFriction is the MODIFIER-adjusted
computeModifiedFriction (26.3 — matches frictC), the final scale is
blockFriction × computeModifiedFriction(0.91, dragMod) (== hC), the
vertical is movementY × computeModifiedFriction(0.98, dragMod) (== vf;
omnidirectionalAirMover() = false in the Entity base), the restitution
is provably 0.0 for the gated dmY (|dmY| ≤ g always → the max() branch
never taken), getInputVector normalizes by DIVISION (Vec3.normalize
bytecode: x/d, y/d, z/d — same as the engine; the 1e-5 guard is
unreachable at lsC > 1.0), Mth.sin/cos are the SAME table through a
DOUBLE parameter in 26.3 (sin(double) — the module's call is identical),
getFrictionInfluencedSpeed HAS the 0.6 special-case (matches fisC), and
Vec3.scale widens the float speed — all bit-equal.

## L75 (6fcded7): the direct id read — getId()'s throw edge

javap 26.3: Entity.getId() reads the id field TWICE around a defensive
"Tried to access entity ID before ID assignment" IllegalStateException,
and the method is not final. The probes/dispatch/mode tables read the id
~9x per item per tick → 4.52% self on the items-100k leg (the #4 leaf,
ABOVE the whole Unsafe cluster). entId() = one Unsafe field load
(O_ENT_ID resolved idempotently in initDataOffsets, the virtual-call
fallback when unresolved); 88 call sites replaced, the 9 println/append
diagnostics keep the virtual call. The 0-id entity previously threw
(disarming through the probe guards); it now indexes slot 0 and declines
by mismatch — dead on live populations.

### The frontier (2026-11-03, after L74/L75)

0. The L74 gate runs (in flight): the shipped-config parity + baseline on
   f22fb53/6fcded7, the traced travelSteer re-bisect (trace_id=652 — a
   T382 drifted entity, not the t≈672-spawned 78), the mob_ai=1 SR/LR
   hunt for the AI engine's RNG protocol (the −19% mobs lever).
1. **The itemTickFlush compact pass-2** (15.4% items across the flush
   lines; the loop head `null + isRemoved` is 11.03% alone) — the batch
   bits already classify every entry; the structural cut is the
   two-loop split: a TIGHT counter loop (MODE stamps + tickCount/
   totalEntityAge/age writes, ~6 ops/entry) over all n, and the FULL
   tail loop over Rust's compact index list (entries with bits beyond
   the counters). Contract: outI grows to n*4 + 1 + n (outI[n*4] = cn,
   then the compact indices); include an entry when the delta write /
   MOVED / MERGE / DESPAWN / VOID / LAND / needsSync bits fire OR the
   out-fd/out-pd differ from the batch pre-state (the Java compares
   stay in Loop B — over-inclusion costs one branch). natives_parity
   pins the new descriptor.
2. The guardEntityTick + appendDispatch chain (~11% items) — the L7
   dispatcher's batch-tick step.
3. The AI engine RNG protocol (~-19% mobs) — the mob_ai=1 hunt is armed;
   scripts/srlr_first_div.py pairs the OFF/ON legs' SR/LR/PH streams and
   names the first divergence tick + stream.
4. The per-leg long tails — unchanged.

## L77 (this commit): the steering widening ships ON

The L74 move()-setPos fix RESOLVED the f5156eb divergence: the traced
re-bisect (run 41521042020 on f22fb53, travelSteer=true, trace_id=652 —
a T382 drifted entity) passed the gate 12/12 (the earlier 41514292000
run traced id 78 — spawned t≈672, NOT a T382 witness; the TTR data was
routing-correct but predates the divergence). The shipped-config parity
(41521019643) and the L75 parity (41522268657) are GREEN too — the
no-move band branch and the direct id read are parity-clean.

TRAVEL_STEER_ON defaults ON now (run-baseline can still ship it off per
run via -Dcentities.travelSteer=false). The quiet-travel catch should
jump: the actively-steering population (the dominant walking share,
pathing zombies with xxa/zza != 0) rides SHAPE C instead of paying full
vanilla travel — the L71-era projection was ~10% of the mobs leg on top
of the proven 18% quiet catch.

## The AI-engine hunt (mob_ai=1, run 41521523478): the first REAL engine data

- parity-items: SR/LR/PH identical through t=272 (the engine touches no
  item).
- parity-mobs: **LR (Level.random) diverges FIRST at t=143**, SR at
  t=145, PH at t=147.
- The PH hash covers ONLY x/y/z + velocity (8 invokes — the harness
  comment; NO yaw/hp/tc/type) — the state was "still identical" while
  the streams split.
- NO AI-path class reads Level.random (javap Fieldref scan: zero across
  21 goal/entity/navigation classes; the only level().getRandom() in
  Mob is finalizeSpawn — spawn-time).
- The doc-40 (ai_floor=0, mobAi=1) bisect was INERT: the mobAiFloor
  guard requires VH_GS_DISABLED (the L15 init), so the engine never ran;
  41521523478 is the first run with the engine actually engaged.

L76 arms the AITR per-round protocol trace (the (round, req-op, goalIdx,
cursor, seed) sequence for the traced mob); the follow-up dispatch is
mob_ai=1 + trace_id=<a t=143-alive mob> + PH_DUMP 138-160.

## The hunt deepens (runs 41525558896 + the step-count unwind): t=143 = ONE extra LR draw

The L76 AITR run (mob_ai=1 + trace_id=1 + PH_DUMP 138-160) landed while
in flight — the partial log gave everything:

- **The PD diff named the spawn batch**: at t=147 the OFF leg spawned ids
  83-92 (10 entities), the ON leg 83-91 + one at t=148, at DIFFERENT
  positions — the spawn ROLLS had already shifted.
- **The LCG step unwind** (the ThreadUnsafeRandom recurrence inverted
  between the per-tick end-of-tick probe values): t=143 is the FIRST
  divergence and it is EXACTLY ONE extra Level.random step on the ON
  leg (LR 132→133; SR 426/426 — the SHARED stream still identical!).
  The LR shift stays +1..+3 through t=146 and explodes at t=147
  (799/760 — the extra spawn).
- The AITR per-round protocol trace shows the engine busy with op=6
  (REQ_FIND_TARGET) staged requests around the window — NONE of which
  consume RNG (findTarget over 0 players).
- javap scans across 21+ classes (Zombie, LivingEntity, Monster,
  PathfinderMob, Entity base, every classified goal, the navigation, the
  controls, PathFinder): ZERO Level.random references; the only
  level().getRandom() in Mob is finalizeSpawn (spawn-time). The module's
  own code never touches it either.

The +1 draw at t=143 comes from a state-conditional Level.random call
whose STATE is outside the PH hash (yaw/goal-state/blocks). The L77
instrument (this commit): the harness (both legs) replaces
ServerLevel.random with a delegating CountingRandom — the stream is
untouched, every draw is counted and its caller frames captured; the
tick end prints LRC (the count) always and the LRF frame dump inside
LRC_WINDOW (default 138-150). The OFF/ON LRF diff names the extra
caller class.method directly.

## L79 (this commit): the compact pass-2 — the flush applier walks two lists

The frontier's item 1 shipped. The old single apply loop paid its head
(the `IB_ITEM[i]` null check + the `isRemoved()` pair + the MODE stamp
writes) for EVERY entry — 9.30% + 1.30% of the items-100k leg at the
top two flush lines, plus the 1.70% `Entity.isRemoved()` leaf — while
the post-counter tail population is a small fraction of n on live
batches (the L78 TAIL_NONE gate already proved MOST tails are no-ops).

The core now appends a COMPACT INDEX LIST after outI[n*4]: outI[n*4] =
cn, then the batch indices whose Java tail is NOT the proven TAIL_NONE
no-op — the exact !tail_none predicate the 12/12 L78 gate proved,
restricted to the IF_NATIVE non-abort entries (non-NATIVE entries never
reach the Java tail: the applier's IF_NATIVE gate continues first). The
applier splits into two loops: LOOP A walks all n for the MODE stamps +
the counters (tickCount/totalEntityAge/age — the ~6 ops/entry the old
loop paid anyway), LOOP B walks ONLY the compact list for the full
tails (delta/pd/fd/onGround/setPos/needsSync/merge/despawn/void/land),
batch order kept. The JNI descriptor is UNCHANGED — only the outI
length contract grows to n*4 + 1 + n.

Loop-order note (the one semantic delta): the counters now run BEFORE
any tail of the same pass; the old loop interleaved them. The only
entries whose counters a tail could have skipped are ones the same pass
REMOVED (merge absorb / despawn / void) — a removed entity's
tickCount/totalEntityAge/age are unobservable (discard fires no reads
of them; the store mirror already holds the core's committed values).
The itemTailNone diagnostic moved to loop A (the core excludes
TAIL_NONE entries from the list, so the loop-B belt continue is dead by
construction).

## The LRC instrument was a silent no-op — the interface scan bug

The first LRC/LRF hunt (run 41581856021, mob_ai=1 + LRC_WINDOW=138-160)
streamed 6738 LRC lines — EVERY one calls=0, all 160 LRF dumps n=0,
while the SR/LR stream probes still tracked the LCG state perfectly.
The parity gate failure was the EXPECTED mob_ai=1 divergence (the hunt
is not a gate run); the zero counts were the instrument bug.

Root cause: Paper 26.3's Level.random is a
`ca.spottedleaf.moonrise.common.util.ThreadUnsafeRandom`, whose DIRECT
interface is `net.minecraft.world.level.levelgen.BitRandomSource` — a
RandomSource SUB-interface. The harness resolved the proxy interface by
scanning `random.getClass().getInterfaces()` (directly-declared
interfaces only), never matched `net.minecraft.util.RandomSource`,
returned the original object, and the `LR_WRAP != LR_OBJ` guard then
skipped the field write: the "proxy" was the original, nothing counted,
and no exception ever fired (the catch's disable-if only covers SR
failures). javap: `ThreadUnsafeRandom implements BitRandomSource`,
`Level.random` is `protected final`.

The fix: resolve the interface BY NAME through the delegate's loader
(`Class.forName("net.minecraft.util.RandomSource")`) and accept it via
`isInstance()` (hierarchy at any depth); a failing `rf.set` now prints
`PH t=... LR-wrap-set-failed` instead of degrading silently. The
re-diff of the failed run's SR/LR probes (L77+L78 shipped) re-confirmed
the divergence tick: mobs LR first differs at t=143 (141/142
identical), SR cascades at t=145 — the LRC_WINDOW=138-160 dispatch
parameters stand.

## L80: the dispatch append drops the dead quad zeros

appendDispatch wrote all four IB_I slots per appended item; the native
path's ONLY iv read is iv[k*4+2] — the fresh-pickupDelay re-read (the
plugin-parity guard). The tick/age/rate slots were always dead for
IF_NATIVE entries: phase 1 overwrites all of them from the store
(st.tick_count/age/pickup_delay/despawn_rate, the L7 hydration), and
every Java-side reader of those slots is either the resync writer (the
check path fills its own quads) or behind the `(f & IF_NATIVE) != 0`
short-circuit (the rateOk arms, both tails). Three array stores per
item per tick come off the #2 items leaf. The batch quads of appended
items now carry stale data in those slots — harmless by the same reads.

## L81 (f5d9762): the RemoveBinomial verdict — the engine hunt's t=143 +1 draw is the ARMOR durability path

The L81 re-diff of the 07:16Z run pair (41595180152 push GREEN / 41595189807
hunt RED, both on 2eb760a) settled the AI-engine hunt's open question and
CORRECTED its scope:

- The RED run's bisect line reads `CE_MOB_AI=1` — it is the engine hunt, NOT
  the shipped config. The push-triggered parity on the SAME commit is GREEN:
  the shipped-config parity is intact, the divergence belongs to the
  mob_ai=1 path.
- The LRF frame diff at the first divergence tick (mobs t=143, window
  138-148, LRF_DEPTH=16): the ON leg consumed EXACTLY ONE extra
  Level.random draw — `k=92` (the tick's LAST draw, k=0..91 byte-identical):
  `RemoveBinomial.process <- Enchantment.lambda$modifyItemFilteredCount$0 <-
  Enchantment$FloatAction.lambda$asGeneric$0 <- Enchantment.applyEffects <-
  Enchantment.modifyDurabilityChange <- EnchantmentHelper.processDurabilityChange
  <- ItemStack.processDurabilityChange <- ItemStack.hurtAndBreak` — the
  armor durability path (Unbreaking-family effect) fired by a hurt event.
  The frame is depth-cut at 13 — the caller of hurtAndBreak (which damage
  source, which entity) needs LRF_DEPTH=30.
- The module has EXACTLY ONE hurtServer call site: the kernel's cramming
  audit (CeHooks pushDispatch: `roll == 0 && nonPass > cramming - 1 ->
  hurtServer(cramming, 6f)`). A hurt on an armored LivingEntity consumes a
  Level.random draw in `ItemStack.hurtAndBreak` -> processDurabilityChange
  — the L38f comment's "an extra hurt with ZERO RNG draw" premise is FALSE
  whenever the crammed mob wears an enchanted armor piece (the LRF
  populateDefaultEquipmentSlots frames prove the scenario mobs DO spawn
  equipped).
- The SAME signature reproduces on the other legs of the SAME hunt run, in
  the OPPOSITE direction: mixed first-divergence t=146 and spread t=145 are
  OFF-extra RemoveBinomial draws (vanilla hurt, module did not). The
  kernel's mirror-based cramming audit (hits_count/non_pass over the
  mirror) diverges from vanilla's LIVE `getEntities(this, box,
  pushableBy(this))` list in BOTH directions at the cramming boundary —
  never a stream-level shift: items (no LivingEntities, no armor) are
  bit-identical through the whole window.
- mobs LRC counts at the window: t=142 96/96, t=143 92 (off) / 93 (on),
  t=144 95/97, t=145 92/95, t=146 94/96 — the +1 persists and grows as the
  extra hurt repeats on later ticks before the spawn rolls explode at t=147
  (the doc-40 PD observation).

The L81 instrument (this commit) names the culprit pair per run:

- `LHURT t= ct= id= src= amt= hp= type=` — the LivingEntity.hurtServer
  PREFIX probe (woven into the die-probe chain, javap-verified 26.3
  signature `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world
  /damagesource/DamageSource;F)Z`), window-gated, prints EVERY living-entity
  hurt on the ON leg (the vanilla-internal fire/fall/cramming hurts reach
  the same virtual call) — the ON leg's full hurt ledger.
- `CRAM t= ct= id= np= roll= ncol= hurt=` — every kernel cramming-condition
  firing (bit1 = hits_count > cramming-1) with the audit's numbers — the
  module's view of the list vanilla counts live.
- The OFF leg's LRF draw stream (LRF_DEPTH=30 renders the full armor-hurt
  chain + the aiStep/baseTick frame that names the damage path) is
  vanilla's truth.
- The whitelist carries both lines (run-baseline.sh).

Dispatched: 41651523897 (shipped config + instruments — the reference
ledger and the weave-parity check) and 41651746764 (mob_ai=1 + the same
instruments — the red-leg reproduction with entity naming). The verdict
reads: at the divergence tick the hunt run's CRAM line (module fired) with
NO matching OFF-leg LRF armor draw (vanilla did not hurt) — or the inverse
— IS the divergence; the entity id + the kernel's np/ncol numbers then aim
the mirror-freshness fix (the KPI/KPIc dump aims the candidate set).

## L81b pre-dispatch: the 41653564594 post-mortem — the streamed job log alone triangulates the divergence to the fill's t=140-147 seed zone

The L81-fixed re-run (41653564594, d159bfa, mob_ai=1, PH_DUMP=138-152; the
push-paired 41653558651 shipped GREEN again): gate RED on mobs/mixed/spread,
items bit-identical. The 1.8GB parity-results artifact was NOT needed — the
delta-tail whitelist already streams PH/PD/SR/LR/LRC/LRF for every leg, and
the PH adaptive emit covers t=1..400 at full resolution. The per-leg diff of
the JOB LOG (8 legs = 4 scenarios x 2):

- PH (pos+vel+fire+hp+item-amount avalanche): first h-diff mobs t=147,
  mixed t=147, spread t=146; items NEVER. First n-diff: mobs t=147
  (off 92 / on 91, adds 13/13 BOTH), mixed t=153, spread t=161.
- LRC (Level.random calls/tick): mobs t=143 92/93 (ON +1), mixed t=146
  65/64 (OFF +1), spread t=145 58/57 (OFF +1) — re-confirms the L81
  verdict ticks on the new commit.
- SR (SHARED_RANDOM end-of-tick state): first diff mobs t=145, mixed
  t=146, spread t=145 — AFTER the mobs LR count diff, BEFORE the PH diff.
- PD (per-entity avalanche, ids sorted): mobs t=147 ids 83..91 m-diff,
  id 92 OFF-only; mixed t=147 ids 451..457 m-diff (no only-*); spread
  t=146 ids 251..263 m-diff (no only-*). Ids 1..82 byte-identical.
- The fill mapping (mobs: scale 2500, 13/tick, fill starts plugin-t=140):
  the t=146 batch = ids 79..91, so the OFF-only id 92 is an EXTRA entity
  created during that batch, and the m-diff at 83..91 is the +1 id shift
  (OFF's id 84..92 = ON's 83..91). The T382 snapshot flips (off=CHICKEN
  at ids 84/165/354/415/871/873/1227/1308) are the JOCKEY CHICKENS: the
  mobs fill spawns 100% zombies (plugin rnd = Random(42L), item/stand
  percentages 0), so every chicken is a Mob.finalizeSpawn jockey mount.

The LRF_DEPTH=30 frames name BOTH extra-draw paths exactly:

- mobs t=143 k=92 (ON-extra): RemoveBinomial.process <- ...
  <- ItemStack.hurtAndBreak x3 <- **Mob.burnUndead <- Mob.aiStep** <-
  Monster.aiStep <- LivingEntity.tick <- Zombie.tick <- tickNonPassenger
  <- guardEntityTick <- CeHooks.tickListForEach <- EntityTickList.forEach
  <- ServerLevel.tick. The SUNLIGHT ARMOR-WEAR path — the L81b
  hypothesis confirmed verbatim.
- mixed t=146 k=64 + spread t=145 k=57 (OFF-extra): RemoveBinomial <-
  ... <- ItemStack.hurtAndBreak <- **Mob.finalizeSpawn <-
  Zombie.finalizeSpawn <- CraftRegionAccessor.addEntity <-
  BaselinePlugin.fillStep** — the SPAWN-TIME equipment roll.

Verdict refinement (kills the L81 kernel-cramming hypothesis): CRAM lines
are ZERO across the whole window (the kernel audit never fired) and the PH
hash — which mixes fire AND hp since doc-40 — is byte-identical at the
mobs t=143 wear divergence: a cramming hurtServer would flip hp the same
tick. The t=143 +1 draw is durability-only (invisible to PH AND to the
snapshot — neither hashes durability). With SR end-of-tick state identical
at t=143..144 and the first 92 LR draws byte-identical (same callers, same
order), the extra burnUndead wear is a WITHIN-TICK serving flip: the
engine's staged-request RNG protocol (doc-30 item 4) hands the burn
gate (nextInt(2)) a different value while keeping the per-tick SR draw
COUNT identical — a reorder/reassignment, not a count error. The LR count
flip is the first VISIBLE consumer consequence; the jockey rolls (t=146
finalizeSpawn, Level.random) then flip and the id streams explode.

Dispatch plan (L81b, commit c1a0f75): the BURN ledger (per burnUndead
call: id/type/fire/hp/isSunBurnTick, BOTH legs — the module weaves the
probe in the shipped declined arm too) names the exact flipping call;
CE-INIT proves the window parse once per boot; BURN-LIVE separates
"not woven" from "window empty". Run A = mob_ai=1 + TRACE_ID=83 (the
first m-diff id, exists in both legs) + LRC_WINDOW=138-160. Run B =
the push-triggered shipped parity on the same commit (already in flight)
— the weave-neutrality check for the declined-arm burnUndead insert.

## L82 (7f1f72f): THE PARITY FIX — the engine ran a phase 26.3 never runs

The L81b hunt (41670844291, dd59a1d + BURN/AITR/trace-83, LRC_WINDOW
138-160) collapsed the whole engine-hunt frontier to ONE line of bytecode:

**javap Mob.serverAiStep (26.3)** — the branch structure:
- bc 75-96: `n = tickCount + getId()`; `if (n % 2 != 0 && tickCount > 1)`
  → **ODD branch (bc 91-145): `targetSelector.tickRunningGoals(false)` →
  `goalSelector.tickRunningGoals(false)`** — ONLY the running goals with
  `requiresUpdateEveryTick()` tick (javap: the classified zombie set's
  single override is RandomLookAroundGoal → iconst_1; the engine's
  every_tick() == GK_LOOKAROUND is exact). NO cleanup, NO update.
- **EVEN branch (bc 148-191): `targetSelector.tick()` →
  `goalSelector.tick()`** — and GoalSelector.tick() (javap) is ONLY the
  goalCleanup loop (cc/stop per availableGoals order) + the lockedFlags
  removeIf + the goalUpdate loop (canUse/start). **NO running-goal tick
  phase exists on even ticks.**

The engine's even path hopped CLEANUP → LOCK_REBUILD → UPDATE →
**PH_TICK_RUNNING with force=true** — every running goal's tick body
(state writes AND any RNG draws) executed between the update and the
next selector — a phase 26.3 never runs, both a state-write delta and a
within-tick SHARED_RANDOM serving reorder. The engine comment itself
("even: full tick()") already described the correct shape; the
implementation added the extra hop.

The hunt run's ledger pins the causal chain (the streamed job log, no
artifact): **SR state first diverges at t=141 in ALL THREE mob
scenarios** — the first tick the engine serves the fill's first batch —
then the burnUndead armor-wear gates (nextInt(2), SR) read shifted
positions (mobs t=142: +2 OFF-extra RemoveBinomial draws, k=94+), the
finalizeSpawn rolls shift, the jockey chickens appear (the t=147 n=92/91
+ id shift; the T382 snapshot's CHICKEN flips are the jockey mounts —
the fill spawns 100% zombies from a plugin-local Random(42L)), and the
entity-id streams explode. The previous run's t=143/145/146 LRC ticks
were cascade effects. The engine's own arithmetic is exonerated: javap
ThreadUnsafeRandom.advanceSeed (value*25214903917+11 & 2^48-1) and
BitRandomSource.nextInt(bound) (the exact biased loop) match the Lcg
mirror bit-for-bit — the divergence was ORDER, not values.

**The fix**: PH_UPDATE ends the even pass — SEL_TARGET's update falls
through to SEL_GOAL's CLEANUP, SEL_GOAL's update returns done. The odd
path keeps TICK_RUNNING with every_tick only. The BURN probe's extra
isSunBurnTick() call is entropy-neutral (javap: zero random calls in
the body) — the shipped-config parity on c1a0f75 (41670352580) stayed
GREEN with the weave live, the weave-neutrality check passed.

Instrument fixes in the same commit: the LRF frames print deepest-first
(the console handler truncated the depth-30 RemoveBinomial line at
~430 chars — exactly the caller window that names the path: burnUndead
vs finalizeSpawn vs the cramming hurt) and the whitelist carries
CE-INIT + BURN-LIVE (both were dropped from the streamed log: zero
CE-INIT lines in the hunt run was a whitelist artifact, not a dead
probe — the BURN t= lines proved the weave live).

Dispatched: the push-triggered shipped parity (the neutrality gate) +
the fixed-engine hunt (mob_ai=1, trace 83, window 138-152). If the SR
streams now hold through the fill and the window, the gate goes GREEN
and the AI engine becomes shippable — the doc-30 frontier item 0
(mobs -61% per e774d00, the single biggest module-owned move toward
-90%).

## L83 (b30936d): the zero-player findTarget resolves natively — the biggest staged cost dies

The AITR witness of the L81b hunt (run 41670844291, t=420): EVERY traced
mob's first staged round is `op=6 r=0` — REQ_FIND_TARGET, parked once per
mob per even tick. The gate that forced the park was
`ctx.pcount() > 0` on the player-typed native path: with an EMPTY player
snapshot (the parity scenarios, the player-less phases of the benches)
the engine skipped the native scan and parked — one full selector-state
mirror + JNI crossing per mob per even tick, ~2500 x ~400 = ~500k rounds
on mobs-25k (at the doc-30 ~300ns/round order: 150ms+ of the leg).

But ZERO players is the MOST provable case: getNearestPlayer iterates an
empty level.players() — the follow range, the TargetingConditions and
the Sensing LoS never matter, the result is null, and the null write
(ACT_REF_SET_PLAYER(-1)) + canUse=false is the whole observable. The
player-typed native path's own cands.is_empty() branch already performed
the identical write for the provably-empty-after-range case; the L83
gate extends it to the provably-empty-before-scan case. No RNG on either
side (the park drew nothing at zero players) — the streams are untouched;
the JNI budget collapses. Also in this window: L82b (the melee
requiresUpdateEveryTick fix — the Java fast path knew, the Rust side
didn't) and the profile.yml extra_env input (the engine-speed profiling
without workflow edits).

In flight: the L82 hunt (7f1f72f — the even-path fix, the first parity
verdict of the corrected engine) and the definitive hunt on b30936d
(L82+82b+83). If GREEN, the engine config is shippable and the next
frontier is the engine's remaining staged rounds (the stroll getPosition
park — REQ_STROLL_POS fires only when the 1/interval gate passes, ~1/60
per mob — negligible) and the travel/collide port (frontier 1).

## The t=141 root narrows to the engine — the LCG solve + the push=0 bisect

The L82 fix (the even-path phase order) did NOT move the divergence: the
L82 hunt (41674048550, 7f1f72f) still shows SR t=141 / LRC t=142, the PH
state holding to t=147, the same jockey shape. Two new facts close the
net:

1. **The LCG forward-solve.** The SHARED_RANDOM LCG (value*25214903917+11
   mod 2^48) is a permutation: iterating the ON leg's t=141-end state
   forward reaches the OFF leg's state in EXACTLY 17 steps, in BOTH hunt
   runs (41670844291 and 41674048550). The ON leg advanced the
   ENTITY-random stream 17 draws more by t=141 — not a single missed
   roll, a PER-MOB serving delta (~13 mobs ticking their first AI at
   t=140-141).
2. **The push=0 bisect** (41676736936, mob_ai=1 + CE_PUSH=0): the SR
   divergence at t=141 is UNCHANGED — the push kernel's cramming audit
   (the only kernel RNG surface) is EXONERATED.
3. **The AITR "t=420" line is CUR_TICK-space** (the module's flush
   counter, 3 per game tick) — CUR_TICK 420 = game tick 140: the
   engine's FIRST serving IS at t=140-141, exactly where the +17 lands.
   The module arms at boot, not at t=420.

The engine's own draw SITES were audited against the javap bodies: the
Lcg recurrence + nextInt(bound)'s biased loop are bit-exact
(ThreadUnsafeRandom.advanceSeed + BitRandomSource.nextInt), the stroll
gate packs positiveCeilDiv(interval,2) (reducedTickDelay) correctly, the
nearest gate takes the raw randomInterval, the lookat/lookaround floats
match, and the staged resolutions call the REAL virtual bodies
(MH_FIND_TARGET = the real findTarget, REQ_STROLL_POS = the real
getPosition). The +17 draws' CONSUMER is still unnamed — the SR state
probe (one line per tick) cannot attribute draws. The L85 instrument
(this commit) wraps SHARED_RANDOM with the same delegating proxy the LR
stream uses: the SRC count + the SRF per-draw caller frames (deepest-
first) in the window, in BOTH legs. The module's seed handoff reads and
writes the ORIGINAL object's 'value' field directly — the wrap is a
pure observer. The next SRF diff names the +17's caller directly.

Instrument archaeology (fixed): the SR/LR probes never stream before
t=141 because logParityHash early-returns on the EMPTY entity list
(the pre-fill ticks) — the whole entropy block lives after the emit,
not before it; and the "entropy-probe-disabled" lines never streamed
(the whitelist pattern "entropy probe" does not match the hyphenated
text) — both quirks masked the probe-start boundary for two runs.

## L85b (1d1a2d7): the SRF instrument was blind by construction — the wrap now installs at ENABLE

Run 307 (the L85 SRF hunt) streamed `SRC t=… calls=0` for every tick and
zero SRF frames. Not a dead probe — a blind one: **javap 26.3 shows
`Entity.random` is an instance field copied from the static in the
constructor** (`getstatic SHARED_RANDOM; putfield random`), and every
entity draw site reads the instance field (`getfield random`). The proxy
installed lazily at the first non-empty tick (t≈141) — AFTER the whole
fill batch was constructed — so every mob held the ORIGINAL reference and
its draws bypassed the proxy forever. The instrument could never see
anything regardless of the engine's behavior.

The fix (L85b): `installSharedRandomWrap` runs at **onEnable**, right
after the SHARED_RANDOM seed — before any entity exists — so every fill
mob captures the proxy. Two contract pieces keep the engine exact:

1. The SR proxy carries the `SrUnwrap` marker interface (defined in the
   harness): `ceSrOriginal()` returns the wrapped ORIGINAL.
2. `CeHooks.initMobAiHandles` unwraps duck-typed by the method name
   (setAccessible waives the cross-loader visibility check; no harness
   class link). The seed handoff keeps reading/writing the ORIGINAL's
   `value` — one underlying LCG, the proxy only watches.

Neutrality proof: the push-triggered shipped-config parity on 1d1a2d7
must stay GREEN (the wrap delegates every call to the same object; the
OFF leg proxies too). The dispatched hunt (mob_ai=1, trace 83, window
138-152) now streams real SRC counts + SRF frames in both legs — the
t=141 SRC diff either names the +17's Java consumer directly or (if the
Java counts match) pins the +17 on the engine's staged Rust serving.

## L86 (hotspot ledger, engine mode, mobs-25k @ ~215ms): the vanilla lookup cluster is the next lever

The lean profile (CE_MOB_AI=1, bench bot present, 11546 samples):

| cluster | self-time | shape |
|---|---:|---|
| vanilla AABB scan | ~14% | `AABB.intersects` 8.58% + `getBoundingBox` 2.94% + `ListItr.next` 2.30%, called from `ChunkEntitySlices$EntityCollectionBySection.getEntities` (3.05%) |
| targeting | ~4% | `HurtByTargetGoal.start/alertOthers` 2.10+1.59+0.30% — real class-based scans (`getEntitiesOfClass`) |
| attributes | ~3.5% | `AttributeMap.getValue` 1.84% ← `LivingEntity.travel` (1.65% caller) |
| module engine hooks | ~4.5% | aiPackSel 0.67, mobTravelNative 0.88, aiMeleeRunning 0.49, sensingTick$ 0.54, aiClassify 0.40, mobBlockEffectsNoOp 1.44, tickEffectsNoOp 0.42 |
| TTL tracker | ~1.2% | newTrackerTick$ 0.67 caller + ttlCachedPath 0.48 |

With a player present the L83 zero-player findTarget fast path never
fires — the staged findTarget resolves through the REAL vanilla body:
`TargetingConditions` → moonrise `ChunkEntitySlices.getEntities(Class,
Entity, AABB, List, Predicate)` → the AABB.intersects leaf. The
**native targeting port** (module index scan in the exact vanilla order +
condition evaluation) is the next staged-round candidate: it deletes the
~14% lookup cluster per scan and keeps the engine's staged ledger exact.

items-100k-mod (30ms p50, −60%) top costs: `guardEntityTick` 8.15% (the
vanilla walk itself), `itemTickFlush` 6.29+1.25+1.21+0.82%, the TTL
tracker cluster 11.3% (ttlCachedPath ×4 lines), `Entity.isRemoved` 4.51%,
`appendDispatch` 3.95%. The TTL loop pays ~10 loads × 100k entities/tick
to re-prove per-entity quietness; a Rust-side batched quiet-scan
(parallel id array → candidate bitset) or an epoch-dirty skip list is
the next items lever.

## L88 (bdb9f4f): the SRF instrument finally names the t=141 divergence — the BURN probe drew entity-randoms

The L85b instrument chain is now fully live (enable-time install +
the defining-loader fix + the whitelist): the L86 hunt (d7fc699) streamed
real SRC counts — 600/tick in the items window, ~208 (OFF) vs ~133
(ON-visible) in the mobs window — and the SRF frames at t=141 named the
divergence directly.

**The smoking gun**: `x13 Monster.aiStep → Mob.aiStep → Mob.burnUndead →
CeHooks.probeBurn → Mob.isSunBurnTick → Proxy.nextFloat` — ON-leg-only
frames at the EXACT divergence tick. javap 26.3: **Mob.isSunBurnTick
draws `random.nextFloat()` whenever getLightLevelDependentMagicValue() >
0.5** (bc 92-96: getfield random; invokeinterface RandomSource.nextFloat)
— the doc-41 "the BURN probe's extra isSunBurnTick() call is
entropy-neutral (javap: zero random calls in the body)" audit was WRONG
for this version. The window-gated probe drew +1 entity-random per lit
mob per window tick, ON leg only (the OFF leg's Mob class is never woven
— precompute_mob declines). The f^17 solve (ON = OFF + 17 at t=141)
decomposes as: +13 the probe's extra isSunBurnTick draws, +~2 the
engine's first-serving goal-start rolls (RandomLookAroundGoal.start
nextDouble+nextInt where vanilla started none at t=141), +~1-2 the
engine's Rust-side canUse gate draws vs vanilla's (the OFF-only SRF
groups name the exact vanilla rolls the engine replaces:
x52 NearestAttackableTargetGoal.canUse nextInt, x13 LookAtPlayerGoal
canUse nextFloat, x13 RandomLookAroundGoal canUse nextFloat, x13
RandomStrollGoal canUse nextInt at t=141 — 91 rolls the engine serves
Rust-side as 92).

**Fix**: the BURN line keeps fire/hp/type (entropy-free) and drops the
burn field. The probe only weaves into the mob_ai=1 legs, so the
shipped-config parity was never affected — but every hunt so far carried
this bias inside its own dump window.

**The road to the engine gate**: with the probe neutral, the next hunt's
SRC/SRF diff at t=141 isolates the engine's RESIDUAL delta — the
Rust-side canUse gate draw counts vs vanilla's 91-roll sequence and the
goal-start ordering. The SRF frames give both legs' full per-draw caller
multisets, so the reconciliation is mechanical: for each vanilla roll
group the engine must consume the same count in the same per-mob order.
Once the hunt gate goes GREEN the AI engine becomes shippable — doc-30
frontier 0 (mobs −61% per e774d00, the single biggest module-owned move
toward −90%).

### L88 ledger (the full t=141 multiset, hunt 41712408901)

The complete SRF multiset at the divergence tick (mobs, OFF 208 vs ON
133 proxy-visible draws):

| group | OFF | ON visible | verdict |
|---|---:|---:|---|
| spawn path (ctor UUIDs, finalizeSpawn, randomizeData) | 91 | 91 | EXACT |
| GoalSelector canUse interval rolls (52 target + 13 lookat + 13 lookaround + 13 stroll) | 91 | 0 (served Rust-side) | engine ledger R=92 |
| burnUndead real (isSunBurnTick nextFloat + ignite) | 26 | 26 | EXACT |
| Zombie.tick base-tick group | 13 | 13 | EXACT |
| probeBurn EXTRA isSunBurnTick | — | 13 | **THE PROBE (+13)** |
| RandomLookAroundGoal.start (nextDouble+nextInt) | 0 | 2 | engine started a goal vanilla didn't |

Reconciliation: ON_true = 133 visible + 92 Rust = 225 = 208 + 17 (the
f^17 solve verified end-to-end). The +17 = probe +13 + canUse +1 + starts
+2 (+1 rounding in the cut-string groups).

**The cascade hypothesis**: the probe's +13 landed MID-TICK (burnUndead
runs per mob inside the same tick as the goal gates), shifting the LCG
stream for every later consumer — the engine's +1 gate roll and the +2
start rolls may both be CASCADE effects of reading shifted positions,
not independent bugs. The L88 hunt (probe neutral) is the decisive read:
if the engine's Rust ledger lands exactly 91==91 with matching starts,
the engine is parity-exact and shippable (doc-30 frontier 0, mobs −61%);
any residual delta is now attributable frame-by-frame.

## L89 (this commit): the L88 decisive read — t=141 is DEAD, the front moved to t=145 (m = −27, the stroll-start split)

The probe-neutral hunt (run 41713824607, TRACE_ID=83, PH_DUMP=138-152,
mob_ai=1 — the first hunt with the L88 entropy fix live) closed the L88
ledger exactly as predicted and moved the frontier:

**t=141 is EXACT.** Both streams lock tick-by-tick: SR/LR at t=141-144 are
bit-identical across the legs (2469881e26ae / 65be53106748 … through
60c556ead8d5 / fc99facf6375). The +13 probe draws were the whole t=141
cascade: with the probe entropy-free the engine's canUse +1 and the
start +2 vanished — the cascade hypothesis CONFIRMED.

**The new first divergence: t=145, entity stream only.** LR stays locked
(92 == 92 Level draws at t=145; the Level stream never splits in this
window). SR splits at t=145: OFF end-state 55e72a9954cf, ON 8fd42ee257f2.
The f-solve over the legacy LCG (a=25214903917, c=11, mod 2^48) gives
**m = −27** — the ON leg consumed 27 FEWER entity-random draws at t=145
(ON total 510 = 223 visible + 287 Rust-served vs OFF 537).

**The visible-stream LCS ledger** (OFF's 247 non-canUse frames vs ON's
223, longest-common-subsequence):
- OFF-only ×30 `RandomPos.generateRandomDirection nextInt` + ×1
  `WaterAvoidingRandomStrollGoal.getPosition nextFloat` — vanilla ran
  STROLL STARTS (position generation) the engine never did;
- ON-only ×1 `RandomLookAroundGoal.start` pair — the engine started a
  lookaround vanilla did not;
- net −24 visible draws, and the remaining −3 are the gate rolls.

Reconciliation: m = −27 = −24 (starts) − 3 (canUse rolls). Vanilla
canUse = 290 (537 − 247), engine Rust-served = 287 (510 − 223).

**The instrument (this commit)**: the L89 GATE LEDGER — per-class canUse
census (evals/No/Staged for the 9 GK_* classes) counted Rust-side in
relaxed atomics, armed per-tick by `nativeGlTake(phDumpTickNow())` at the
itemTickFlush boundary (one JNI crossing per level-tick, zero cost
outside the window, read+reset in the same call), printed as
`GL t=<game> c<k>=evals/no/staged`. The GL line against the OFF SRF
census (canUse frames per class + start frames) names the gate whose
verdict split at t=145 — the candidates are the stroll gate
(noActionTime/passenger flag short-circuits) and the target gate's
pcount==0 fast path.

Zombie target-selector fact sheet (javap 26.3): FOUR
NearestAttackableTargetGoals — prio2 Player (randomInterval 1), prio3
AbstractVillager (0), prio3 IronGolem (1), prio5 Turtle (10); the
interval roll draws nextInt ONLY when randomInterval > 0; findTarget for
non-Player types goes through level.getEntitiesOfClass (the L86 funnel).
The Player goal at pcount==0 consumes exactly 1 roll (interval) + 0
(getNearestPlayer over an empty players() list).

## L90 (this commit): the GLV census + the sc= tags — the OFF side of the t=145 split is finally countable, and the canUse-eval SET is one tick early

Two instrument defects blocked the L89 read: the SRF frames cap at 512
entries (the fill's spawn noise — 2572 frames at mobs t=145 — left only
a biased prefix of the AI frames, so the vanilla-side per-gate census
was arithmetic noise) and the 8 interleaved consoles shared one job log
with no per-scenario marker (the three 43/18/33-scale GL t=145 lines
were unattributable). The fix: the GLV census — a per-CALLSITE draw
counter keyed by token[1] of the stored frame (RandomStrollGoal.canUse,
RandomPos.generateRandomDirection, ...), cap-free, printed BOTH legs as
`GLV t=<tick> sc=<RUN_NAME> n=<total> <Class.method>=<n> ...` — and the
`sc=` suffix on every streamed census/state line (SRC/SR/LRC/LR/GLV/GL).

**The GLV totals are EXACT**: the per-site sums equal the SRC calls=
totals on every tick (mobs OFF t=145: 285 canUse + 31 posgen + 221
spawn/baseTick = 537 ✓).

**The corrected fact sheet (javap 26.3, the doc-41 L89 sheet was
WRONG)**: `NearestAttackableTargetGoal`'s 3-arg ctor passes
`bipush 10` — DEFAULT_RANDOM_INTERVAL = 10. The zombie's FOUR NAT goals
are Player(10, prio2), AbstractVillager(10, prio3, behind the
zombieAggressiveTowardsVillager spigot flag), IronGolem(10, prio3),
Turtle(10, prio5) — EVERY goal rolls every eval; the engine's GL
k8=172/172 for 43 mobs (4 per mob) is CORRECT. The vanilla target
census = 4 × AI'd-mobs − (4 × mobs whose whole selector skipped).

**The AI-cadence reference set** (javap Mob.serverAiStep bc 75-148:
`sum = tickCount + getId(); if (sum % 2 != 0 && tickCount > 1) {
tickRunningGoals(false) ×2 } else { tick() ×2 }`; tickCount is
POST-increment at serverAiStep — LivingEntity.tick bc 1 runs
Entity.tick (baseTick, tickCount++) before bc 178 aiStep):

    AI'd(t) = { mob : tickCount<=1 or (tickCount+id) even }

With the fill spawning 13 zombies/tick from t=141 (ids 1..13, 14..26,
...), the model reproduces the OFF census EXACTLY: t=142 → 19 (b1 tc=2
even-ids 6 + b2 tc=1 all 13), t=143 → 26, t=144 → 31, t=146 → 43.

**THE ENGINE'S SET RUNS ONE TICK EARLY** (mobs, GL k4/k5/k6 evals vs
the model):

| t | population | vanilla AI'd | engine evals |
|---:|---:|---:|---:|
| 141 | 13 | 13 | 13 |
| 142 | 26 | 19 | 27 |
| 143 | 39 | 26 | 31 |
| 144 | 52 | 31 | 41 |
| 145 | 65 | 41 | 43 |
| 146 | 78 | 43 | 55 |
| 147 | 91 | 55 | 55 |

engine-set(t) == vanilla-set(t+1) for five consecutive ticks. The
shifted set re-rolls gates vanilla did not eval that tick (the
stream's CONSUMER ORDER differs), and at t=145 it flipped the verdicts:
vanilla ran the steady-state single-stroll-start search loop (31 posgen
draws — RandomPos.generateRandomDirection 30 + getPosition 1, ONE new
start per tick, each start's navigation instantly done so the goal
stops and re-evals next tick); the engine staged ONE stroll and ran
ZERO posgen draws — the −31 (+2 engine-only lookaround-start) visible
delta the L89 f-solve read as m=−27.

The t=143 burnUndead census (off=5 on=4) is the same story one tick
earlier: the shifted set drew the burn group one draw short and one
gate roll long — the SR LCG END-STATE stayed equal (the state after N
advances is unique — the totals matched even while the consumer order
did not), which is why the SR probe stayed green through t=144 while
the tick was already serving the wrong mob set.

**L90b (this commit): the branch counters** — the GL line prints
`br=<even-ai>/<odd-java>/<odd-rust>` (per-tick, window-gated, two
integer adds per mob). The even counter IS the engine's AI'd set — the
next hunt run names the divergence side directly.

## L91 (008275e): the br counters CONFIRM the one-tick-early verdict — and the tc-phase instruments aim at the mechanism

The ab9b8f2 hunt (41795650141, the shipped knobs + mob_ai=1,
TRACE_ID=15, PH_DUMP=141-150) failed the parity gate with the SAME
families as before — mobs 2 only-off/14 type swaps, mixed 883/915/
2289, spread 99/101/4573, items bit-exact — and its L90b br lines +
GLV censuses settle the L90 question with independent instruments:

- OFF canUse census (vanilla truth, NAT.canUse/4): AI'd(t) =
  13,19,27,31,41,43 at t=141..146 (baseTick 26,39,52,65,78,91 — the
  fill runs 13/tick from t=140).
- ON br even: 13,19,27,31,41,43,55 at t=140..146 = vanilla(t+1).
  **The engine's AI'd set runs one tick early, now measured by the
  Java-side guard counters themselves.**
- mobs PD hashes: bit-identical through t=150 — the AI'd-set shift is
  STATE-invisible until the first stroll start moves a zombie (SR
  splits at t=145 exactly, the L89/L90 verdict reproduced).
- mixed: SR+LR split at t=146; the first PD diff at t=147 hits the
  t=147 spawn batch (ids 451+, 55 of 525) — the spawn stream itself
  diverged mid-fill. spread: SR t=145, LR t=143 (!).

The cadence model that fits BOTH sequences (simulation over
s0/first-offset/gate-shift variants): vanilla gate reads tc_pre at the
mob's first ticker tick s+1; the engine effectively reads tc_post and
reaches the fresh batch one tick earlier — EITHER a +1 tickCount
phase, OR the fresh batch entering the engine's sweep at the spawn
tick, OR both. The three instruments shipped here discriminate them
in ONE run:

1. **PH_TC** — Entity.tickCount joins the per-entity PD avalanche at
   dump ticks (rotate 83): the first tick where the legs' tc phases
   split lights up in the (id -> m) diff with no post-processing.
2. **TC line** — `TC t= sc= n= tc0= tc1= tcmin= tcmax=` per dump tick
   in BOTH legs (same plugin): the tc0/tc1 cohorts name the fresh
   batch's first AI tick (spawn tick vs s+1) per leg directly.
3. **GL gr=** — the guard-reach census (entries / tc<=1 entries) on
   the ON leg, comparable with the OFF canUse census.

Also fixed en route: the sandbox was recycled between sessions (cargo
and JDK 25 gone; Temurin 25 must be re-provisioned before any local
module build), and the stale parity-order WIP in the working tree was
a superseded duplicate of the landed doc-36b feature — discarded via
reset to origin/main.

## L92 (beeb480): the tracker-quiet epoch — the items TTL cluster's structural fix

items-100k-mod JFR (5057 samples): the ttlCachedPath cluster is
**15.6%** of the 28.6ms p50 (11.86 the identity/prologue line + 2.10
+ 0.95 + 0.73 across the SIT/syncVelocity reads) — the probe chain
re-proves per-entity quietness ~100k times per tick for a verdict the
item-tick applier already computed THIS tick.

The design (Java-only): the applier stamps a TT_QUIET bitset for
IF_NATIVE TAIL_NONE entries (the fill's TT_OK verdict still pinned, no
SIT handler, !syncVelocity, the L70 mirror owning the counter) and
performs the ServerEntity.tickCount mirror bump INLINE; the tracker
pass skips the stamped slots (quietArm = cacheArm && TT_MIRROR read
after the reconcile). Cleared at every flush entry — a slot without
this tick's stamp is conservatively full-path; non-owned entities
never get stamps; -Dcentities.ttlQuiet=false disarms.

Byte-exactness ledger for the skipped work: syncVelocity consume
(false — checked), SIT bookkeeping (excluded — checked),
updateDataBeforeSync (javap: an EMPTY stub in 26.3 — the call IS the
whole effect), the tickCount bump (the same +=1 the applier performed).
The parity legs run playerless (cacheArm armed), so the gate exercises
the path end-to-end.

## L94 (9fdda23): THE +-1-TICK FILL-START RACE - the parity harness itself was the divergence

The tc-phase instruments (L91) + a LOCAL A/B (the sandbox can boot both
legs now: JDK 25 + the mapped jar + the CRUSSTY runtime, ~2 min/leg)
settled what three CI hunts could not:

**The legs' fill phases were never tick-aligned.** begin() registered
fillStep with the scheduler's RELATIVE 40-tick delay; its first-fire
tick depends on where within the boot the begin() poll crossed its
tickNo threshold. Two local boots of the SAME seed put the first spawn
batch at tickNo 141 (OFF) and 140 (ON).

Every L90 signature follows mechanically:
- The shared-RNG re-seed pins the stream to the FILL COUNT, not the
  absolute tick, so the SR end-states stay locked through the fill
  while the entities' tickCount phases differ by one REAL tick.
- The (tickCount+id) cadence gate then classifies the same game tick
  differently: OFF census(141)=13 with ONE batch (its fill at 141) vs
  ON br even(141)=19 with TWO batches (its fill at 140) - BOTH are the
  vanilla model applied to their own fill phase. The "engine-set(t) ==
  vanilla-set(t+1)" verdict compared the legs' models across a one-tick
  fill offset; the ENGINE'S OWN SET WAS EXACT ALL ALONG (the local A/B
  with the fill pinned: the engine's br even = the OFF census
  tick-for-tick, 3=3, 4=4, 5=5, 6=6, 7=7, 8=8).
- The PD hashes stayed identical (positions don't move until a goal
  starts) while the SR split at the first goal start (t=145 in the CI
  hunts' 13/tick fill = the first stroll-start cohort).

The fix: fillAnchorTick = tickNo at begin() + 40; fillStep polls the
absolute anchor. Local A/B after the fix: the SR end-states match
tick-by-tick through t=154; the first REAL divergence is t=155, a
single +1 entity-RNG draw (the LCG state diff solved from the SR
end-states: off 194 vs on 195 advances), which surfaces at t=162 as
entity 30's yaw/vx (the engine started a lookaround vanilla did not,
on the shifted stream). That residual is the mob-AI engine's own bug,
now bisectable in minutes locally instead of CI round-trips.

Also shipped en route:
- L92: the tracker-quiet epoch (the applier stamps TAIL_NONE items,
  the tracker pass skips them) - the shipped-config parity gate went
  GREEN on 655b4be with it live.
- L93: the native-funnel findTarget (javap facts: findTarget =
  getEntitiesOfClass(type, inflate(fd,fd,fd), TRUE) + getNearestEntity
  (first strict-min over test()); the module index answers the class
  scan in bit-identical vanilla order).

## L95 (this session): the FULL-SCALE local A/B — the sets are EXACT, the residual is the update-pass micro-semantics

The sandbox was rebuilt from zero (rustup stable 1.99.1 + Temurin
25.0.5 + maven-3.9.9 via the aliyun mirror after central 429s + the
paperclip mapped jar) and ran the L94 local A/B at FULL scale
(SCALE=2500, fill 13/tick, CE_MOB_AI=1, LRC_WINDOW=144-160,
LRF_DEPTH=30, both legs clean-room booted, world wiped between runs).
The first SR split reproduces exactly: **t=145, off=55e72a9954cf,
on=8fd42ee257f2** — the CI hunts' signature — with LR split at t=147
and PH at t=151.

**The one-tick-early verdict (L90b) is DEAD — an artifact stack.** The
GLV census column I first built (target-canUse draws / 4) said
"engine even(t) == vanilla(t+1) for 8 consecutive ticks" — but the
census counts DRAWS, not evals: vanilla's flag-conflict skips remove
evals that do not draw, so the divisor-derived "set" undercounts
exactly when the skip semantics differ. The javap cadence model
(cohorts 13/tick from t=140, ids strictly sequential, set(t) =
{tc<=1} U {(tc+id)%2==0}) gives set(145)=43, set(146)=55, set(147)=55,
set(148)=68 — and the engine's own GL br counters read
43/35, 55/36, 55/49, 68/49, 69/61, 80/63, 83/73, 92/77 —
**tick-for-tick EXACT against the model.** The L94 fill-pin holds; the
engine's even/odd classification is exact at full scale.

**The real residual (all inside ONE update pass, t=145, 78 live mobs,
43 even):** the engine's GL gate ledger (1=43/43/0/0 … 8=172/172/0/172)
against the OFF SRF census names four deltas:
1. **+8 target-gate evals** (engine 172 = 43x4; vanilla DREW 164): two
   mobs' target-selector update ran in the engine where vanilla's
   lockedFlags skipped it — the engine's flag-conflict gate does not
   reproduce the running-goal TARGET-flag skip.
2. **+2 stroll evals** (engine 43, vanilla drew 41): same skip family
   in the goal selector.
3. **-3 gate-roll values**: the canUse draws the engine served read
   shifted stream positions (the within-pass serving order is not
   vanilla's), so the stroll interval rolls flipped: vanilla said YES
   on mobs where the engine said NO.
4. **-31 posgen +2 lookaround starts**: the cascade of (3) — vanilla
   ran 30 RandomPos.generateRandomDirection + 1 WaterAvoiding
   getPosition starts the engine never did (m = -27 reconciliation:
   -31 +2 +2 net = the L89 ledger exactly), and 2 lookarounds STARTED
   in the engine (java-side RandomLookAroundGoal.start=2 in the ON
   GLV) that vanilla's stroll-start had blocked.

**The fix surface is now one function**: the engine's update pass
(goals.rs, the canUse eval loop) must reproduce vanilla
GoalSelector.tick()'s (a) lockedFlags skip before canUse, (b) exact
eval order across the selector pair, (c) the staged stroll posgen
serving. The verification loop is FULLY LOCAL now: build (~10 s) +
two legs (~4 min each) + srlr_first_div.py + the GL/GLV/SRF diff —
minutes per iteration instead of CI round-trips.

## L96 (this session): THE EQUAL-PRIORITY LOCK FIX — the t=145 split is GONE, the frontier moved to t=159

javap on the mapped jar settled the L95 ledger's root cause:
`WrappedGoal.canBeReplacedBy` (26.3) is
`return this.isInterruptable() && other.getPriority() < this.getPriority();`
— STRICT. The engine's `can_replace_all` transcribed `<=` (equal-
priority replacement ALLOWED). The 26.3 zombie set carries TWO
equal-priority same-flag pairs — LookAtPlayer(8)/RandomLookAround(8)
on LOOK and NearestAttackable(Villager,3)/NearestAttackable(Golem,3)
on TARGET (registerGoals/addBehaviourGoals bytecode) — so every
equal-priority refusal vanilla makes was an eval+start in the engine.
isInterruptable is a no-op for the classified set (zero overrides,
javap across all nine goal classes).

The fix: `can_replace_all` returns false when
`candidate.prio >= holder.prio` (strict-only replacement), the
`even_tick_never_ticks_running_goals` reference corrected to the
4-draw vanilla model (3 target + stroll; the lookat(8) pull is
REFUSED by the running lookaround(8)'s lock — the old 5-draw model
encoded the bug). All 41 core tests green.

Local A/B (SCALE=2500, fill 13/tick, CE_MOB_AI=1): **SR identical
t=141..158 — the t=145 split (m=-27, the L89 ledger) is DEAD.** The
new first split: t=159 (260 live mobs, 133 even) — the NEXT residual,
inverted shape: the engine now runs the starts vanilla refuses —
62 java-side posgen draws (RandomPos=60 + WaterAvoiding=2) vs
vanilla's ZERO, 6 lookaround starts vs 2, +1 burnUndead draw, while
the target gates match EXACTLY (class 8: 532 evals == 532 vanilla
draws). The stroll gate rolls read shifted stream positions for 2 of
133 mobs — the within-tick serving order diverges once goals are
RUNNING (the t=145-158 pass was all-idle mobs). The hunt continues:
the staged-request continuation draws (park → java continuation →
resume) must land at the exact vanilla walk position once the mob's
serve spans multiple JNI rounds.

## L97-hunt (session 2026-11-05T18:xxZ): the t=159 SR split DECOMPOSED — the seed pair is (107, 139) at t=157, pre-draw

CI bisect run (dispatch 18:11:27Z, e106368, shipped config mob_ai=1, ph_dump=138-170,
LRC_WINDOW=138-170, LRF_DEPTH=14) — the live-log readout:

- The push-triggered parity on L96 (run 42075260274, mob_ai=0) is FULL GREEN:
  all 12 snapshot comparisons PASS bit-identical — the module is exact once
  the native AI floor is off. The residual lives ONLY in the AI floor.
- The shipped-config dispatch (42075292474, mob_ai=1): mobs/items... FAILs —
  the T382 snapshot shows off=2509 on=2508 with 12 CHICKEN↔ZOMBIE type
  mismatches — the id-stream shifted AFTER t=165 (PD hashes identical
  through t=165 in the 140-165 window).
- The new window (138-170) caught the REAL first split: **PD t=157, ids
  107 and 139 ONLY** (their x/y/z/v/tc avalanche differs), with the SR
  end-state still IDENTICAL through t=158. The draw avalanche (SR split,
  +62 posgen, 6 vs 2 lookaround starts) lands on t=159+ — it is the
  CONSEQUENCE of the two-mob tc/phase drift, not the seed.
- SRF t=159 morphology: OFF streams [baseTick → target×4 → lookat →
  lookaround → stroll → sunburn/burnUndead] per mob (all gates are Java
  draws); ON streams [sunburn/baseTick...] with ZERO Java gate draws
  (gates are NATIVE — the LCG handoff covers them) + 60 RandomPos + 2
  WaterAvoiding + 6 RLA.start. GLV off-vs-on comparison is therefore NOT
  apples-to-apples (vanilla counts Java gate draws, the engine's gate
  draws are invisible to the proxy).
- items-100k: the SAME run's A/B is p50 64.98→29.98 (−53.9%) at L96
  (mobs −31.0%, mixed −41.7%, spread −19.9%). The perf residual per JFR:
  fastQuery/rowQuery/ordSortVanilla ≈ 17% (mobs-mod) — the query path,
  not the tracker (ttlCachedPath 1-2.7%), is the next −90% front.

Next: trace_id=107 (ph_dump=152-162, LRC_WINDOW=152-162) dispatch 18:26:55Z —
the per-tick field trace names the exact diverging FIELD of the seed pair
(tc-phase at the first even tick vs a drawless nav/pos move). If the trace
says tc: the cadence classification of the 50/tick CI fill cohort (spawn
tick 143 → first even 145) has a one-tick serving skew in the engine's
packer; if x/v: a drawless movement divergence — chase the nav mirroring.

## L97.1 (session 19:00-20:10Z): THE t=157 SPLIT DECOMPOSED TO +8 TARGET ROLLS — 2 mobs the vanilla skipped and the engine evaluated

The FULL local repro (byte-exact: SR t=159 off=4db7f953bdae on=34322a937683 —
the CI hashes) made this a minutes-per-iteration loop:

- PD first split: **t=157, ids 107 and 139 ONLY** (x/vx/yaw differ; tc, y, z
  identical). Vanilla mob 107 MOVES from t=157 (stroll start), the engine
  stands; the SR end-state stays identical through t=158 (per-mob roll
  redistribution is invisible to the tick-end hash; the advance-parity
  breaks at t=159).
- SRF flows at t=157 (plugin frame cap raised 512→4096): cleaned flows
  (gate-draw frames removed from BOTH) are IDENTICAL through cleaned-310:
  there vanilla calls `getPosition → nextFloat` (the stroll posgen of a
  start) where the engine goes straight to the next mob's sunburn. So the
  engine's mob X stroll roll did NOT return 0 while vanilla's did.
- The roll-count ledger: vanilla even-мobs on t=157 = 119 (119 complete
  4-roll NearestTarget runs; +1 mob with lookat-but-zero-target = 2 skipped
  mobs total; lookat 116 = 119-3 running-LOOK skips; stroll 114 = -5
  running-MOVE skips) — the engine's GL ledger k=8 rolls = 484 = 121×4:
  **the engine evaluated 2 more mobs than vanilla. Those 2 extra mobs'
  rolls shifted every subsequent roll position in the tick → the stroll
  rolls of ~2 later mobs flipped → the observed start-set swap (107 moves
  in vanilla, stands in the engine).**
- The vanilla skip is NOT lockedFlags: nobody in the classified set can be
  RUNNING with FL_TARGET (HurtBy needs lastHurtByMob, Nearest can never
  start on a null findTarget; javap canUse: the interval roll is ALWAYS
  drawn when canUse is called — so the 2 mobs' canUse was NEVER CALLED by
  vanilla's GoalSelector).
- javap Mob.serverAiStep (26.3, mapped): cadence gate is
  `sum = tickCount + getId(); sum % 2 != 0 → odd branch` — read from the
  SAME mob.tickCount the engine's aiPack passes. PTRACE (tick end) shows
  tc=9 in BOTH legs; the engine classified (9+107) even. Vanilla's skip
  therefore points at these 2 mobs NOT reaching the even branch inside
  serverAiStep despite the same end-of-tick tc — the remaining suspects:
  (a) the Paper `aware` early-return (bc 10-48: !aware → FloatGoal +
  jumpControl + RETURN — no selectors, no cadence), (b) a passenger /
  double-tick subtlety, (c) the dispatcher's commonTick++ ordering vs
  aiStep read.
- NEXT (the mechanical bisect): (1) javap who writes Mob.aware and when
  (Paper nerf-spawner paths), (2) a CI dispatch ai_floor=0 (GoalSelector
  floor only) vs mob_ai=0 — isolates which floor introduces the skip, (3)
  if aware: the engine's aiClassify must reproduce the !aware early-return
  BEFORE the cadence gate (and the fill-probe must not differ).

Also shipped here: plugin SRF frame cap 512→4096 (both recLrDraw/recSrDraw)
— the full-tick flow now fits the artifact; local A/B harness steps are
recorded in the session (build-plugin.sh + mvn aliyun mirror + run-baseline
legs MODULE=off/on, SCALE=2500, FILL_TICKS=200 → 13/tick fill, seed 4242 —
reproduces the CI entity-id stream exactly).

## L97.2 (session 20:30-21:15Z): TWO HARD FACTS from the selector-state dump

Plugin PTRACE now streams `targetSelector=[...] goalSelector=[...]` (the
lockedFlags map + isRunning) for PARITY_TRACE_ID in BOTH legs (L97.2 patch:
findField walk + callIsRunning helpers). Local A/B (l97h/l97i runs):

1. **ENGINE BUG (must fix): the engine's start path never fills the vanilla
   lockedFlags map.** aiApply ACT_GOAL_START calls WrappedGoal.start()
   (sets isRunning + goal.start()) — but vanilla's update loop sets
   `lockedFlags.put(flag, goal)` BEFORE start() (javap bc 211-265), and the
   displaced holder's stop() rides the same walk. The engine's displaced
   stops + starts are correct, but the map stays EMPTY (PTRACE on t=157:
   goalSelector=[] while vanilla shows MOVE=Stroll=true). Consequences:
   the vanilla-side cleanup (lockedFlags.removeIf), the
   canBeReplacedForAllFlags checks of any OTHER code path reading the map,
   and the A/B legs' observable selector state diverge from the first
   engine start ever. Fix surface: the engine must emit an explicit
   ACT_LOCK_PUT (flag→goal map write) or the floor must call a
   GoalSelector-shaped helper instead of bare wg.start(); the lock table
   the engine keeps internally is already right.

2. **t=157 resolved further:** vanilla mob 107: goalSelector=[MOVE=Stroll
   running=true] from t=157 (its stroll start), targetSelector=[] ALWAYS
   (149-160) — so the earlier "2 vanilla-skipped target mobs" CAN NOT be
   lockedFlags skips: the vanilla TARGET lock map is empty all along. The
   engine's RLA start (gi=7, AITRACT START cls=RandomLookAroundGoal) DID
   run wg.start() — but the engine started the RLA where vanilla started
   the stroll (different rolls at shifted stream positions — the +8
   target-roll shift is still the top-level drift). Remaining mystery: why
   vanilla evaluated only 119 even mobs' target gates while the engine
   evaluated 121 (tc=9, (tc+id)%2==0 in both legs per PTRACE) — next
   suspects: Paper's entity-activation-range inactive rotation (no players
   in the parity world — check the AI-tick cadence the vanilla leg
   actually serves) and the passenger paths. The activation hypothesis
   would also explain why the even-count looks like a rotation slice
   rather than a plain cadence half.

Next session: (1) fix the ACT lock-map write (parity surface), (2) count
the vanilla leg's REAL even-mob AI-tick set vs the (tc+id) model — if the
vanilla set is the Paper inactive-rotation slice, the engine's serverAiStep
floor must gate on the same activation state before the cadence gate.

## L97.3 (session 14:00Z): THE LOCK-MAP WRITES — vanilla lockedFlags maintained on EVERY engine path

The L97.2 must-fix landed: every engine-side goal start/stop now replays its
paired `lockedFlags` write. Vanilla's GoalSelector.update owns the map
(javap bc 211-265: flag→goal put BEFORE start(); the !isRunning removeIf in
goalCleanup) — WrappedGoal.start() itself never writes it, so the engine's
bare wg.start() left the map EMPTY from the first engine start ever (L97.2
PTRACE: goalSelector=[] while vanilla shows MOVE=Stroll=true).

- aiApply (the engine path): ACT_GOAL_START → per-flag put before start;
  ACT_GOAL_STOP → the paired two-arg remove before stop (a no-op when a
  later start already replaced the value — the same order vanilla's walk
  produces; the engine's displaced stops precede their starts, so the
  remove→put sequence mirrors the vanilla walk exactly).
- The floors: fullPassTick + the fast path now maintain the map too —
  per-stop remove, the goalCleanup !isRunning removeIf (the mirror's port,
  covers externally stopped goals the shadow walk never visits), and the
  per-flag put in the update walk BEFORE start (displaced cur.stop() rides
  above it, same as vanilla). The mirrors already wrote the map — unchanged.
- New helpers: lockedOf(sel) (null-safe VH_GS_LOCKED.getObj) and
  lockWrite(locked, wg, put) — the backing-set walk of the mirrors
  (Paper's OptimizedSmallEnumSet), flag objects from the GOAL_FLAGS cache.
- Zero-cost on the hot path: writes ride start/stop events (rare) + one
  removeIf per selector tick over a 0-2 entry EnumMap; the engine's lock
  table stays the decision-maker — the map is the parity surface, the
  kill-switch guarantee and the A/B selector-state evidence.

NEXT: the CI parity re-run now dumps a NON-EMPTY goalSelector map in BOTH
legs' PTRACE — the A/B selector-state comparison becomes apples-to-apples
for the first time. Then the L97.2 item (2): the vanilla leg's REAL
even-mob AI-tick set vs the (tc+id) model (the Paper inactive-rotation
hypothesis) decides whether the engine's serverAiStep floor must gate on
the activation state before the cadence gate.

## L97.4 (session 15:00Z): THE ORDER LEDGER — prescan-skip + the first-diff prints

The mobs-25k-mod profile (e73c42c-era) still burns `lambda$ordSortVanilla$0`
4.18% + TimSort.countRunAndMakeAscending 0.99% AFTER the L58b index-order
work — real TimSort merges mean the native query's id stream and the live
Java ORD_SEQ disagree somewhere (the index sorts by its STORED stamps; the
Java comparator sorts by ORD_SEQ — a stale-stamp window exists). Two moves
in one commit:

- **ordSortVanilla prescan** (both branches): O(n) already-in-key-order
  test over the extracted ORD_KA/ORD_KB — a fresh index skips the sort
  outright (ordSkip), a stale one pays it (ordWork). The ratio rides the
  hooks@ line (` ord=<skip>/<work>`): if skip dominates, the residual tax
  was TimSort on sorted input (dead); if work dominates, the stale-stamp
  window is the lever (the Rust-side resync / the stamp must move).
- **ORDER-MISMATCH first-diff prints** (both the typed and the untyped
  check): the first differing POSITION now names both entities' full order
  keys — `FD=<i> van=(id sec seq) fast=(id sec seq)` — the stale-stamp side
  identifies itself in one read. ORDER-MISMATCH joined the watchdog
  whitelist, so the prints stream into the job log live.

Diagnostic dispatch: profile mobs-25k with -Dcentities.orderCheck=true —
the mirror-per-query cost is accepted for one run; the FD prints + the
ord= ledger answer the 4.18% question in a single pass.

## L97.5 (session 15:40Z): THE t=157 ROOT CAUSE — the guard preempts vanilla's aware branch

The dispatch (mob_ai=1, trace_id=107, PH 152-162) live log gave the decisive
ledger. The ON leg at t=157: `GL t=157 br=121/113/0 gr=236/13 ... 8=484/484/0/484`
— the native even walk = 121 mobs × 4 NearestTarget rolls = 484, vanilla =
119 × 4 = 476, the delta = THE +8 roll shift. The PTRACE (mob 107): vanilla
strolls at t=157, the engine stands with `goalSelector=[MOVE=...Random...]`
— the L97.3 lock-map writes are LIVE (the engine-side map is populated for
the first time; the A/B selector-state comparison is apples-to-apples now).

The mechanism: the EarlyReturnGuard is woven at serverAiStep's METHOD TOP
(bc 0) — vanilla's aware branch (javap bc 10-48: !aware → goalFloat +
jumpControl.tick + RETURN, NO selectors, NO cadence, NO RNG) runs AFTER the
guard. A mob the vanilla body would serve with the aware branch (zero
selector walks) is served by the ENGINE with the full native AI walk —
+2 mobs in the even set, +8 rolls, the stream shifts, mob 107's stroll
roll reads a shifted value → RLA starts instead. The `gr - br` delta at
t=157 (236 - 234 = 2) is exactly the 2-mob set.

FIX: `if (!mob.aware) return false;` at the guard top — the vanilla body
IS the semantics for unaware mobs (the aware branch draws nothing, so the
decline is byte-exact). The gr census now counts post-aware entries
(gr - br = the aiClassify refusals). The PTRACE carries `aw=` in both legs
— the next dispatch either confirms the unaware set or kills the
hypothesis in one read. Either way the guard no longer preempts a branch
vanilla never runs.

Also landed this session: the floor + ACT lock-map writes (L97.3 — the
push parity PASSed on them) and the order ledger (L97.4 — the prescan
skips the sorted-input sort; the orderCheck profile names the stale-stamp
side if the TimSort tax persists).

## L97.5 addendum (session 16:10Z): the unaware hypothesis is DEAD — the +2 is NOT aware

The dispatch-2 GL comparison killed it in one read: `GL t=157 br=121/113/0
gr=236/13` — IDENTICAL to dispatch-1 (pre-L97.5). The aware gate declined
NOTHING: all 236 fill-phase serverAiStep entries are aware mobs (mob 107's
PTRACE: `aw=1` both legs). The L97.5 guard gate stays shipped as a
correctness hardening (the guard no longer preempts a branch vanilla never
runs — a real hazard for any future unaware mob), but it is NOT the t=157
fix.

Tick-mapping correction (mined from the CE-INIT + the GL windows): the GL
`GL t=` is `CUR_TICK/3` in gameTime units — gameTime ≈ pluginT + 314 (the
pre-plugin world boot ticks). The GL t=157 row IS the fill-phase t=157 —
the +2 walk-set delta stands: the engine's native even walk = 121 mobs ×
4 NearestTarget rolls = 484; vanilla's = 476 (the L97.1 local SRF count).
The phase formula is identical on both legs (the same tc, all aware), so
the remaining suspects are DOWNSTREAM of the phase gate:

(a) the vanilla even branch's canUse is gated by
goalCanBeReplacedForAllFlags — the TARGET lock map — vanilla's own walk
maintains it; the engine's native walk consults ITS lock table. If the two
lock tables disagree about a TARGET-flag holder for 2 mobs (a stale
holder on either side), the canUse counts diverge. The L97.3 map writes
make the VANILLA-side map consistent on the ON leg — the PTRACE
selector-state dumps in the dispatch-2 artifact now show both legs' full
lockedFlags per tick for mob 107 (t=149-956) — the first
apples-to-apples lock-map evidence.
(b) the L97.1 local SRF count (119) itself — the GLV-vs-engine
apples-to-apples caveat; a re-count with the 4096-frame streams of the
dispatch-2 artifact names the exact 2 mobs' draw frames.

NEXT: the dispatch-2 artifact (PD 152-162 + the PTRACE selector states +
the 4096 SRF streams) is the data pack; the local A/B harness (rustup +
maven, the L97.1 loop) is the iteration tool. The even-set question —
121 vs 119 on an identical phase formula — has two concrete, checkable
suspects and the evidence to settle both.
