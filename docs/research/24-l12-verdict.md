# 24. L12 verdict — the push kernel in numbers

Date: 2026-10-24. Runs: baseline 40092430312 (playerless, kernel live,
06df8d0) and ab-paired #14 40092929611 (WITH the bench bot, paired
same-runner). The doc-21 verdict table (ab-paired #12) is the pre-kernel
reference.

## 1. Baseline A/B (same-run pairs, kernel live)

| scenario | vanilla mean | module mean | ratio |
|---|---:|---:|---:|
| mobs-25k | 317.87 | **162.16** | **0.510 (-49.0%)** |
| mixed-150k | 451.57 | **202.34** | **0.448 (-55.2%)** |

Pre-kernel references (older runs, different runners — ratios are the
honest comparison): mobs-25k-mod 193.0/366.3 = 0.527; mixed-150k-mod
396.7/458.3 = 0.866. The kernel moved mixed by ~42 points of ratio and
kept mobs at its best while BOTH legs now keep every entity alive
(the pre-kernel mod leg silently drained 25k→19.9k mobs — the doc-21-era
"merge shrink" for mobs was actually mob deaths, now understood and gone).

Kernel telemetry (fresh baseline): push=15.3M checks/15.2M native
(mobs-25k), 11.0M/10.95M (mixed-150k) — the guard serves ~100% of pushes.

## 2. The hotspot collapse (mobs-25k-mod, JFR self-time top)

Pre-kernel: `AABB.intersects` 14.1+2.8%, `ChunkEntitySlices.getEntities`
~4.8%, GoalSelector 2.5% — the push query stack owned the profile.
Post-kernel: the top-15 is flat junk (ConversionTracker 2.3%,
InsideBlockEffectApplier 2.2%, setTicksFrozen 2.0%...) — the push share
fell out of the top entirely. The kernel removed the O(section) per-mob
scan AND the per-pair Java floor.

## 3. The with-bot paired verdict (ab-paired #14)

PENDING — filled in when run 40092929611 completes. The with-bot world is
the kernel's hardest case (the anchor pile: 7.5k-12.5k zombies in one
block, all active — the O(N²) pair loop the kernel accelerates ~10-18x).

## 4. Debugging ledger (what CI caught, for the next lever)

1. **check_desc ≠ Java descriptor** — the woven guard was generated with
   `(LLivingEntity;)Z` while the method takes `(LEntity;)Z` →
   NoSuchMethodError per entity tick → **Paper 26.3 silently DISCARDS
   entities that throw in their tick** → the whole world vanished
   (add==rm==25066, push=0). Contract: the guard receiver rides as the
   base type; the check_desc must equal the Java descriptor byte-for-byte.
2. **invokeExact needs the EXACT handle type** — `(Entity,int)void` handle
   vs a statically-typed LivingEntity receiver → WrongMethodTypeException
   → ACTIVE=false. Fix: `(Entity) le` at the call site.
3. **The e2e LIVE assert (pushNative>0) is now mandatory** — patch-applied
   logs prove nothing about linkage.
4. The arena slot collision now evicts the previous owner from the push
   index (the kernel can never see a half-migrated id).
5. The delta merge ledger (mirror = java + kernel_add) keeps queued
   probeDelta events from erasing pending kernel pushes.

## 5. What remains (the honest -90% path)

* items-100k/spread: the floor is the bot's own playerTouch (canHold
  ~5%), the tracker/sync stack, and the distributed dispatcher — no single
  lever; needs an L13 batch (tracker skip + merge-check batching).
* The zombie AI floor (GoalSelector/Brain/navigation) — the next frontier
  for mobs beyond -50%: a native idle-mob dispatch with a Brain-port is a
  separate project-sized effort.
* The kernel itself can go further (SoA force accumulation, per-cell
  batched numCollisions) if the with-bot verdict demands it.
