# Baseline run: `items-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63002 -> 63002) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 36.646 / 96.696 / 171.800 / 252.480 |
| MSPT mean, ms | 48.420 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1269.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8939` |
| 2 | 10.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5226` |
| 3 | 5.61% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.25% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 5.10% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 3.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8886` |
| 7 | 3.56% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3819` |
| 8 | 2.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5962` |
| 9 | 1.97% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 10 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.02% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8900` |
| 12 | 0.99% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5232` |

