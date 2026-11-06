# Baseline run: `items-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62868 -> 62868) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 30.152 / 80.585 / 156.592 / 268.083 |
| MSPT mean, ms | 40.106 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1729.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.25% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10521` |
| 2 | 9.18% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 7.61% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 4.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6120` |
| 5 | 4.36% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4568` |
| 6 | 3.88% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10512` |
| 8 | 2.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 2.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4436` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6064` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6055` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6883` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3237` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6191` |

