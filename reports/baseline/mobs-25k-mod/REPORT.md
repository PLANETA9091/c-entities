# Baseline run: `mobs-25k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 21453 -> 20245) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 347.300 / 409.293 / 473.006 / 538.185 |
| MSPT mean, ms | 358.269 |
| TPS estimate (1000/p50) | 2.9 |
| heap used / max, MB | 1202.8 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 17.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.55% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.97% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 5 | 1.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.53% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 7 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 8 | 0.98% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 9 | 0.93% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 0.93% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 0.91% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 12 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.86% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 14 | 0.85% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 15 | 0.84% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |

