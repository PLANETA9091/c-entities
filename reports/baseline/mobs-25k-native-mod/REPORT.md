# Baseline run: `mobs-25k-native-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25065 -> 25045) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 19.826 / 34.499 / 112.963 / 193.262 |
| MSPT mean, ms | 27.107 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1394.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15622` |
| 2 | 4.71% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 4 | 1.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15902` |
| 5 | 1.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 6 | 1.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.73% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.69% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.54% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14742` |
| 11 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.39% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 13 | 1.24% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 14 | 1.17% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 1.09% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |

