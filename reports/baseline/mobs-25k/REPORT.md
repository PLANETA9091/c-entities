# Baseline run: `mobs-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25055 -> 25052) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 575.255 / 627.898 / 682.374 / 727.107 |
| MSPT mean, ms | 579.636 |
| TPS estimate (1000/p50) | 1.7 |
| heap used / max, MB | 1131.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 32.69% | `java.util.HashMap.getNode(Object) line: 585` |
| 2 | 4.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.19% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 2.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.22% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 6 | 0.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 7 | 0.91% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 8 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 0.81% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 0.71% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 11 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 13 | 0.65% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 14 | 0.65% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.64% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |

