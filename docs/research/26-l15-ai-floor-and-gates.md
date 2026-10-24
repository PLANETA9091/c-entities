# 26. L15 — the AI floor and the no-op write gates

Date: 2026-10-25. Targets: the flat per-entity tail the push kernel (doc
23/24), the tick-list walk (L13) and the tracker gate (L14) left behind.
The 26.3 `mobs-25k-mod` profile (run 40121311720, 19049 samples) had no
dominating leaf anymore — the top frame (`AABB.intersects`, 4.71%) is the
push resolve itself — but the scheduler bookkeeping and two pure no-op
writes added up to a real slice:

| share (module leg, self) | method |
|---:|---|
| 2.05% | `CeHooks.worldId` — OUR OWN tax (the 1-slot LRU missed on every world alternation and allocated an `Object[2]` per hit; worlds with id >= 4 fell through to `ConcurrentHashMap.putIfAbsent` on EVERY call) |
| 1.83% | `Entity.setTicksFrozen(int)` — the whole inlined `SynchedEntityData.set` chain, paid per entity per tick to write the SAME value |
| 1.33% | `GoalSelector.tick` |
| 1.26% | `ConversionTracker.setConverting` (via `ConversionTracker.tick`) — same equal-value write shape |
| 1.12% | `EnumMap.get` — the `lockedFlags.getOrDefault` chains |
| 0.72% | `GoalSelector.tickRunningGoals` |
| 0.70% | `AbstractCollection.containsAll` (the removeIf path) |
| 0.37% | `WrappedGoal.getFlags` |

## 1. worldId — the 16-slot identity table

The LRU pair assumed "same-level bursts". The 26.3 benchmarks keep three
worlds live (overworld + nether + end exist by default), so every
alternation re-allocated and worlds >= 4 never hit the table. L15 replaces
both structures with one flat identity scan over 16 slots: ~4 reference
compares typical, ZERO allocation on any hit, the map only on the first
sight of a level. Slots are claimed once (world ids are never reassigned)
and never rebound — a reader racing a publisher sees a null slot and takes
the map path, never a wrong id.

## 2. rowQuery — the home re-check without divisions

The per-candidate home re-check (`cellOfBox(eb) != packed`) cost 1.26%
caller CPU across 19M push queries: two floor divisions per candidate.
The row already carries its exact cell origin (`MQ_CELL_X0`), so the test
becomes two interval compares against `[x0, x0+16)`. Equivalence: with
c = center, floor(c/16) == cell (as doubles) iff c ∈ [cell*16, cell*16+16),
and `(min+max)*0.5` is exact (division by 2 is mantissa-preserving), so the
interval form is the SAME test without the division. `MQ_CELL_Z0` joins the
row-collect for the z axis.

## 3. The AI floor — GoalSelector microkernel

Paper 26.3's GoalSelector (decompiled, shipped build
f649771a2190214306fb0cfba22c7b2cbac5df6308b1158309593603ed07d488) keeps
goals in an `ObjectLinkedOpenHashSet` (insertion order), locks flags in an
`EnumMap<Flag, WrappedGoal>`, and runs per even tick: cleanup pass
(`canContinueToUse` per running goal), `lockedFlags.entrySet().removeIf`,
update pass (per non-running candidate: flag checks + `canBeReplacedForAllFlags`
+ `canUse`), then `tickRunningGoals(true)`. Odd ticks run only
`tickRunningGoals(false)` (vanilla 1.21.2+ id-parity splitting —
`(tickCount + id) % 2`).

The floor replaces both method bodies (`replace_body`) with
`CeHooks.goalSelectorTick / goalSelectorTickRunning`:

* the flag math collapses to long bitmasks (`OptimizedSmallEnumSet` IS a
  long — `hasCommonElements` is one AND);
* the `EnumMap` is replaced by a per-thread `WrappedGoal[FLAG_N]` lock
  table REBUILT each cleanup from the running goals' current flags. This
  is exact because every `setFlags` call in the shipped goal classes sits
  in a constructor (grep over the full decompiled goal set — zero
  behavioral callers), so current flags == flags-at-start for vanilla
  goals. `lockedFlags` has no readers outside `GoalSelector` (decompile-
  verified) — the mirror reproduces it through a RefField;
* `tickRunningGoals(false)` walks a RUNNING-ONLY shadow (insertion order
  preserved, rebuilt by every floor tick pass, live `isRunning` re-check +
  lazy pruning) — the odd-tick pass iterates ~2-4 running goals instead of
  the full ~12-15 goal set;
