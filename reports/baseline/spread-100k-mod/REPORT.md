# Baseline run: `spread-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97461 -> 97461) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 248.742 / 390.459 / 528.915 / 709.398 |
| MSPT mean, ms | 274.684 |
| TPS estimate (1000/p50) | 4.0 |
| heap used / max, MB | 2210.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.09% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.84% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10361` |
| 3 | 2.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 4 | 2.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 1.84% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9759` |
| 7 | 1.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.61% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 9 | 1.57% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.07% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.00% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 15 | 1.00% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

