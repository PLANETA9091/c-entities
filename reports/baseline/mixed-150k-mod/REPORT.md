# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109598 -> 109598) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 368.300 / 471.538 / 678.112 / 778.666 |
| MSPT mean, ms | 388.126 |
| TPS estimate (1000/p50) | 2.7 |
| heap used / max, MB | 1867.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.02% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7501` |
| 3 | 2.83% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7695` |
| 4 | 2.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4614` |
| 5 | 1.90% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7554` |
| 8 | 1.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4469` |
| 10 | 1.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 11 | 1.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 14 | 1.22% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

