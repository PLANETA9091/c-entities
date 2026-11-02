# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62848 -> 62848) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 20.753 / 46.927 / 122.816 / 183.262 |
| MSPT mean, ms | 28.644 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1469.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5543` |
| 2 | 7.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 3 | 4.41% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.28% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4030` |
| 5 | 3.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 6 | 2.59% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9608` |
| 7 | 2.51% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 8 | 2.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 2.40% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 2.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6321` |
| 11 | 2.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5548` |
| 12 | 1.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |
| 13 | 1.79% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9615` |
| 14 | 1.68% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 1.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2777` |

