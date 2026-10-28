# Baseline run: `mobs-25k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25051 -> 25051) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 334.432 / 380.779 / 412.021 / 469.345 |
| MSPT mean, ms | 341.296 |
| TPS estimate (1000/p50) | 3.0 |
| heap used / max, MB | 1553.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 26.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.07% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.51% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 11 | 1.18% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 12 | 1.16% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 1.12% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 15 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

