# Baseline run: `spread-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97433 -> 97433) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 340.535 / 457.493 / 577.419 / 783.639 |
| MSPT mean, ms | 355.420 |
| TPS estimate (1000/p50) | 2.9 |
| heap used / max, MB | 2239.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.79% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 2 | 3.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 3 | 3.00% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10643` |
| 4 | 2.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3136` |
| 5 | 2.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 7 | 2.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6736` |
| 8 | 2.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.60% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3081` |
| 11 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10041` |
| 14 | 1.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3110` |
| 15 | 1.05% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |

