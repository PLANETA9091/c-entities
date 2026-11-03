# Baseline run: `items-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63028 -> 63028) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 22.835 / 59.286 / 130.101 / 178.264 |
| MSPT mean, ms | 32.614 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1479.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.22% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 8.04% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 4.94% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4314` |
| 4 | 3.76% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9906` |
| 5 | 3.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9899` |
| 7 | 3.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5820` |
| 8 | 2.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4182` |
| 9 | 2.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 2.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5794` |
| 11 | 2.02% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4262` |
| 12 | 1.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6593` |
| 13 | 1.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2945` |
| 14 | 1.54% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 1.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9859` |

