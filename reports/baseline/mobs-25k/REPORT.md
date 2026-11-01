# Baseline run: `mobs-25k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25063 -> 25063) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 560.021 / 624.642 / 673.119 / 731.527 |
| MSPT mean, ms | 569.759 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 1254.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 37.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.91% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 0.79% | `java.util.HashMap.getNode(Object) line: 586` |
| 10 | 0.79% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.78% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 12 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.71% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 14 | 0.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.60% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |

