# Baseline run: `items-100k-mod`

Server: `26.3-239-13565bc (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62944 -> 62944) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 25.313 / 55.748 / 129.842 / 210.473 |
| MSPT mean, ms | 34.561 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1508.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.34% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 7.02% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 4.39% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10093` |
| 5 | 4.07% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 6 | 3.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 3.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5978` |
| 8 | 2.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 9 | 2.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10086` |
| 10 | 1.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5952` |
| 11 | 1.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6049` |
| 12 | 1.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10049` |
| 13 | 1.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10046` |
| 14 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |
| 15 | 1.22% | `net.minecraft.world.entity.Entity.getType() line: 633` |