* the call sequence is the vanilla sequence: same live-set iteration, same
  conditions, same short-circuit order — `canUse`/`canContinueToUse` run
  iff vanilla runs them; profiler sections (`goalCleanup`/`goalUpdate`/
  `goalTick`) are kept;
* the per-selector shadow rides a weak-keys identity map (`MapMaker`) —
  the alternative (adding a field to a loaded class) is impossible:
  retransform forbids schema changes.

Kill-switch integrity: the replaced bodies carry a byte-exact vanilla
mirror (`gsMirrorTick`), run while the floor is disarmed; `gsMirrorTicks`
moving is the verifier's PROOF that the weave is live BEFORE
`setAiFloorOn(true)` flips. A lock-table collision (two running goals on
one flag — impossible under vanilla semantics, possible for a plugin goal
mutating flags while running) or any in-floor throw disarms the floor
globally; exceptions propagate exactly like vanilla.

Documented divergence surfaces (both outside vanilla behavior):
a plugin goal that calls `setFlags` while running (the mirror assumption
above), and a plugin that calls `WrappedGoal.start()` directly between
ticks (the shadow lags one parity tick; the next full pass self-heals).

## 4. The no-op write gates

`Entity.setTicksFrozen(int)` is EXACTLY
`entityData.set(DATA_TICKS_FROZEN, Integer.valueOf(i))` (javap) and
`SynchedEntityData.set` already skips equal values — the profile cost is
the inlined chain itself. The gate compares the live data value first
(`getTicksFrozen()`), skipping the whole body on equality; unequal values
run the unchanged write. `ConversionTracker.setConverting` is the same
shape (`getEntityData().set(conversionId, false)` for every live,
non-afflicted, non-noAi mob, every tick — cached Booleans, no boxing).
Both bodies are replaced with bridges that bump liveness counters on EVERY
entry (`frozenCalls`/`convCalls` — the verifier proves the weave before
arming) and keep the exact vanilla write otherwise.

## 5. Failure policy

All three L15 weaves are OPTIONAL: a refusal logs + declines
(`*WOVEN` stays false, no `PATCHES_FAIL`) and the rest of the module arms
normally — a failed AI floor must never disable the push kernel. The e2e
asserts the full chain (precomputed + patched + ARMED markers and
`gs=`/`froz=`/`conv=` traffic); the first e2e zombie now spawns WITHOUT
`no_ai` so the goal pipeline actually runs.

## 6. L15b/L15c — the tracker path (added after the first green e2e)

The items-100k-mod profile put OUR OWN gate at the top of the leg:

* `trackerDispatch` self ≈ 16.8% across 3 lines + `MethodHandle*Accessor`
  3.3% — the L14 guard itself. Two fixes: the pure-read REORDER (the
  cheapest sufficient no-send discriminators first — `isDirty` rejects a
  resting item before any RefField/interface work; every read in the guard
  is pure, so the read order is free — the two unconditional effects stay
  at the tail in vanilla order), and the per-call counters went from
  volatile to PLAIN (a volatile add per entity per tick was 1-2ms/tick on
  items-100k; the verifier/e2e readers tolerate eventual visibility).
* `ChunkMap$TrackedEntity.moonrise$clearPlayers` 14.00% — the whole
  `moonrise$tick` early path (100k trackers × null nearbyPlayers chunk →
  clearPlayers = two idempotent writes + `seenBy.isEmpty()` early return,
  javap-verified). L15c gates `moonrise$tick` with a RUST early-return
  guard (patch.rs `insert_early_return_guard`: loads (this, chunk),
  invokestatic check, ifne over the method's own return opcode). The
  branch target is the original body START — the implicit entry frame —
  so NO new StackMapTable entry is needed; the existing first-frame +delta
  shift suffices. The check (`trackerTickNoOp`) returns true ONLY for
  chunk==null && seenBy empty && lastChunkUpdate==-1 &&
  lastTrackedChunk==null — the exact pre-cleared state where vanilla's
  writes are provable no-ops. Any other shape (real players, stale
  bookkeeping) routes vanilla wholesale.

The guard patcher required no SDK change: the SDK's op-4 passes only the
first ref arg, but the rust patcher builds arbitrary prefixes — the same
splice/shift machinery the probe builder uses, refactored into
`splice_prefix` shared by both.
