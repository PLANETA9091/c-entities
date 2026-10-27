# Baseline run: `items-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54833 -> 54833) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 11.560 / 16.032 / 100.227 / 125.941 |
| MSPT mean, ms | 15.252 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1126.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 2 | 5.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 4 | 3.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 5 | 3.36% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 2.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 2.03% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 8 | 1.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2089` |
| 9 | 1.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3567` |
| 10 | 1.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2135` |
| 12 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.70% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 15 | 0.70% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7177` |

