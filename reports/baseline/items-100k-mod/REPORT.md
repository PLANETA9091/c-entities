# Baseline run: `items-100k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54868 -> 54868) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 15.495 / 19.396 / 37.160 / 128.498 |
| MSPT mean, ms | 16.822 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1296.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.67% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 2 | 10.72% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 7.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 4 | 5.24% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6868` |
| 5 | 2.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 2.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2081` |
| 7 | 2.06% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 1.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3494` |
| 9 | 1.86% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 10 | 1.69% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 11 | 1.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.04% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2219` |

