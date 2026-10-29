# Baseline run: `mobs-25k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25054 -> 25054) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 404.554 / 451.440 / 492.962 / 543.717 |
| MSPT mean, ms | 412.108 |
| TPS estimate (1000/p50) | 2.5 |
| heap used / max, MB | 1757.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 31.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.32% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.08% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 1.06% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 0.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.84% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

