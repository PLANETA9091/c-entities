# Baseline run: `spread-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88750 -> 88615) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 66.074 / 125.735 / 224.412 / 256.592 |
| MSPT mean, ms | 77.895 |
| TPS estimate (1000/p50) | 15.1 |
| heap used / max, MB | 2262.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.76% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 6.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8105` |
| 3 | 5.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 5.02% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13818` |
| 5 | 4.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 4.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8898` |
| 7 | 3.32% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 2.91% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 2.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7834` |
| 10 | 2.23% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.44% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 12 | 1.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5519` |
| 13 | 1.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5523` |
| 14 | 1.19% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13165` |

