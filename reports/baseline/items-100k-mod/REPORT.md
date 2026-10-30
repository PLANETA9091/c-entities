# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62944 -> 62944) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 25.972 / 72.514 / 130.333 / 191.428 |
| MSPT mean, ms | 36.055 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1187.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4737` |
| 2 | 8.99% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 3 | 5.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 4 | 4.66% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3438` |
| 5 | 3.94% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 6 | 3.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.91% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 8 | 2.28% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3330` |
| 10 | 1.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2450` |
| 11 | 1.79% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5530` |
| 14 | 1.68% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 1.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5528` |

