# Baseline run: `mobs-25k-native-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25068 -> 25058) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 23.534 / 46.214 / 141.107 / 243.299 |
| MSPT mean, ms | 32.361 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1676.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14355` |
| 2 | 4.19% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.60% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.89% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13654` |
| 5 | 2.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9373` |
| 6 | 2.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.77% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.03% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 11 | 0.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.65% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 0.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

