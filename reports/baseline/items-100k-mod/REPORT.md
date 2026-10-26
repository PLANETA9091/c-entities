# Baseline run: `items-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54793 -> 54793) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 14.382 / 17.339 / 104.157 / 135.510 |
| MSPT mean, ms | 16.692 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1379.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.25% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4001` |
| 2 | 11.17% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 7.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 4 | 2.76% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 5 | 2.05% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 1.98% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 7 | 1.95% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 1.74% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 9 | 1.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2075` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3488` |
| 11 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.88% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 0.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2029` |
| 15 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

