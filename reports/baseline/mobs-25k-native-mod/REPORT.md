# Baseline run: `mobs-25k-native-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25062 -> 25044) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 18.844 / 34.546 / 113.866 / 192.006 |
| MSPT mean, ms | 26.911 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1330.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.34% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 4.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15846` |
| 3 | 2.47% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14966` |
| 4 | 2.24% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 5 | 2.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.53% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 9 | 1.37% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.10% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 12 | 1.06% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 13 | 0.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10482` |
| 14 | 0.90% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15867` |
| 15 | 0.86% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |

