# Baseline run: `items-100k-mod`

Server: `26.3-240-6b68358 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62930 -> 62930) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 28.606 / 84.325 / 162.151 / 188.433 |
| MSPT mean, ms | 39.375 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1658.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.86% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10083` |
| 2 | 7.85% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 4.71% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 3.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5979` |
| 5 | 3.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4468` |
| 6 | 2.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4336` |
| 8 | 2.10% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10074` |
| 9 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5953` |
| 11 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6050` |
| 12 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5944` |
| 13 | 1.01% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 1.01% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

