# Baseline run: `items-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62969 -> 62969) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 30.840 / 81.179 / 170.872 / 236.917 |
| MSPT mean, ms | 41.395 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1353.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.82% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10491` |
| 2 | 7.34% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 4.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6120` |
| 4 | 4.68% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 3.92% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4568` |
| 6 | 2.99% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10482` |
| 8 | 1.90% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4436` |
| 11 | 1.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6191` |
| 12 | 1.43% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 1.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6055` |
| 14 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6064` |

