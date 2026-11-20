# Baseline run: `mobs-25k-native-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25054 -> 25047) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 15.442 / 26.968 / 88.350 / 96.603 |
| MSPT mean, ms | 20.831 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1148.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.81% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15127` |
| 2 | 2.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15404` |
| 3 | 2.44% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 4 | 2.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 5 | 1.93% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 1.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 7 | 1.67% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9932` |
| 10 | 1.37% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 11 | 1.33% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 12 | 1.33% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 13 | 1.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.77% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

