# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62880 -> 62880) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 20.798 / 51.512 / 111.322 / 178.396 |
| MSPT mean, ms | 28.834 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1179.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5461` |
| 2 | 5.80% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4041` |
| 3 | 5.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9374` |
| 4 | 4.84% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 5 | 3.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3914` |
| 6 | 3.20% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3994` |
| 7 | 2.94% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 8 | 2.38% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6240` |
| 10 | 2.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 2.07% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 12 | 2.04% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 1.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2720` |
| 14 | 1.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5467` |
| 15 | 1.44% | `net.minecraft.world.entity.Entity.getType() line: 633` |

