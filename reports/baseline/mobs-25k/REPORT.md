# Baseline run: `mobs-25k`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25064 -> 25064) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 376.947 / 419.030 / 456.138 / 471.071 |
| MSPT mean, ms | 384.358 |
| TPS estimate (1000/p50) | 2.7 |
| heap used / max, MB | 1060.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 25.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.17% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 5 | 2.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 1.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.38% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.90% | `java.util.HashMap.getNode(Object) line: 586` |
| 11 | 0.86% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 12 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 14 | 0.70% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

