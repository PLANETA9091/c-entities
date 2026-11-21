# Baseline run: `mixed-150k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111877 -> 111376) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 61.288 / 102.135 / 254.777 / 388.896 |
| MSPT mean, ms | 75.565 |
| TPS estimate (1000/p50) | 16.3 |
| heap used / max, MB | 1986.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10331` |
| 2 | 6.71% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15622` |
| 3 | 5.98% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9082` |
| 5 | 3.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.44% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14742` |
| 7 | 1.51% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 8 | 1.47% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 9 | 1.37% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 10 | 1.28% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14965` |
| 11 | 1.27% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5924` |
| 12 | 1.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6600` |
| 13 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8843` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15643` |
| 15 | 0.98% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |

