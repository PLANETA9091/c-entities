# Baseline run: `spread-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88792 -> 88681) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 62.507 / 127.077 / 222.067 / 360.723 |
| MSPT mean, ms | 73.874 |
| TPS estimate (1000/p50) | 16.0 |
| heap used / max, MB | 2242.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8646` |
| 2 | 6.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7884` |
| 3 | 4.86% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13538` |
| 4 | 4.62% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.24% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 7 | 2.22% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.97% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5054` |
| 10 | 1.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5322` |
| 11 | 1.67% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12913` |
| 14 | 1.24% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 1.06% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |

