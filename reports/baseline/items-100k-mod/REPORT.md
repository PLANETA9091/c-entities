# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62874 -> 62874) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 27.519 / 72.028 / 161.167 / 204.118 |
| MSPT mean, ms | 38.452 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1769.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4944` |
| 2 | 9.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8699` |
| 3 | 5.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8646` |
| 4 | 5.20% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 5.04% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3645` |
| 6 | 4.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.59% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 2.43% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 9 | 2.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3537` |
| 10 | 2.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2601` |
| 12 | 1.60% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3598` |
| 13 | 1.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5735` |
| 14 | 1.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5737` |
| 15 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5008` |

