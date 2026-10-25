# Baseline run: `items-100k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54819 -> 54819) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 15.157 / 19.022 / 81.654 / 104.161 |
| MSPT mean, ms | 17.533 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1155.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.18% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 10.34% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3489` |
| 3 | 4.77% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3488` |
| 4 | 3.93% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4469` |
| 5 | 3.36% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 6 | 2.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3670` |
| 7 | 2.76% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 1.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 9 | 1.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 11 | 1.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2978` |
| 12 | 1.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 13 | 0.99% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

