# Baseline run: `mobs-10k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 10000 (items 0 / stands 0 / zombies 10000) (merge shrink: 10023 -> 10021) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 135.030 / 148.353 / 187.039 / 200.881 |
| MSPT mean, ms | 137.109 |
| TPS estimate (1000/p50) | 7.4 |
| heap used / max, MB | 1157.4 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.04% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getAtIndexVolatile(ConcurrentLong2ReferenceChainedHashTable$TableEntry[], int) line: 166` |
| 3 | 4.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.98% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 5 | 1.91% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 1.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 9 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 10 | 1.09% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.08% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 12 | 1.07% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 1.05% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 14 | 0.96% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 15 | 0.84% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |

