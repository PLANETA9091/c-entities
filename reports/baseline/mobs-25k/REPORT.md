# Baseline run: `mobs-25k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25054 -> 25054) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 393.931 / 442.835 / 488.684 / 505.848 |
| MSPT mean, ms | 402.979 |
| TPS estimate (1000/p50) | 2.5 |
| heap used / max, MB | 1251.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 31.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.54% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.66% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.31% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.08% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 0.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 0.83% | `java.util.HashMap.getNode(Object) line: 586` |

