# Baseline run: `items-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62925 -> 62925) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 23.084 / 52.035 / 119.162 / 194.440 |
| MSPT mean, ms | 31.788 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1417.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.93% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 7.53% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 5.44% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 4 | 3.99% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9770` |
| 6 | 3.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 7 | 3.19% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9777` |
| 8 | 2.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 9 | 2.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 2.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2835` |
| 11 | 1.84% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4152` |
| 12 | 1.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 13 | 1.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5684` |
| 14 | 1.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5781` |
| 15 | 1.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9733` |

