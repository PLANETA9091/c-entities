# Baseline run: `mobs-25k-native`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24137 -> 23298) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 322.042 / 387.809 / 464.404 / 559.912 |
| MSPT mean, ms | 328.989 |
| TPS estimate (1000/p50) | 3.1 |
| heap used / max, MB | 1616.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 25.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 2.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.05% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 6 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 7 | 1.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.38% | `java.util.HashMap.getNode(Object) line: 586` |
| 10 | 1.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 11 | 1.26% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.15% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 13 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 14 | 1.01% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.98% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |

