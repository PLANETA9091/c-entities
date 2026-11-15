# Baseline run: `spread-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88714 -> 88583) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 58.531 / 108.827 / 205.122 / 341.936 |
| MSPT mean, ms | 69.708 |
| TPS estimate (1000/p50) | 17.1 |
| heap used / max, MB | 2169.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8794` |
| 2 | 7.91% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 3 | 5.32% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13714` |
| 4 | 4.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8033` |
| 5 | 4.13% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 3.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.82% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5454` |
| 8 | 2.47% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 1.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7769` |
| 10 | 1.34% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.30% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.25% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 13 | 1.24% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.24% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 1.14% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13061` |

