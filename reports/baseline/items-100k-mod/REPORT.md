# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62959 -> 62959) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 31.403 / 69.667 / 144.554 / 193.752 |
| MSPT mean, ms | 41.359 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1584.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.38% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 2 | 8.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5546` |
| 3 | 7.35% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 6.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9618` |
| 5 | 3.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 6 | 2.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 8 | 2.03% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9574` |
| 9 | 2.01% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9571` |
| 10 | 1.94% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9611` |
| 11 | 1.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5622` |
| 12 | 1.54% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5551` |
| 14 | 1.28% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 15 | 1.24% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4030` |

