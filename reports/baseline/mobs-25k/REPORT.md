# Baseline run: `mobs-25k`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25070 -> 25070) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 309.353 / 350.081 / 390.800 / 411.869 |
| MSPT mean, ms | 317.489 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 1859.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 20.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 5.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.89% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.66% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.57% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.21% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.06% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 13 | 1.04% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 14 | 0.98% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.95% | `java.util.HashMap.getNode(Object) line: 586` |

