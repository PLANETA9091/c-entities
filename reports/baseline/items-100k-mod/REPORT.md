# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62959 -> 62959) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 20.423 / 46.831 / 106.867 / 121.577 |
| MSPT mean, ms | 28.430 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1323.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.69% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 2 | 8.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5546` |
| 3 | 4.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.31% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4030` |
| 5 | 3.90% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 6 | 2.90% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 7 | 2.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5555` |
| 8 | 2.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 2.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9625` |
| 10 | 1.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6338` |
| 11 | 1.76% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |
| 12 | 1.76% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 1.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5669` |
| 14 | 1.73% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 15 | 1.53% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |

