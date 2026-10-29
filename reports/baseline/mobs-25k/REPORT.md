# Baseline run: `mobs-25k`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25046 -> 25046) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 508.692 / 567.522 / 614.932 / 634.827 |
| MSPT mean, ms | 519.878 |
| TPS estimate (1000/p50) | 2.0 |
| heap used / max, MB | 1201.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 35.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.01% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 0.98% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 0.97% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.97% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.80% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 13 | 0.78% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.67% | `java.util.EnumMap.get(Object) line: 248` |

