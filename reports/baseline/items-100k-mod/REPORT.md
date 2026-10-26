# Baseline run: `items-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54706 -> 54706) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 12.459 / 16.283 / 80.021 / 119.898 |
| MSPT mean, ms | 14.302 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1331.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.95% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 8.92% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4001` |
| 3 | 4.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 4 | 2.80% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 5 | 2.78% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 6 | 2.70% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.01% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 1.89% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 9 | 1.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2075` |
| 10 | 1.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2029` |
| 12 | 1.25% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3488` |
| 15 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

