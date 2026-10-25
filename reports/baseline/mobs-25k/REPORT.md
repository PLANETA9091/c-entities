# Baseline run: `mobs-25k`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25056 -> 25056) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 332.480 / 377.598 / 410.141 / 441.771 |
| MSPT mean, ms | 339.819 |
| TPS estimate (1000/p50) | 3.0 |
| heap used / max, MB | 1094.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 21.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 4.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.82% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 2.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.61% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.56% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 1.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.28% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.24% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 12 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.90% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |

