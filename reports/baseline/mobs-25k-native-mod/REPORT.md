# Baseline run: `mobs-25k-native-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25064 -> 25047) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 21.899 / 45.523 / 109.966 / 243.483 |
| MSPT mean, ms | 30.390 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1714.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14614` |
| 2 | 2.63% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 4 | 2.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9568` |
| 5 | 2.00% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.83% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.80% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 8 | 1.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14924` |
| 12 | 1.43% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13849` |
| 13 | 1.17% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.07% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 15 | 1.03% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |

