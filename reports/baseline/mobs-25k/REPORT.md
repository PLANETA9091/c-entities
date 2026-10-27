# Baseline run: `mobs-25k`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25052 -> 25052) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 372.869 / 417.797 / 454.965 / 477.779 |
| MSPT mean, ms | 380.540 |
| TPS estimate (1000/p50) | 2.7 |
| heap used / max, MB | 1291.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 23.96% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 2.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.25% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.11% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 11 | 1.04% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.86% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

