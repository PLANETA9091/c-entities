# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62870 -> 62870) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 35.365 / 83.756 / 171.757 / 195.621 |
| MSPT mean, ms | 46.787 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1187.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4737` |
| 2 | 11.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 3 | 7.41% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.40% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 6 | 3.24% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 7 | 2.70% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3438` |
| 8 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 1.29% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4801` |
| 11 | 1.03% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8453` |
| 12 | 1.02% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 13 | 0.97% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2450` |
| 14 | 0.92% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5528` |

