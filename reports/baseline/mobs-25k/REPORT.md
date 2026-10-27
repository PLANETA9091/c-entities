# Baseline run: `mobs-25k`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25050 -> 25050) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 381.168 / 430.477 / 472.145 / 490.850 |
| MSPT mean, ms | 388.884 |
| TPS estimate (1000/p50) | 2.6 |
| heap used / max, MB | 1356.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 21.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 5.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.51% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 2.03% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.28% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.23% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 1.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 0.90% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 13 | 0.85% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.82% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.81% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |

