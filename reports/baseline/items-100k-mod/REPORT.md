# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62941 -> 62941) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 27.971 / 74.329 / 163.197 / 250.990 |
| MSPT mean, ms | 38.884 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1415.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5537` |
| 2 | 6.97% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 4.75% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9598` |
| 4 | 4.69% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4071` |
| 5 | 2.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 2.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9591` |
| 8 | 1.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9551` |
| 9 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5542` |
| 10 | 1.27% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.17% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5613` |
| 13 | 1.11% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 1.11% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9554` |
| 15 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

