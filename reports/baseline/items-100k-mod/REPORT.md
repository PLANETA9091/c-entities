# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62830 -> 62830) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 26.662 / 65.437 / 132.403 / 197.146 |
| MSPT mean, ms | 36.297 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1544.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4941` |
| 2 | 10.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8696` |
| 3 | 5.02% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.70% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3642` |
| 5 | 4.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8643` |
| 6 | 3.30% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 2.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3534` |
| 8 | 2.64% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.42% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 10 | 2.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2598` |
| 11 | 1.89% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3595` |
| 12 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5734` |
| 14 | 1.51% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 15 | 1.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |

