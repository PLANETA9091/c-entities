# Baseline run: `mobs-25k-native-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25096 -> 25077) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 23.871 / 44.124 / 115.937 / 170.117 |
| MSPT mean, ms | 31.305 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1637.8 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.18% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12263` |
| 2 | 3.08% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.01% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 4 | 2.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 5 | 2.79% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 6 | 2.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |
| 9 | 1.70% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 1.59% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12282` |
| 11 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.23% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 1.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12487` |

