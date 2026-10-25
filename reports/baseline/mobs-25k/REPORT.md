# Baseline run: `mobs-25k`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25084 -> 25084) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 360.751 / 405.411 / 446.429 / 487.042 |
| MSPT mean, ms | 368.007 |
| TPS estimate (1000/p50) | 2.8 |
| heap used / max, MB | 1351.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 24.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 5.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 1.99% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.13% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.01% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 0.99% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 0.90% | `java.util.HashMap.getNode(Object) line: 586` |
| 11 | 0.88% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 12 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 0.82% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 0.78% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |

