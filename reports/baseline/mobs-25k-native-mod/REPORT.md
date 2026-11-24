# Baseline run: `mobs-25k-native-mod`

Server: `26.3-262-827eeeb (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25066 -> 25052) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 18.765 / 36.222 / 107.397 / 139.789 |
| MSPT mean, ms | 25.761 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1397.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.11% | `dev.planeta9091.centities.CeHooks.sitStepsEmpty(Object) line: 15343` |
| 2 | 4.96% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.10% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 5 | 1.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 6 | 1.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.58% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.38% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 10 | 1.38% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 11 | 1.34% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.24% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15210` |
| 14 | 1.15% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 15 | 1.15% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |

