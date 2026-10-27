# Baseline run: `items-100k-mod`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54843 -> 54843) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 11.316 / 16.045 / 71.425 / 98.438 |
| MSPT mean, ms | 13.839 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1356.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.21% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 2 | 5.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2410` |
| 3 | 4.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 4 | 3.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 5 | 3.49% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 3.04% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 2.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2381` |
| 9 | 1.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2335` |
| 10 | 1.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.27% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3813` |
| 15 | 0.79% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |

