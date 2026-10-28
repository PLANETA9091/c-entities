# Baseline run: `mobs-25k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25056 -> 25056) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 531.780 / 590.544 / 650.524 / 702.845 |
| MSPT mean, ms | 538.561 |
| TPS estimate (1000/p50) | 1.9 |
| heap used / max, MB | 1169.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 35.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.12% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.10% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 0.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 10 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.88% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 0.86% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 13 | 0.82% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 14 | 0.81% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

