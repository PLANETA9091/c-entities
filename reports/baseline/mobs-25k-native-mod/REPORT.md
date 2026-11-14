# Baseline run: `mobs-25k-native-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25062 -> 25029) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 16.998 / 33.975 / 103.256 / 137.005 |
| MSPT mean, ms | 23.985 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1536.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.90% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12263` |
| 2 | 2.90% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 4 | 1.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |
| 5 | 1.81% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12282` |
| 6 | 1.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 7 | 1.66% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 8 | 1.62% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.43% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 12 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.09% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12487` |
| 15 | 0.90% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |

