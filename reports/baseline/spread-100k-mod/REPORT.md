# Baseline run: `spread-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88705 -> 88581) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 50.026 / 73.415 / 185.551 / 254.539 |
| MSPT mean, ms | 58.646 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 2250.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 6.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 3 | 5.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8357` |
| 4 | 4.63% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14070` |
| 5 | 4.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8086` |
| 7 | 2.87% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 2.27% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5771` |
| 10 | 1.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5775` |
| 12 | 1.48% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.30% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 14 | 1.21% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13417` |
| 15 | 1.19% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |

