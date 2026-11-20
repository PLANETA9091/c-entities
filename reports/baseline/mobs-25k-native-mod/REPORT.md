# Baseline run: `mobs-25k-native-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25056 -> 25046) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 17.763 / 27.851 / 103.408 / 143.749 |
| MSPT mean, ms | 23.696 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1179.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15283` |
| 2 | 3.20% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 4 | 1.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15563` |
| 6 | 1.81% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 7 | 1.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 8 | 1.69% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10067` |
| 12 | 1.47% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 13 | 1.39% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 14 | 1.18% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 15 | 0.76% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14417` |

