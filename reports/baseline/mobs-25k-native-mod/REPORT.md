# Baseline run: `mobs-25k-native-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25060 -> 25050) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 23.049 / 45.818 / 115.477 / 124.434 |
| MSPT mean, ms | 31.388 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1739.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12311` |
| 2 | 3.67% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.82% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7808` |
| 6 | 1.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 7 | 1.58% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.23% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 1.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 0.89% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.66% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12535` |
| 14 | 0.66% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 0.63% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

