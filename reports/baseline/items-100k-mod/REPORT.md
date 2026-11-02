# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62909 -> 62909) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.306 / 97.473 / 173.719 / 268.311 |
| MSPT mean, ms | 44.072 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1475.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.97% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 2 | 10.03% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 9.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5471` |
| 4 | 4.12% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4052` |
| 5 | 3.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 1.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 1.64% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5476` |
| 9 | 1.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9388` |
| 10 | 1.12% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 11 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9385` |
| 13 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3925` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5547` |

