# Baseline run: `items-100k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54835 -> 54835) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 15.325 / 19.644 / 85.879 / 155.024 |
| MSPT mean, ms | 17.523 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1075.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.91% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 9.26% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3846` |
| 3 | 5.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4024` |
| 4 | 3.88% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5646` |
| 5 | 3.86% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 6 | 2.42% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 7 | 2.10% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 2.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3352` |
| 9 | 1.76% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 10 | 1.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 11 | 1.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 14 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |

