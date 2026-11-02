# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63013 -> 63013) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 26.817 / 75.552 / 134.942 / 181.323 |
| MSPT mean, ms | 36.955 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1444.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9116` |
| 2 | 8.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5403` |
| 3 | 6.30% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 5 | 5.51% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3996` |
| 6 | 4.88% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9063` |
| 7 | 4.86% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3869` |
| 9 | 1.68% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3949` |
| 10 | 1.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 11 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.58% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 13 | 1.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 14 | 1.32% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 1.24% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9077` |

