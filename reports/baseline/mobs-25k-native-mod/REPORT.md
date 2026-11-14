# Baseline run: `mobs-25k-native-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25062 -> 25041) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 20.467 / 35.296 / 105.510 / 147.255 |
| MSPT mean, ms | 27.348 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1370.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.69% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12804` |
| 2 | 2.81% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 2.77% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 4 | 2.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 5 | 2.17% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8292` |
| 7 | 1.78% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 8 | 1.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13032` |
| 9 | 1.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.54% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 13 | 1.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 1.03% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12823` |

