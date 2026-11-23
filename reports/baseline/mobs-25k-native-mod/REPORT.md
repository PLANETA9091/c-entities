# Baseline run: `mobs-25k-native-mod`

Server: `26.3-259-c7097a9 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25076 -> 25038) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 21.096 / 45.752 / 119.533 / 227.960 |
| MSPT mean, ms | 30.011 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1256.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.90% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 4.00% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15992` |
| 3 | 2.25% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15112` |
| 4 | 1.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.68% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.54% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 8 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10628` |
| 11 | 1.11% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 12 | 1.07% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16013` |
| 13 | 0.89% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 0.79% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |

