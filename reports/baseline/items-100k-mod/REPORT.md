# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54832 -> 54832) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 10.390 / 14.650 / 72.383 / 103.715 |
| MSPT mean, ms | 12.921 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1327.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7525` |
| 2 | 5.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7472` |
| 3 | 5.13% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.76% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 5 | 3.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2398` |
| 6 | 3.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4585` |
| 7 | 3.14% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 8 | 2.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3876` |
| 10 | 1.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2444` |
| 11 | 1.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.43% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 13 | 1.15% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

