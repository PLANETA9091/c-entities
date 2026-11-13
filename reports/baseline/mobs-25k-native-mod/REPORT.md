# Baseline run: `mobs-25k-native-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25050 -> 25045) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 47.189 / 94.221 / 219.467 / 239.434 |
| MSPT mean, ms | 58.222 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1426.8 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.18% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 4.98% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11849` |
| 3 | 3.04% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 2.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7347` |
| 6 | 1.81% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.20% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 12 | 0.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 0.82% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 0.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12073` |
| 15 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |

