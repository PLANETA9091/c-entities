# Baseline run: `items-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54806 -> 54806) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 12.755 / 16.446 / 94.962 / 142.121 |
| MSPT mean, ms | 15.024 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1344.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 7.52% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4006` |
| 3 | 4.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 4 | 3.09% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 5 | 2.79% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 2.65% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6531` |
| 7 | 2.40% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 8 | 2.01% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.57% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2034` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3493` |
| 12 | 1.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2080` |
| 13 | 1.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

