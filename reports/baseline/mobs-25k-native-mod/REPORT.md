# Baseline run: `mobs-25k-native-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25057 -> 25041) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 19.097 / 35.506 / 112.364 / 151.773 |
| MSPT mean, ms | 26.685 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1570.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.05% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 4.63% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14070` |
| 3 | 2.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.11% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 5 | 2.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 7 | 1.96% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.66% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13417` |
| 9 | 1.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.36% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.13% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14089` |
| 14 | 1.02% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 0.98% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |

