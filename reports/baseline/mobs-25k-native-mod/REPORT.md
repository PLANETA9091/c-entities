# Baseline run: `mobs-25k-native-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25059 -> 25043) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 17.748 / 30.480 / 109.734 / 146.227 |
| MSPT mean, ms | 24.227 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1242.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15535` |
| 2 | 4.04% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.24% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 3.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.52% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14655` |
| 6 | 2.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10280` |
| 7 | 2.16% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.75% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 10 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15815` |
| 13 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.21% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 0.94% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |

