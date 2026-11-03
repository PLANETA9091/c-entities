# Baseline run: `items-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62862 -> 62862) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 24.612 / 56.789 / 135.367 / 241.840 |
| MSPT mean, ms | 33.864 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1381.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.56% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 5.59% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 4.07% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.02% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 5 | 3.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 6 | 3.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 2.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 8 | 2.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9789` |
| 9 | 2.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5684` |
| 10 | 2.10% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9796` |
| 11 | 1.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5781` |
| 12 | 1.70% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9749` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9752` |
| 14 | 1.25% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 1.22% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4152` |

