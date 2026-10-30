# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62796 -> 62796) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 26.623 / 75.644 / 140.760 / 186.326 |
| MSPT mean, ms | 37.122 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1371.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4726` |
| 2 | 9.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8481` |
| 3 | 5.26% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3427` |
| 4 | 4.78% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8428` |
| 5 | 4.44% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.40% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 2.61% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 8 | 2.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3319` |
| 9 | 2.25% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5517` |
| 11 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2439` |
| 13 | 1.54% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3380` |
| 14 | 1.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5519` |
| 15 | 1.31% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |

