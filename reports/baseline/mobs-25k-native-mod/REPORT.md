# Baseline run: `mobs-25k-native-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25049 -> 25035) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 21.325 / 33.039 / 114.617 / 136.060 |
| MSPT mean, ms | 27.889 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1772.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13818` |
| 2 | 5.49% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.96% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 5 | 2.41% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13165` |
| 6 | 2.26% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8898` |
| 8 | 1.91% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.75% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 1.60% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 11 | 1.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.32% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.01% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14096` |

