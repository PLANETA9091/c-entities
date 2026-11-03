# Baseline run: `items-100k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62937 -> 62937) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 26.162 / 58.871 / 154.146 / 210.364 |
| MSPT mean, ms | 36.512 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1649.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.84% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 6.44% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 4.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5820` |
| 4 | 3.90% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4314` |
| 5 | 3.09% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 2.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9906` |
| 8 | 2.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4182` |
| 9 | 1.96% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9899` |
| 10 | 1.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9859` |
| 11 | 1.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5891` |
| 12 | 1.26% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 1.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5794` |
| 15 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

