# Baseline run: `spread-100k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97353 -> 97353) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 150.562 / 222.857 / 286.951 / 355.697 |
| MSPT mean, ms | 163.337 |
| TPS estimate (1000/p50) | 6.6 |
| heap used / max, MB | 2016.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4024` |
| 3 | 2.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.89% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 5 | 2.79% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3860` |
| 7 | 2.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.47% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5646` |
| 11 | 1.20% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 12 | 1.03% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3888` |
| 13 | 1.02% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5550` |
| 14 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 15 | 0.86% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

