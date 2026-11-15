# Baseline run: `mobs-25k-native-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25048 -> 25038) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 23.110 / 48.844 / 120.036 / 219.768 |
| MSPT mean, ms | 31.761 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1662.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.17% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13714` |
| 2 | 4.92% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8794` |
| 4 | 2.93% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 5 | 2.74% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13061` |
| 6 | 2.29% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.21% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.64% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 1.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13992` |
| 13 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.14% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 0.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

