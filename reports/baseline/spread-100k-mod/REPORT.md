# Baseline run: `spread-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97422 -> 97422) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 178.672 / 283.340 / 393.372 / 510.087 |
| MSPT mean, ms | 199.362 |
| TPS estimate (1000/p50) | 5.6 |
| heap used / max, MB | 1987.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3188` |
| 2 | 3.46% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.30% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1484` |
| 4 | 3.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.97% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 3844` |
| 6 | 2.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 2.53% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3053` |
| 8 | 1.88% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.86% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.50% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3063` |
| 12 | 1.15% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 3818` |
| 13 | 1.13% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 14 | 1.12% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 15 | 1.04% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

