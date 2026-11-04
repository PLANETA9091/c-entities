# Baseline run: `items-100k-mod`

Server: `26.3-240-6b68358 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62999 -> 62999) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.569 / 87.059 / 167.923 / 244.252 |
| MSPT mean, ms | 40.648 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1550.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.67% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 6.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5978` |
| 3 | 5.73% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10094` |
| 4 | 4.09% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 3.61% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 6 | 2.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 2.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10047` |
| 9 | 1.72% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10050` |
| 10 | 1.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10087` |
| 11 | 1.39% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6049` |
| 13 | 1.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |
| 14 | 0.96% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 15 | 0.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3136` |

