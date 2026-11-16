# Baseline run: `mobs-25k-native-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25043 -> 25025) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 15.585 / 26.423 / 93.804 / 112.878 |
| MSPT mean, ms | 20.980 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1528.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.71% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13818` |
| 2 | 4.10% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.15% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 6 | 2.15% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 7 | 2.10% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.60% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.30% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 13 | 1.30% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 14 | 1.15% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 1.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8898` |

