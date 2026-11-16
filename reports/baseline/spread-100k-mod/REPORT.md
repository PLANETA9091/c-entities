# Baseline run: `spread-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88789 -> 88671) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 57.283 / 86.365 / 207.798 / 435.496 |
| MSPT mean, ms | 67.902 |
| TPS estimate (1000/p50) | 17.5 |
| heap used / max, MB | 2134.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8945` |
| 2 | 6.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8152` |
| 3 | 5.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13865` |
| 4 | 5.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 4.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.77% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5566` |
| 8 | 3.48% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 1.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13212` |
| 11 | 1.38% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.30% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.11% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 14 | 1.06% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 0.94% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |

