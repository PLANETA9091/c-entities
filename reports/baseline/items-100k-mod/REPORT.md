# Baseline run: `items-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62916 -> 62916) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 31.716 / 90.345 / 169.777 / 231.944 |
| MSPT mean, ms | 43.311 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1427.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10392` |
| 2 | 7.00% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 6.13% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 4.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 5 | 3.67% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 6 | 2.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.06% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 2.04% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10383` |
| 9 | 2.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6092` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 12 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.88% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |

