# Baseline run: `items-100k-mod`

Server: `26.3-235-bb10cf0 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63033 -> 63033) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 25.723 / 66.086 / 141.604 / 192.914 |
| MSPT mean, ms | 35.224 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1232.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 2 | 8.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5145` |
| 3 | 6.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 4 | 4.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 5 | 4.67% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 6 | 4.58% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 4.30% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.65% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 2.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 10 | 1.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.80% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.76% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3723` |
| 13 | 1.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5874` |
| 14 | 1.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 15 | 1.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5151` |

