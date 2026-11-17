# Baseline run: `mixed-150k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 111750 -> 111355) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 61.652 / 101.519 / 220.073 / 301.252 |
| MSPT mean, ms | 73.956 |
| TPS estimate (1000/p50) | 16.2 |
| heap used / max, MB | 2319.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9503` |
| 2 | 8.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14509` |
| 4 | 3.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8660` |
| 5 | 3.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.85% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7697` |
| 7 | 2.33% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13784` |
| 8 | 1.59% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.55% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.51% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14546` |
| 12 | 1.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8446` |
| 13 | 1.44% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 14 | 1.28% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 15 | 1.03% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5990` |

