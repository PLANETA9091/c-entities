# Baseline run: `mixed-150k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100565 -> 100565) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 238.780 / 287.089 / 335.413 / 363.397 |
| MSPT mean, ms | 242.964 |
| TPS estimate (1000/p50) | 4.2 |
| heap used / max, MB | 1988.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.80% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7092` |
| 3 | 2.66% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7281` |
| 4 | 2.39% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7145` |
| 7 | 1.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1414` |
| 8 | 1.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 9 | 1.62% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.42% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4060` |
| 11 | 1.22% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1433` |
| 14 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.04% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

