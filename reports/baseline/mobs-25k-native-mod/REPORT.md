# Baseline run: `mobs-25k-native-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25045 -> 25036) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 43.490 / 75.114 / 226.346 / 371.495 |
| MSPT mean, ms | 53.942 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1455.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11976` |
| 2 | 3.90% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.26% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7473` |
| 5 | 2.07% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 1.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 1.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.96% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 13 | 0.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12200` |
| 14 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11995` |
| 15 | 0.55% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |

