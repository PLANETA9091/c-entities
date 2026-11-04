# Baseline run: `mobs-25k`

Server: `26.3-239-13565bc (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25067 -> 25067) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 550.594 / 617.522 / 664.828 / 719.330 |
| MSPT mean, ms | 562.633 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 996.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 36.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.03% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 0.98% | `java.util.HashMap.getNode(Object) line: 586` |
| 8 | 0.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.78% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 11 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.72% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.71% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 14 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |

