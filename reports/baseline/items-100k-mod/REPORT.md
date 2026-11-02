# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62929 -> 62929) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 27.218 / 69.552 / 159.995 / 173.470 |
| MSPT mean, ms | 37.315 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1680.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5454` |
| 2 | 7.53% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 4.56% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4070` |
| 4 | 4.52% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 3.03% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 2.19% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9508` |
| 8 | 1.46% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9468` |
| 9 | 1.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9471` |
| 10 | 1.31% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 11 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5459` |
| 12 | 1.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3943` |
| 13 | 1.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5530` |
| 14 | 1.05% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 1.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6232` |

