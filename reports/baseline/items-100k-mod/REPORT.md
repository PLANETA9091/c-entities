# Baseline run: `items-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54724 -> 54724) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 14.692 / 19.243 / 91.783 / 249.107 |
| MSPT mean, ms | 16.792 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1387.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.93% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 9.59% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4002` |
| 3 | 4.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4198` |
| 4 | 3.60% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6444` |
| 5 | 2.58% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 6 | 2.44% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.14% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 8 | 1.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3489` |
| 9 | 1.73% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2076` |
| 11 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.31% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.99% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2030` |

