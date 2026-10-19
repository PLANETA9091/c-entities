# Baseline run: `mobs-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25056 -> 25051) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 523.464 / 575.423 / 627.261 / 699.966 |
| MSPT mean, ms | 530.179 |
| TPS estimate (1000/p50) | 1.9 |
| heap used / max, MB | 1771.4 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 22.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.55% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 7 | 1.10% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.08% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 9 | 1.08% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 12 | 0.69% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 13 | 0.69% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 14 | 0.68% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 15 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

