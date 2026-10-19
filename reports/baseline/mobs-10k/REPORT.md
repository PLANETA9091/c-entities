# Baseline run: `mobs-10k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 10000 (items 0 / stands 0 / zombies 10000) (merge shrink: 10025 -> 10023) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 131.792 / 141.873 / 177.663 / 191.100 |
| MSPT mean, ms | 132.633 |
| TPS estimate (1000/p50) | 7.6 |
| heap used / max, MB | 1796.4 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.38% | `java.lang.ThreadLocal.get() line: 172` |
| 3 | 1.96% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 1.61% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 5 | 1.44% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.33% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 7 | 1.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.25% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 11 | 0.91% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 12 | 0.89% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 13 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 14 | 0.87% | `java.util.EnumMap.get(Object) line: 247` |
| 15 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

