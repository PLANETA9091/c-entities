# Baseline run: `mobs-10k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 10000 (items 0 / stands 0 / zombies 10000) (merge shrink: 10026 -> 10026) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 131.228 / 143.498 / 180.230 / 212.497 |
| MSPT mean, ms | 132.953 |
| TPS estimate (1000/p50) | 7.6 |
| heap used / max, MB | 1491.9 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 3 | 1.84% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 1.59% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 5 | 1.53% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 1.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 8 | 1.13% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 9 | 1.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.01% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 11 | 1.00% | `java.lang.ThreadLocal.get() line: 172` |
| 12 | 0.98% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 0.94% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 14 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |

