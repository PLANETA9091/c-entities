# Baseline run: `items-100k-mod`

Server: `26.3-239-13565bc (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62995 -> 62995) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 26.585 / 82.730 / 153.770 / 178.942 |
| MSPT mean, ms | 37.034 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1579.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.79% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 5.22% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 4.03% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10093` |
| 4 | 3.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5978` |
| 5 | 3.79% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 6 | 3.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 1.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 9 | 1.73% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10086` |
| 10 | 1.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6049` |
| 11 | 1.71% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10049` |
| 12 | 1.60% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10046` |
| 13 | 1.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |
| 15 | 1.08% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |

