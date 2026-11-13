# Baseline run: `spread-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 84784 -> 84652) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 180.167 / 254.211 / 387.157 / 513.723 |
| MSPT mean, ms | 198.614 |
| TPS estimate (1000/p50) | 5.6 |
| heap used / max, MB | 2490.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.97% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 5.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3460` |
| 3 | 3.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3404` |
| 4 | 3.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7346` |
| 5 | 3.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11977` |
| 6 | 2.47% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 7 | 2.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3417` |
| 8 | 2.03% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3434` |
| 10 | 1.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3421` |
| 11 | 1.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3442` |
| 12 | 1.66% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3406` |
| 14 | 1.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3424` |
| 15 | 1.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

