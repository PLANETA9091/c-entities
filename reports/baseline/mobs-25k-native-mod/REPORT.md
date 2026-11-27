# Baseline run: `mobs-25k-native-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25055 -> 25037) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 22.566 / 39.427 / 118.177 / 175.901 |
| MSPT mean, ms | 30.896 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1396.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.87% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16623` |
| 2 | 4.75% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13967` |
| 3 | 2.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16901` |
| 4 | 2.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.08% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15343` |
| 6 | 2.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10813` |
| 8 | 1.73% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 9 | 1.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.47% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 1.44% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 12 | 0.88% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.79% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 0.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

