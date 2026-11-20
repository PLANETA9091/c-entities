# Baseline run: `mixed-150k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111842 -> 111307) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 70.895 / 113.508 / 290.226 / 411.800 |
| MSPT mean, ms | 86.288 |
| TPS estimate (1000/p50) | 14.1 |
| heap used / max, MB | 1979.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9942` |
| 2 | 6.79% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 6.01% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15127` |
| 4 | 4.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8799` |
| 5 | 2.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.49% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14282` |
| 7 | 1.89% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1018` |
| 8 | 1.84% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 9 | 1.65% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 10 | 1.32% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 11 | 1.29% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5783` |
| 12 | 1.10% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 13 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8585` |
| 14 | 0.94% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14481` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15145` |

