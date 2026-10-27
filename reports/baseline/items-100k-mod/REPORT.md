# Baseline run: `items-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54792 -> 54792) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 13.081 / 17.839 / 96.579 / 151.651 |
| MSPT mean, ms | 15.490 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1287.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.76% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 8.86% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 3 | 5.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 4 | 3.25% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 5 | 3.11% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6870` |
| 6 | 2.46% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 8 | 1.91% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3494` |
| 10 | 1.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2035` |
| 11 | 1.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2081` |
| 12 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.98% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |

