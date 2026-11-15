# Baseline run: `spread-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88761 -> 88642) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 66.640 / 130.925 / 237.030 / 377.112 |
| MSPT mean, ms | 79.751 |
| TPS estimate (1000/p50) | 15.0 |
| heap used / max, MB | 2145.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8734` |
| 2 | 6.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7973` |
| 3 | 4.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13654` |
| 4 | 3.88% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.50% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.21% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 9 | 1.84% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13001` |
| 10 | 1.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5394` |
| 11 | 1.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7731` |
| 12 | 1.46% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.28% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 14 | 1.22% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 15 | 1.16% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |

