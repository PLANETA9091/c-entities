# 31. L24 verdict — the SATB crash root-cause, SE_FAST deleted, the tracker fast exit

Date: 2026-10-26. Status: SHIPPED (this commit). Evidence: hs_err_pid4417
(items-100k @ c2140cf) + hs_err_pid5536 (mixed-150k @ e64e66f), both pulled
from the CI artifacts while they were still in retention.

## The crash, decoded

Both dumps are the same fault:

- `SIGSEGV` in `G1SATBMarkQueueSet::filter(SATBMarkQueue&)+0xb7`
- chain: `enqueue_known_active(qset, oop)` <- `handle_zero_index` <-
  `g1_pre_barrier_slow` <- **c1 `CeHooks.trackerDispatch`** <- c1
  `ServerEntity.sendChanges` <- `ChunkMap.newTrackerTick`.
- registers (4417): the faulting instruction is `mov rdx,[rdx]` — filter
  dereferencing a queue entry; RDX = the entry = `0x7f41_7009d4d8` — a
  NATIVE-range value (the G1 heap is reserved at `0x0000_0007...`, zero-based
  compressed oops) — **the entry is not an oop at all**, it is garbage that
  was enqueued as one.

Reading the HotSpot path precisely: `handle_zero_index` runs when the
per-thread SATB buffer FILLS (the store that happens to fill it is
trackerDispatch's — it executes ~100k pre-barriers/tick on the 100k legs);
`filter` then walks the WHOLE buffer and dereferences every entry. So the
crash site is where the buffer filled, **not necessarily where the garbage
came from** — any earlier barrier-less-corrupting write on the server thread
could have left the entry.

## The exoneration and the verdict

Run correlation over the 26.3 era:

| path state | runs | SATB SIGSEGV |
|---|---|---:|
| pre-L20b (no SE_FAST) | many, all green | 0 |
| L20b+L22 (SE_FAST default-on) | 6 | 4 |
| post-7c88ee2 (SE_FAST retracted) | 2, green | 0 |

And the e64e66f mixed-150k crash happened with the block-fx gate ALREADY
mob-scoped — **the block-fx item path is exonerated** (it was the first
suspect because the c2140cf items leg died with it on, but both were on in
every L22-era run; the only factor present in all four crashes and absent in
every green run is SE_FAST).

The module's own native surface was re-audited against the wild-write
hypothesis (the only way garbage gets into a native-range queue entry):
one raw pointer store total (`n_query2` → registered direct buffer), kept
alive by a `NewGlobalRef`, zeroed before delete — clean; all array ops are
bounds-checked JNI `Get/Set*ArrayRegion`; no cached jobjects across calls;
`RefField` offsets are resolved per-field by `getDeclaredField` +
`objectFieldOffset` (no name-caching); zero `Unsafe.putObject` call sites
(`putRefAt`/`setObj` have no callers). The mechanism stays formally
unresolved — which is exactly why the path is not coming back.

## Shipped

1. **SE_FAST deleted** from `CeHooks.trackerDispatch` (the VarHandle path is
   the only path again). The `CE_TRACKER_FAST` bisect switch is gone.
2. **The block-fx gate re-scoped to ALL entities** (its own kill switch:
   `CE_BLOCK_FX_ITEMS=0`). The gate was proven fast on mobs-25k (-23ms) at
   c2140cf.
3. **L24a — the tracker fast exit.** javap of 26.3 `ServerEntity.sendChanges`
   (doc 31 analysis): with `trackedPlayers` EMPTY the body cannot send —
   every emit rides `sendToTrackingPlayers(AndSelf)` over an empty set. The
   remaining effects split into:
   - reproduced in the fast exit: `updateDataBeforeSync` (bc 19-23),
     `tickCount++` (bc 1106-1113);
   - LEFT SET/STALE exactly like a vanilla empty-tracking tick: the
     syncPosition / needsSync / forceStateResync / isDirty latches and the
     lastPassengers compare — every one of them bypasses the interval gate
     (bc 347-390), so the first sendChanges with a recipient consumes the
     latch and emits exactly what vanilla would have;
   - ROUTED VANILLA (kept conditions): non-NO_OP interpolation trackers
     (updateTracking advances real state every tick), hard-colliding
     entities (teleportDelay=9999 written every tick, bc 0-16), and
     syncVelocity entities (the tail's velocity branch consumes the flag and
     fires the PlayerVelocityEvent path for ServerPlayers, bc 1116-1246).

   The fast exit reads `trackedPlayers` FIRST: untracked entities (the
   playerless bench legs, and every far-from-player entity on real servers)
   now cost one set-read + one identity compare + two cheap predicate reads
   instead of seven object reads + List.equals + interval.test + the dirty
   volatile chain.

## Expected effect

items-100k: the tracker gate family was ~25% of the 18.8ms profile
(trackerDispatch 13.9% + Unsafe.getObject 12.4% (shared) + trackerTickNoOp
5.1% + getTrackedEntity 3.7%); the fast exit removes most of the per-entity
work for the whole untracked population, and the block-fx item path adds
back its ~2-4ms win. Watch the next ab-paired run.

## Not in this milestone

- The batch-native tracker pass (one JNI crossing per world) — the reads
  must happen Java-side regardless; the fast exit already collapsed them.
- The moving-mob travel/collide port (doc 28) — the next Rust port target.
- The goal-engine pre-resolved-predicate redesign (doc 30 item 4).
