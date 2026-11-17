# Baseline run: `spread-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88750 -> 88617) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 54.809 / 78.718 / 209.673 / 292.024 |
| MSPT mean, ms | 64.526 |
| TPS estimate (1000/p50) | 18.2 |
| heap used / max, MB | 2184.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 6.57% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 6.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 5.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9373` |
| 5 | 3.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8580` |
| 6 | 3.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14355` |
| 7 | 3.21% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 1.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5864` |
| 10 | 1.17% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 11 | 1.07% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13654` |
| 12 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14392` |
| 13 | 0.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5860` |
| 14 | 0.93% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 0.85% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |

