# Baseline run: `mobs-25k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25048 -> 25048) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 249.237 / 284.696 / 317.919 / 346.289 |
| MSPT mean, ms | 255.350 |
| TPS estimate (1000/p50) | 4.0 |
| heap used / max, MB | 1294.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 17.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.22% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 5.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 2.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.99% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.92% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 9 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.37% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 11 | 1.36% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.34% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 13 | 1.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.26% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 1.13% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |

