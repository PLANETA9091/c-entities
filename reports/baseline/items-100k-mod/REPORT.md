# Baseline run: `items-100k-mod`

Server: `26.3-242-fd28d65 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62920 -> 62920) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 33.851 / 98.405 / 184.744 / 325.505 |
| MSPT mean, ms | 45.630 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1528.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.71% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10303` |
| 2 | 8.67% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 7.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 4 | 5.38% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 3.86% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 6 | 2.44% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10294` |
| 8 | 1.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 1.56% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6092` |
| 11 | 1.18% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10298` |
| 12 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 13 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 15 | 0.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

