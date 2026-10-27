# Baseline run: `items-100k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62863 -> 62863) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 12.197 / 16.915 / 78.118 / 119.510 |
| MSPT mean, ms | 15.563 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1427.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 2 | 5.55% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.78% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 4 | 3.46% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 5 | 3.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 6 | 2.01% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 1.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2417` |
| 8 | 1.90% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2463` |
| 9 | 1.80% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 10 | 1.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3895` |
| 11 | 1.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.01% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 15 | 0.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2492` |

