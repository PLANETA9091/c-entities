# Baseline run: `items-100k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54787 -> 54787) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 14.115 / 18.185 / 78.393 / 125.331 |
| MSPT mean, ms | 15.902 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1333.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 8.01% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3613` |
| 3 | 5.20% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 4 | 4.33% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5413` |
| 5 | 4.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3791` |
| 6 | 2.73% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.09% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 1.90% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3119` |
| 10 | 1.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 11 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 13 | 1.05% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

