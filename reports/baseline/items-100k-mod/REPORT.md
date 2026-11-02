# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62999 -> 62999) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 24.760 / 54.221 / 129.840 / 150.515 |
| MSPT mean, ms | 33.557 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1605.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5505` |
| 2 | 7.34% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4071` |
| 3 | 4.98% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 4.34% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 3.47% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9559` |
| 7 | 2.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 2.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3944` |
| 9 | 2.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5510` |
| 10 | 2.08% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6283` |
| 11 | 1.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |
| 12 | 1.77% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4024` |
| 13 | 1.63% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9519` |
| 14 | 1.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9522` |
| 15 | 1.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2777` |

