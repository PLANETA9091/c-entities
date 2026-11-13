# Baseline run: `mobs-25k-native-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25068 -> 25058) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 44.611 / 84.603 / 196.484 / 214.302 |
| MSPT mean, ms | 54.186 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1502.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.75% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11862` |
| 2 | 3.10% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.47% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 5 | 2.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12086` |
| 6 | 1.90% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 1.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7360` |
| 10 | 1.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.42% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11881` |
| 13 | 1.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.11% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 0.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

