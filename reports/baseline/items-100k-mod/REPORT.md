# Baseline run: `items-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54857 -> 54857) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 10.793 / 13.763 / 42.030 / 92.222 |
| MSPT mean, ms | 12.256 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1361.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7176` |
| 2 | 6.75% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7123` |
| 3 | 5.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4236` |
| 4 | 4.43% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.27% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 2.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 2.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2114` |
| 9 | 1.74% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 10 | 1.62% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 11 | 1.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3527` |
| 13 | 1.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.04% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |

