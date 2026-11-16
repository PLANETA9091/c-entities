# Baseline run: `mixed-150k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 112185 -> 112149) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 96.253 / 160.584 / 315.375 / 401.030 |
| MSPT mean, ms | 113.115 |
| TPS estimate (1000/p50) | 10.4 |
| heap used / max, MB | 2119.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8945` |
| 2 | 8.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 3 | 8.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8102` |
| 4 | 5.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 4.74% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13865` |
| 6 | 3.42% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.16% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.81% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.54% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13212` |
| 10 | 1.45% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 11 | 1.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5566` |
| 12 | 1.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13902` |
| 13 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7955` |
| 14 | 1.27% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8955` |

