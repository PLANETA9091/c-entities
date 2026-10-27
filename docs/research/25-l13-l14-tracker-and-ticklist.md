# 25. L13+L14 — the tracker fast gate and the tick-list raw walk

Date: 2026-10-24. Targets: the flat per-entity vanilla WRAPPER cost that the
push kernel (doc 23/24) left behind. Playerless module legs now spend their
time in bookkeeping, not in the kernels we built:

| share (module leg) | items-100k | mixed-150k | mobs-25k |
|---|---:|---:|---:|
| `ServerEntity.sendChanges` (3 frames) | ~13% | ~2.6% | ~0.8% |
| `ChunkMap$TrackedEntity.moonrise$clearPlayers` | ~8% | ~3.2% | ~1.5% |
| `EntityTickList.forEach` (+`isRemoved`+guard frames) | ~21% | ~7% | ~2.6% |
| `SynchedEntityData.getValue` | — | ~4.4% | ~1.2% |

Everything below is javap-verified against the shipped paper-26.3 build
(sha256 f649771a2190214306fb0cfba22c7b2cbac5df6308b1158309593603ed07d488).

## 1. L14 — `ServerEntity.sendChanges` fast gate

The whole per-tick sendChanges cost for an entity that sends NOTHING is the
decision chain itself plus one `Vec3` allocation (`trackingPosition()`) plus
an interface call (`InterpolationTracker.updateTracking`). On the no-send
path the method has EXACTLY three observable effects (bytecode offsets from
the shipped build):

1. `entity.updateDataBeforeSync()` (bc 19-23; virtual — `LivingEntity` adds
   `updateDirtyEffects()`, so the guard CALLS it, never skips it);
2. `interpolationTracker.updateTracking(trackingPosition())` (bc 302-311) —
   the `NO_OP` implementation is an empty body (javap'd), so skipping the
   call AND the `Vec3` allocation for `NO_OP` entities is exact;
3. `tickCount++` (bc 1106-1113) — the unconditional tail.

The weave is the L12 early-return pattern: `sendChanges()V` prefixed with
`if (CeHooks.trackerDispatch(this)) return;`. The guard re-evaluates the
vanilla gate in the vanilla order and routes vanilla on ANY doubt:
`moonrise$isHardColliding` (teleportDelay bookkeeping), `syncPosition`
(interval reset side effect), `needsSync`/`syncVelocity`/`forceStateResync`,
passengers changed (vanilla writes `lastPassengers` + emits the packet
there), `trackedPlayers` non-empty (ItemFrame map tick + all emit paths),
`updateInterval.test(tickCount)` (PURE — both implementations javap'd:
`NEVER` returns false, `periodic` is `n % period == 0`),
`SynchedEntityData.isDirty` (a flag read).

Order note: the guard reads the pure gate fields BEFORE calling
`updateDataBeforeSync` (vanilla calls it first). The only divergence surface
is the entityData dirty-flag lifetime: `updateDirtyEffects` may mark the data
dirty; with `trackedPlayers` empty vanilla would clear it via
`sendDirtyEntityData` only inside the ItemFrame branch (bc 290) — which the
empty-players path never reaches — so vanilla ALSO leaves it dirty there.
Any non-empty `trackedPlayers` routes vanilla wholesale. The private
`ServerEntity` state (`entity`, `tickCount`, `lastPassengers`,
`trackedPlayers`, `interpolationTracker`, `updateInterval`,
`forceStateResync`) rides the existing `RefField` helper (VarHandle with a
reflection fallback), built ATOMICALLY at first use (the doc-24
half-published-handles lesson). The catch routes vanilla — a throw inside
sendChanges propagates into the tick loop and kills the server.

Class-load timing: `ServerEntity` is only referenced by `ChunkMap` once a
level instance exists, so it may load AFTER the arm. The precompute DEFERS
(no `PATCHES_FAIL`), the byte hook stashes the bytes whenever they appear,
and the patch verifier completes the weave outside any ClassFileLoadHook
callback (the ASM helper's loadClass self-deadlocks inside one).

## 2. L13 — `EntityTickList.forEach` raw walk

Vanilla `forEach` = `iterator()` + `hasNext`/`next` + `accept` +
`finishedIterating` in try/finally. The moonrise `BaseIterator` semantics,
reproduced exactly by the PUBLIC raw API of the same set
(`IteratorSafeOrderedReferenceSet`):

* visit order = array order (insertion), nulls skipped — `remove()` nulls
  the slot immediately (bc 51-57), even mid-iteration;
* entities ADDED during the walk are NOT visited: `iterator()` captures
  `maxIndex = listSize` at creation and caps at `min(maxIndex, listSize)`;
  the mirror snapshots `getListSize()` once and caps the same way;
* `iteratorCount++/--` and the defrag-on-finish bookkeeping ride on
  `createRawIterator()`/`finishRawIterator()` (same counters, same
  `iteratorCount==0` defrag condition);
* a consumer exception still passes `finishRawIterator` (try/finally)
  before propagating — vanilla's exception table does the same.

The mirror is exact with the module ARMED or DISARMED (it only changes the
iteration strategy), so it deliberately does not read `ACTIVE`: both A/B
legs walk the same way and the gate stays honest. On a stale `RefField`
(class reload) the mirror rebuilds the handle once and retries; a second
failure rethrows — silently skipping the entity tick phase is the one
outcome this weave must never produce.

## 3. Telemetry and gates

* `stats()` gained `tracker=checks/fast/vanilla/err` and
  `ticklist=checks/err`.
* e2e asserts: both weaves applied; `tracker>=1` AND `tracker fast>0` (a
  playerless soak must serve provable no-ops); `ticklist>=1`.
* Kill switches: `-Dcentities.trackerFast=false` (guard inert, weave
  applied), `-Dcentities.tickListRaw=false` (weave refused at precompute;
  never counted as a failure so the module still arms).

## 4. What this buys (expected)

The tracker gate attacks the largest coherent wrapper share on every
scenario AND the with-bot case (entities far from the bot keep
`trackedPlayers` empty): items ~13%, mixed ~3%, mobs ~1%, spread ~2-3% of
module-leg time, plus the per-entity `Vec3` allocation churn. The tick-list
walk attacks the iteration+dispatch floor: items ~5-8% net, mixed ~3-5%,
mobs/spread ~2%. Honest expectation: +8-15 points of ratio on items-100k
(currently 0.544 playerless), +5-8 on mixed-150k (0.608), smaller on
mobs/spread. The with-bot gate is the bigger question — if the #14-era
regression was fix-3's disarm bug (fixed) plus tracker churn, L14 should
move the with-bot ratios most.
