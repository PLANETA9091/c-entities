# Baseline run: `mobs-25k-native-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25050 -> 25047) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 25.242 / 51.915 / 123.484 / 249.448 |
| MSPT mean, ms | 33.758 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1689.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.92% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13003` |
| 2 | 4.04% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.22% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8335` |
| 6 | 2.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 1.58% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.24% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 0.94% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 0.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13245` |
| 13 | 0.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.73% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 15 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |

