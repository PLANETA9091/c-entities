# Baseline run: `spread-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88734 -> 88583) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 47.133 / 67.022 / 185.932 / 357.959 |
| MSPT mean, ms | 55.416 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 2526.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 21.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9414` |
| 3 | 3.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14420` |
| 4 | 3.18% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 5 | 2.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8621` |
| 6 | 1.62% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 7 | 1.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14439` |
| 8 | 1.20% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7612` |
| 9 | 1.05% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 987` |
| 10 | 1.04% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14457` |
| 11 | 1.02% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 12 | 1.02% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13695` |
| 13 | 0.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5905` |
| 14 | 0.94% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 0.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5901` |

