# Baseline run: `mobs-25k-native-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25057 -> 25049) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 21.400 / 37.419 / 107.656 / 214.502 |
| MSPT mean, ms | 28.990 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1672.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.07% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 2.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14509` |
| 3 | 2.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 4 | 2.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 5 | 2.06% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 6 | 1.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9503` |
| 8 | 1.58% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 9 | 1.42% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13784` |
| 10 | 1.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14528` |
| 11 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.28% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.21% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 1.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14800` |
| 15 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |

