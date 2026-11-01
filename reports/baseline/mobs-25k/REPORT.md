# Baseline run: `mobs-25k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25064 -> 25064) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 541.097 / 596.295 / 661.652 / 688.869 |
| MSPT mean, ms | 549.894 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 1806.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 37.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.67% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 2.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 2.11% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 0.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.80% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 0.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 0.77% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 14 | 0.71% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 15 | 0.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |

