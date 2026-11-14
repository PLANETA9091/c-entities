# Baseline run: `mobs-25k-native-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25050 -> 25042) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 20.312 / 36.164 / 107.067 / 148.135 |
| MSPT mean, ms | 27.526 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1719.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.12% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12886` |
| 2 | 2.96% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.84% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 2.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 5 | 2.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 6 | 1.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.95% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.56% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8218` |
| 12 | 1.52% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12905` |
| 13 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13128` |
| 15 | 1.17% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |

