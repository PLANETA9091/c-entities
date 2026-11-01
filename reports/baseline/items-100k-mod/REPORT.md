# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62987 -> 62987) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 37.091 / 98.889 / 170.166 / 221.140 |
| MSPT mean, ms | 48.566 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1323.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8988` |
| 2 | 9.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5275` |
| 3 | 5.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 5.04% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 4.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 4.00% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3868` |
| 7 | 3.55% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8935` |
| 8 | 1.72% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6011` |
| 10 | 1.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3741` |
| 11 | 1.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8949` |
| 14 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5281` |

