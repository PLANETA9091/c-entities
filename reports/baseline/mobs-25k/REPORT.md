# Baseline run: `mobs-25k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25047 -> 25047) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 518.540 / 574.728 / 624.272 / 660.576 |
| MSPT mean, ms | 527.472 |
| TPS estimate (1000/p50) | 1.9 |
| heap used / max, MB | 1350.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 33.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.01% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.04% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.82% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.04% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.84% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 12 | 0.80% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 13 | 0.78% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.78% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

