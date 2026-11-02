# Baseline run: `mobs-25k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25076 -> 25076) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 518.698 / 571.993 / 627.043 / 711.507 |
| MSPT mean, ms | 526.546 |
| TPS estimate (1000/p50) | 1.9 |
| heap used / max, MB | 1614.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 34.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.07% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.09% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 0.88% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 10 | 0.82% | `java.util.HashMap.getNode(Object) line: 586` |
| 11 | 0.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.79% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 13 | 0.78% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 14 | 0.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 0.63% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |

