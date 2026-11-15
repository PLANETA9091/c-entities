# Baseline run: `spread-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88760 -> 88620) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 52.697 / 80.824 / 181.333 / 203.518 |
| MSPT mean, ms | 60.604 |
| TPS estimate (1000/p50) | 19.0 |
| heap used / max, MB | 2217.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 6.86% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7963` |
| 4 | 4.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13644` |
| 5 | 3.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8724` |
| 6 | 3.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.95% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 2.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5388` |
| 9 | 2.40% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7717` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5384` |
| 13 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7773` |
| 14 | 1.16% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13663` |
| 15 | 1.14% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12991` |

