# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62962 -> 62962) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.555 / 86.061 / 165.410 / 249.787 |
| MSPT mean, ms | 40.366 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1257.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5537` |
| 2 | 7.08% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 5.61% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4071` |
| 4 | 4.15% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9598` |
| 5 | 2.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9591` |
| 7 | 2.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 1.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3944` |
| 9 | 1.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9554` |
| 10 | 1.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5542` |
| 11 | 1.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9551` |
| 12 | 1.25% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.23% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5613` |
| 15 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

