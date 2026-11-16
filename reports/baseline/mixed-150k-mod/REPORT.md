# Baseline run: `mixed-150k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 112245 -> 112080) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 79.843 / 115.734 / 227.015 / 331.453 |
| MSPT mean, ms | 89.753 |
| TPS estimate (1000/p50) | 12.5 |
| heap used / max, MB | 2201.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 7.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 6.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8055` |
| 4 | 5.90% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13818` |
| 5 | 4.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8898` |
| 6 | 2.95% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.01% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5523` |
| 11 | 1.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5519` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13855` |
| 13 | 1.23% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13165` |
| 15 | 1.11% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |

