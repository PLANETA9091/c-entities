# Baseline run: `items-100k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54812 -> 54812) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 13.487 / 18.485 / 79.096 / 113.252 |
| MSPT mean, ms | 15.539 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1344.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 8.86% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3870` |
| 3 | 4.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4066` |
| 4 | 3.41% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6277` |
| 5 | 2.84% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 2.53% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 7 | 2.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 2.34% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 9 | 1.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3357` |
| 10 | 1.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 12 | 1.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.07% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

