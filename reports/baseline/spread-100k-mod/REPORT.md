# Baseline run: `spread-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88778 -> 88648) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 63.741 / 122.897 / 223.457 / 286.794 |
| MSPT mean, ms | 75.350 |
| TPS estimate (1000/p50) | 15.7 |
| heap used / max, MB | 2304.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 2 | 5.08% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14070` |
| 3 | 4.92% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 4.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8357` |
| 5 | 3.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.34% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.05% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 9 | 1.70% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13417` |
| 10 | 1.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5771` |
| 11 | 1.35% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.30% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.23% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8108` |

