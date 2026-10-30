# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62907 -> 62907) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 33.587 / 81.599 / 167.252 / 309.649 |
| MSPT mean, ms | 45.157 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1122.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4838` |
| 2 | 11.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8593` |
| 3 | 5.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.81% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 3.31% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8540` |
| 6 | 3.11% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 379` |
| 7 | 2.76% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3539` |
| 8 | 2.66% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2525` |
| 10 | 1.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3431` |
| 11 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.01% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5629` |
| 13 | 0.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 14 | 0.78% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 15 | 0.78% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8554` |

