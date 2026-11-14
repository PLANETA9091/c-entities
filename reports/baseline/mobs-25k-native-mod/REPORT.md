# Baseline run: `mobs-25k-native-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25062 -> 25051) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 16.191 / 29.826 / 99.237 / 153.755 |
| MSPT mean, ms | 22.220 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1574.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12511` |
| 2 | 3.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 3 | 2.98% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 4 | 2.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 5 | 1.89% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 1.84% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 7 | 1.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.39% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8008` |
| 11 | 1.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12735` |
| 12 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12530` |
| 14 | 1.09% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.89% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |

