# Baseline run: `mobs-25k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25053 -> 25053) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 534.457 / 587.849 / 631.477 / 648.163 |
| MSPT mean, ms | 542.111 |
| TPS estimate (1000/p50) | 1.9 |
| heap used / max, MB | 1197.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 36.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.15% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.02% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 0.98% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.89% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 0.85% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 11 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.76% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 13 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.75% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 0.69% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |

