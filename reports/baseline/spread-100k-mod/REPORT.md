# Baseline run: `spread-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88723 -> 88580) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 59.988 / 106.538 / 198.453 / 296.398 |
| MSPT mean, ms | 70.479 |
| TPS estimate (1000/p50) | 16.7 |
| heap used / max, MB | 2401.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8945` |
| 2 | 5.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8152` |
| 3 | 5.14% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13865` |
| 4 | 4.84% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.91% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.42% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 8 | 2.35% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 1.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5566` |
| 10 | 1.67% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7903` |
| 12 | 1.55% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13212` |
| 14 | 1.08% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 1.05% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |

