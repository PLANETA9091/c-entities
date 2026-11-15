# Baseline run: `mobs-25k-native-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25051 -> 25041) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 19.018 / 34.531 / 101.506 / 196.927 |
| MSPT mean, ms | 25.607 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1645.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13003` |
| 2 | 4.37% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.35% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 5 | 2.16% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 2.07% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.73% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13245` |
| 12 | 1.15% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 13 | 1.10% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8335` |
| 15 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |

