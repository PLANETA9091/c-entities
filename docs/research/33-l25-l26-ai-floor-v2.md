# 33. L25/L26 — the AI floor v2: the flat goal scheduler, the direct-read data gates, the Sensing O(1) clear, the getSharedFlag read floor

Date: 2026-10-26. Status: SHIPPED (this commit). Basis: the a792e84 ab-paired
run (mobs-25k -38.5%, mixed-150k -32.1%, items-100k -77.4%, spread-100k -62.1%
— the SHAPE C numbers) and the fresh hotspots pulled from that run's
reports/baseline/*-mod profiles.

## Where the remaining time is

The profiles are flat: no 10%+ leaf. The AI-scheduler family in mobs-25k-mod
alone bills ~6% (goalSelectorTick 2.51 self, ObjectLinkedOpenHashSet.next
2.03, EnumMap.get 1.31, WrappedGoal.getFlags 0.89), Sensing.tick 2.23, the
flag/frozen/conversion shim family ~5.4, the attribute reads ~3.5. In
mixed-150k the SynchedEntityData family (DataItem.getValue 5.05 + getItem
2.85) is the largest single group and the tracker fast-exit reads another
~7. Every fix below is a "thousand cuts" cut, chosen so each one is exact by
construction and independently kill-switchable.

## L25a — the flat goal scheduler (GoalSelector v2)

The L15 microkernel walked `availableGoals` FOUR times per tick (cleanup,
lock-rebuild, update, tickRunning(force)). The v2 body walks it ONCE:

1. **Shape stamp.** The AIShadow records `allSize/first/last` — two direct
   field reads on fastutil's ObjectLinkedOpenHashSet (`first()`/`last()`,
   javap-verified) plus the size. Three compares prove the set identity the
   running[] snapshot was built from.
2. **Cleanup over the running shadow** (set-order preserved, `isRunning`
   re-checked per entry — an externally stopped goal skips exactly where
   vanilla's set walk would skip it).
3. **Lock table from the survivors** — the collide check (two running goals
   on one flag = plugin flag mutation) disarms to the mirror exactly like
   v1.
4. **Update = the only full set pass**, with a **survivor merge-walk**: a
   running set element must be the next expected survivor (both walk the
   same insertion order). Anything else — an externally started goal, direct
   set surgery through getAvailableGoals(), a same-size mid-set swap — falls
   to `fullPassTick` (the exact v1 4-pass body, which also re-records the
   shape).
5. **The vanilla tail** `tickRunningGoals(true)` runs over the running array
   (the set's running subset in the same order, each entry re-checked).

Exactness anchors: `removeGoal`/`removeAllGoals` STOP running goals before
removing them (vanilla), so a removed goal can never be re-started or
re-ticked from a stale shadow; `addGoal` changes the size; a direct-set
same-size mutation breaks the merge-walk alignment; `WrappedGoal.equals` is
identity-or-goal-equals but the walk uses `==` only. The goal-counter
evidence: the update pass is the only LinkedHashSet iteration left, so
`next()`'s 2.03% and most of `goalSelectorTick`'s 2.51% self are the target.

Also: `canReplaceAll` read `cand.getFlags()` twice per call (a virtual hop
onto the inner goal each time) — one read now.

## L25b — the direct-read data gates (frozen/converting/shared-flag)

`SynchedEntityData.set` already skips equal values (javap:
`ObjectUtils.notEqual(value, item.getValue())` — bc 11-20), so a same-value
gate is exact; what the L18/L15 gates paid was the BOXED GET CHAIN to learn
the current value — the same chain as the write they avoided. `getTicksFrozen`
IS that chain. The fix: `entityDataValue(entity, accessor)` — one typed
Unsafe walk (Entity.entityData → itemsById[id] → DataItem.value), every hop
instanceof-checked against the class its offset belongs to (the doc-32 rule;
`O_ACC_ID` reads the accessor record's `id` component). The gates collapse
from ~50-100ns to ~10ns, and the retracted-L19 `dataValueAt` approach is
rehabilitated with the typed checks the SE_FAST postmortem demanded.

Why this is safe where SE_FAST was not: SE_FAST fed a FOREIGN receiver (a
ChunkMap$TrackedEntity) into ServerEntity offsets and dereferenced the
garbage as an oop. Here every receiver is the class the offset was resolved
from (Entity/SynchedEntityData/DataItem/ConversionTracker — the last two
guaranteed by the weave itself), the unbox happens only after the value's
own instanceof, and a miss routes the vanilla chain.

## L26a — the Sensing O(1) clear

Vanilla `Sensing.tick()` is `seen.clear(); unseen.clear();` — and fastutil's
`IntOpenHashSet.clear()` is an O(capacity) `Arrays.fill` whenever non-empty
(decompiled; the empty early-out is why the cost survives on quiet legs:
the LookAtPlayer scans fill the sets every tick in a dense crowd — 2.23% of
mobs-25k-mod on those two lines). The floor swaps the two PRIVATE FINAL
fields for `GenIntSet` — an open-addressed int table with per-slot
generation stamps whose `clear()` is one integer increment:

- `contains/add` probe exactly like vanilla for the current generation;
  stale entries are invisible tombstones (probe-safe, rehashed away on
  growth);
- the vanilla `hasLineOfSight` body is NOT touched — it calls
  `contains/add` on whatever IntSet is in the field;
- the swap stores a REAL reference into a declared IntSet field via
  `Unsafe.putObject` — a proper oop store with the GC barrier; the SE_FAST
  bug class was a raw-offset READ of a foreign-class receiver, not a
  real-reference store;
- entity id 0 never occurs (ENTITY_COUNTER starts at 1) but is handled
  through an explicit slot anyway;
- the kill-switch mirror clears whatever IntSet sits in the fields —
  including a GenIntSet — so a mid-run disarm stays exact.

## L26b — the getSharedFlag read floor

`Entity.getSharedFlag(int)` (javap, 26.3) is the full boxed get chain for a
read made ~10-30x per mob per tick (isOnFire/isCrouching/isSprinting/
isGlowing/...). The floor replaces the body with the same DataItem read
directly (one typed walk, `FLAGR_ACC_ID` precomputed from the captured
DATA_SHARED_FLAGS_ID accessor) and `(1 << flag) & b != 0` — byte-exact,
including the no-bounds-check `(1<<flag)` overflow behavior for flag ≥ 32.
The mirror is the exact vanilla chain; a mirror failure disarms and
rethrows (never silently wrong).

## L26c — the tracker fast-exit direct reads

The L24a fast exit paid three megamorphic RefField reads per entity per tick
(the `Unsafe.getObject` 2.29% in mixed-150k). The reads now go through
single-class ServerEntity offsets — the receiver is the replaced sendChanges
body's own `this` (weave-guaranteed), which is exactly the typed-receiver
condition whose ABSENCE killed SE_FAST; a dedicated kill switch
(`centities.trackerDirect`) can cut this without losing the fast exit.

## Kill switches (each path independent)

`centities.aiFloor` (GS v2 — falls to fullPassTick on any shape doubt),
`centities.frozenGate`, `centities.convGate`, `centities.flagGate`,
`centities.sensing`, `centities.flagRead`, `centities.trackerDirect`.
Arming rides the existing verifier: a weave is armed only after its handle
proof AND its liveness counter moves (senseCalls/flagrCalls added to the
counter map).

## Expected effect

mobs-25k: -4..-7% (scheduler 2-2.5%, sensing 2.2%, gates 1.5-2%).
mixed-150k: -3..-5% (gates + tracker reads + sensing + scheduler share).
items-100k/spread-100k: smaller (the floors are mob-centric; the flag-read
floor covers items too). Watch the next ab-paired run.

## The bench verdict (mobs-25k-mod server log, baseline run 40340133235 @ a8ea95c)

The hooks@ counter line settles every open question (note the field order:
gs = floorTicks/floorRunning/starts/stops/mirrorTicks/collide):

- `gs=20576466/15475570/157274/149098/0/0` — the v2 floor served ALL 20.5M
  goal ticks; the mirror NEVER ran (gsMirrorTicks=0); zero collide disarms.
  The flat scheduler is the production path.
- `froz=15499933/15419343/0` and `conv=15470315/15389990/0` — both gates
  catch 99.5% of all calls; zero vanilla writes leaked.
- `flag=31024920/30860581/0` — the shared-flag gate catches 99.5%.
- `row=3377801/0/0/0/112/0/712/0/0/0/0/0/w350810046` — the row path served
  3.38M single-cell queries with 350.8M window skips; rowMulti=0 (multi-cell
  boxes never reach the row path — the next lever).
- `fast=3378625 mirror=12121308` — 78% of Level.getEntities calls (the big
  targeting scans) routed to the vanilla mirror by the per-bucket EMA — the
  mirror family (AABB.intersects 4.42% + ChunkEntitySlices + Entity.getId
  2.49%) is the largest remaining bucket in the leg.

Two reporting artifacts to keep in mind when reading hotspots.md:

1. The hotspot script's CeHooks frame names can mislead after an edit shift
   (the a8ea95c profile printed "gsMirrorTick line 4679" for what is the v2
   goalSelectorTick cleanup pass — the counters above are the ground truth).
2. The e2e assert family (grep on the server log) stays the exactness gate;
   the counters are the perf truth.

L28a adds the missing retransform after the Sensing precompute (the bench
loaded Sensing pre-arm; "precomputed" alone never went live — vanilla
Sensing.tick frames at 1.51% in the same profile).
