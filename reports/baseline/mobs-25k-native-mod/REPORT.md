# Baseline run: `mobs-25k-native-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25059 -> 25053) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 19.409 / 34.198 / 108.906 / 130.581 |
| MSPT mean, ms | 26.711 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1669.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.61% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 4.84% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13865` |
| 3 | 2.73% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.88% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 7 | 1.50% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13212` |
| 8 | 1.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.46% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8945` |
| 13 | 1.15% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 0.92% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13884` |
| 15 | 0.77% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

