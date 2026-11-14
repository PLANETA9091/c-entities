# Baseline run: `mobs-25k-native-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25057 -> 25045) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 21.622 / 43.567 / 117.268 / 141.630 |
| MSPT mean, ms | 29.695 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1520.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.88% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 4.68% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12320` |
| 3 | 2.50% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.98% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 1.94% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 7 | 1.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7817` |
| 8 | 1.82% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.95% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 14 | 0.87% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 0.79% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

