# Baseline run: `mobs-25k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25056 -> 25056) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 497.335 / 547.197 / 590.785 / 611.556 |
| MSPT mean, ms | 505.214 |
| TPS estimate (1000/p50) | 2.0 |
| heap used / max, MB | 931.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 35.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.01% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.14% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 7 | 1.13% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.90% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.79% | `java.util.HashMap.getNode(Object) line: 586` |
| 13 | 0.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.73% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.72% | `java.util.EnumMap.get(Object) line: 248` |

