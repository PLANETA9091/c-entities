# Baseline run: `mobs-25k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25075 -> 25075) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 352.087 / 395.614 / 439.486 / 469.333 |
| MSPT mean, ms | 360.602 |
| TPS estimate (1000/p50) | 2.8 |
| heap used / max, MB | 1875.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 27.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.87% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.07% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 1.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 11 | 1.25% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 12 | 1.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.96% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 15 | 0.91% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

