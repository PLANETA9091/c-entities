# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62913 -> 62913) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.284 / 79.556 / 169.378 / 219.777 |
| MSPT mean, ms | 43.483 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1563.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5472` |
| 2 | 10.97% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9395` |
| 3 | 8.21% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.70% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4052` |
| 5 | 4.31% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 1.81% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.22% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9390` |
| 9 | 1.14% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9387` |
| 10 | 1.04% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 11 | 1.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 0.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3925` |
| 14 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2758` |
| 15 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2720` |

