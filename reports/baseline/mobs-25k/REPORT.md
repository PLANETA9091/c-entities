# Baseline run: `mobs-25k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25058 -> 25058) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 547.489 / 605.657 / 670.752 / 730.255 |
| MSPT mean, ms | 551.766 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 959.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 36.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.33% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 0.97% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.79% | `java.util.HashMap.getNode(Object) line: 586` |
| 11 | 0.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 0.70% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 13 | 0.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.63% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |

