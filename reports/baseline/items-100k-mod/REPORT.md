# Baseline run: `items-100k-mod`

Server: `26.3-235-bb10cf0 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63000 -> 63000) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 37.250 / 88.338 / 165.344 / 275.433 |
| MSPT mean, ms | 48.667 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1543.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 2 | 11.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 3 | 4.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.05% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 3.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 6 | 3.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3734` |
| 7 | 2.97% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 8 | 2.34% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5097` |
| 11 | 1.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 12 | 1.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 13 | 0.95% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2661` |
| 15 | 0.87% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |

