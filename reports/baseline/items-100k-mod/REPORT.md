# Baseline run: `items-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54871 -> 54871) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 13.795 / 17.614 / 84.575 / 104.501 |
| MSPT mean, ms | 15.432 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1054.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.68% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 7.02% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 3 | 5.29% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6863` |
| 4 | 4.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 5 | 2.62% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 6 | 2.57% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.13% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 8 | 2.05% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2035` |
| 10 | 1.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2081` |
| 11 | 1.51% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3494` |
| 14 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

