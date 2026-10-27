# Baseline run: `items-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54861 -> 54861) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 8.740 / 12.854 / 60.475 / 71.927 |
| MSPT mean, ms | 10.354 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1349.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.81% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 2 | 4.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2410` |
| 3 | 4.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 5 | 2.61% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 2.31% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 7 | 1.99% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2381` |
| 8 | 1.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2335` |
| 10 | 1.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 12 | 1.66% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 13 | 1.66% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.07% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7413` |

