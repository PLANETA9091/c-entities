# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62987 -> 62987) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.149 / 91.088 / 150.291 / 182.344 |
| MSPT mean, ms | 45.558 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1177.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 2 | 10.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4073` |
| 3 | 5.65% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.94% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 4.04% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2769` |
| 7 | 2.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 8 | 1.82% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 10 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7683` |
| 13 | 0.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4784` |
| 14 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4110` |

