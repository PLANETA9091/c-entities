# Baseline run: `items-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63029 -> 63029) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 28.954 / 69.272 / 150.263 / 329.075 |
| MSPT mean, ms | 39.590 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1467.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.67% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 4.99% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 4.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5962` |
| 4 | 4.26% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4456` |
| 5 | 4.16% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10048` |
| 7 | 2.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 2.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4324` |
| 9 | 2.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10041` |
| 10 | 1.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6033` |
| 11 | 1.87% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10001` |
| 12 | 1.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10004` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5936` |
| 14 | 1.24% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

