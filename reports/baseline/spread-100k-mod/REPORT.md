# Baseline run: `spread-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88691 -> 88537) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 52.251 / 78.763 / 210.200 / 365.049 |
| MSPT mean, ms | 62.765 |
| TPS estimate (1000/p50) | 19.1 |
| heap used / max, MB | 2201.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 6.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7936` |
| 3 | 6.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8698` |
| 4 | 5.04% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13618` |
| 5 | 4.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.87% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.66% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5374` |
| 9 | 1.66% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.63% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 11 | 1.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5378` |
| 12 | 1.29% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.28% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 14 | 1.10% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7766` |

