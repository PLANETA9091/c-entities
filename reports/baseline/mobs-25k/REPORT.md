# Baseline run: `mobs-25k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25058 -> 25058) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 546.458 / 602.767 / 653.333 / 682.106 |
| MSPT mean, ms | 557.123 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 1245.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 38.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.97% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 0.74% | `java.util.HashMap.getNode(Object) line: 586` |
| 11 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 0.67% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 13 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

