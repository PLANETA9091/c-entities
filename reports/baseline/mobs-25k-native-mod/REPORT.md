# Baseline run: `mobs-25k-native-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25060 -> 25047) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 38.080 / 61.902 / 201.043 / 221.962 |
| MSPT mean, ms | 47.576 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1486.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11848` |
| 2 | 3.60% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 4 | 2.41% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 2.05% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.62% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12072` |
| 12 | 1.22% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 13 | 1.08% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11867` |
| 14 | 0.97% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 15 | 0.94% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |

