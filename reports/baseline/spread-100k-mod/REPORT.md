# Baseline run: `spread-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97406 -> 97406) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 337.878 / 523.910 / 674.319 / 927.654 |
| MSPT mean, ms | 369.796 |
| TPS estimate (1000/p50) | 3.0 |
| heap used / max, MB | 2023.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 2.66% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10380` |
| 4 | 2.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 5 | 1.90% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 1.81% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9778` |
| 7 | 1.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 9 | 1.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.16% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.12% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 1.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |

