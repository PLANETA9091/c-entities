# Baseline run: `mixed-150k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 112184 -> 112075) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 49.710 / 76.054 / 176.832 / 251.063 |
| MSPT mean, ms | 57.813 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 2408.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 5.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8698` |
| 3 | 5.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7936` |
| 4 | 4.80% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 4.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13618` |
| 6 | 2.55% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.24% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 2.07% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13655` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13637` |
| 10 | 1.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.29% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8708` |
| 13 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7678` |
| 14 | 1.04% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12965` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5378` |

