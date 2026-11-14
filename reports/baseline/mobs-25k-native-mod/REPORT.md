# Baseline run: `mobs-25k-native-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25057 -> 25050) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 20.590 / 37.765 / 109.339 / 214.452 |
| MSPT mean, ms | 28.138 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1595.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.96% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12887` |
| 2 | 3.92% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 4 | 2.57% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 2.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.00% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 7 | 1.88% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 8 | 1.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12906` |
| 9 | 1.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8219` |
| 11 | 1.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.67% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 13 | 1.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13129` |
| 14 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.22% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |

