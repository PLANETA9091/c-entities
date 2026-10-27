# Baseline run: `mobs-25k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25057 -> 25057) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 327.873 / 377.013 / 423.876 / 432.113 |
| MSPT mean, ms | 336.301 |
| TPS estimate (1000/p50) | 3.0 |
| heap used / max, MB | 1629.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 30.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 5 | 2.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.36% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 8 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.97% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 0.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.80% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 13 | 0.79% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.68% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |

