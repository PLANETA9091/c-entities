# Baseline run: `mobs-25k`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25065 -> 25065) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 525.143 / 628.721 / 725.906 / 773.256 |
| MSPT mean, ms | 549.128 |
| TPS estimate (1000/p50) | 1.9 |
| heap used / max, MB | 921.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 36.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.62% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.00% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 0.85% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.69% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 11 | 0.69% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 0.69% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.63% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 0.62% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |

