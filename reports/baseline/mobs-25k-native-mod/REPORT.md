# Baseline run: `mobs-25k-native-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25064 -> 25054) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 20.806 / 41.422 / 114.601 / 163.391 |
| MSPT mean, ms | 28.848 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1386.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.63% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 3.77% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15438` |
| 3 | 2.78% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14572` |
| 4 | 2.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10216` |
| 5 | 2.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.20% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 7 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.54% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 1.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.10% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.02% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 13 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.88% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 0.80% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |

