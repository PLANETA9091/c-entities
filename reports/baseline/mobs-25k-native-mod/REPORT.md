# Baseline run: `mobs-25k-native-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25052 -> 25040) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 19.315 / 34.866 / 104.480 / 247.566 |
| MSPT mean, ms | 26.773 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1915.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.51% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 5.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12967` |
| 3 | 2.63% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 2.41% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 6 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.72% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.47% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 10 | 1.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8299` |
| 11 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12986` |
| 15 | 0.99% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |

