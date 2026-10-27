# Baseline run: `items-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54820 -> 54820) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 13.320 / 17.241 / 78.710 / 93.050 |
| MSPT mean, ms | 15.602 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1183.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 2 | 7.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 3 | 5.09% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2410` |
| 5 | 3.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 2.75% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2335` |
| 8 | 2.20% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 9 | 2.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2381` |
| 10 | 2.04% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 11 | 1.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 13 | 1.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.99% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |

