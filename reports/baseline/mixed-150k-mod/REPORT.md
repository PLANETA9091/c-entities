# Baseline run: `mixed-150k-mod`

Server: `26.3-259-c7097a9 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111851 -> 111327) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 59.992 / 102.430 / 236.630 / 416.870 |
| MSPT mean, ms | 74.595 |
| TPS estimate (1000/p50) | 16.7 |
| heap used / max, MB | 1815.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10717` |
| 2 | 6.60% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16008` |
| 3 | 5.27% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9416` |
| 6 | 2.80% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15128` |
| 7 | 1.74% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 8 | 1.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10761` |
| 9 | 1.33% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9162` |
| 11 | 1.20% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 12 | 1.19% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 13 | 1.11% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6705` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15351` |
| 15 | 1.07% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16029` |

