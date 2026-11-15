# Baseline run: `mobs-25k-native-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25060 -> 25046) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 21.498 / 39.032 / 114.032 / 155.151 |
| MSPT mean, ms | 29.526 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1741.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.83% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12953` |
| 2 | 3.20% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.74% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 2.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 5 | 2.20% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 2.01% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13195` |
| 8 | 1.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.78% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.74% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.66% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12972` |
| 12 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.39% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 14 | 1.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8285` |
| 15 | 1.24% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |

