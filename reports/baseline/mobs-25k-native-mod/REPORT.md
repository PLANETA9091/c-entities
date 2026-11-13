# Baseline run: `mobs-25k-native-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25055 -> 25046) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 47.282 / 120.860 / 218.903 / 275.801 |
| MSPT mean, ms | 58.050 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1424.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11904` |
| 2 | 4.09% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.73% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 2.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7401` |
| 6 | 1.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.64% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.16% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 0.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12128` |
| 12 | 0.68% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 13 | 0.65% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 0.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 15 | 0.59% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

