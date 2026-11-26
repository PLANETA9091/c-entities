# Baseline run: `mobs-25k-native-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25073 -> 25059) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 20.875 / 43.886 / 110.222 / 190.063 |
| MSPT mean, ms | 29.198 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1251.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.64% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 3.63% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16449` |
| 3 | 2.52% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15240` |
| 4 | 2.45% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 5 | 1.93% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.78% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.74% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.45% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.45% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 12 | 1.37% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10756` |
| 14 | 1.04% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 15 | 1.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |

