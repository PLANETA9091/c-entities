# Baseline run: `mobs-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25056 -> 25052) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 699.130 / 739.775 / 785.886 / 882.052 |
| MSPT mean, ms | 700.030 |
| TPS estimate (1000/p50) | 1.4 |
| heap used / max, MB | 1535.6 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 32.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.90% | `java.util.HashMap.getNode(Object) line: 585` |
| 3 | 2.43% | `java.lang.ThreadLocal.get() line: 172` |
| 4 | 2.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 7 | 1.74% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 8 | 0.86% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 10 | 0.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 11 | 0.70% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 12 | 0.68% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 13 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.60% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |

