# Baseline run: `mobs-25k`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25045 -> 25045) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 365.889 / 405.638 / 439.427 / 481.868 |
| MSPT mean, ms | 371.002 |
| TPS estimate (1000/p50) | 2.7 |
| heap used / max, MB | 1772.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 25.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.33% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 5 | 2.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.92% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 0.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 11 | 0.88% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.75% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.74% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.73% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |

