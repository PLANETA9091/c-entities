# Baseline run: `items-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62929 -> 62929) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 19.173 / 46.357 / 103.063 / 178.076 |
| MSPT mean, ms | 27.123 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1722.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 6.03% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.94% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 4 | 3.31% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 3.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 6 | 3.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4152` |
| 7 | 2.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5684` |
| 8 | 2.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 9 | 2.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5781` |
| 10 | 2.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2835` |
| 11 | 1.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 12 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.64% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 1.55% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 15 | 1.45% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9770` |

