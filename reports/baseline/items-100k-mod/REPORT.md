# Baseline run: `items-100k-mod`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54826 -> 54826) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 14.620 / 19.108 / 98.737 / 137.385 |
| MSPT mean, ms | 16.840 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1147.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 11.65% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4001` |
| 3 | 6.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 4 | 2.89% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6418` |
| 5 | 2.85% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 2.48% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 7 | 2.26% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2075` |
| 9 | 1.85% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3488` |
| 11 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.91% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2213` |
| 15 | 0.80% | `net.minecraft.world.entity.Entity.getType() line: 633` |

