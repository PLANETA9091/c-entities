# Baseline run: `spread-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88785 -> 88676) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 63.942 / 122.475 / 232.725 / 323.429 |
| MSPT mean, ms | 75.897 |
| TPS estimate (1000/p50) | 15.6 |
| heap used / max, MB | 2342.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8898` |
| 2 | 5.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8105` |
| 3 | 5.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13818` |
| 4 | 4.72% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 4.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.39% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 1.72% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 9 | 1.66% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.55% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5519` |
| 12 | 1.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7834` |
| 13 | 1.46% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13165` |
| 14 | 1.33% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 15 | 1.23% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |

