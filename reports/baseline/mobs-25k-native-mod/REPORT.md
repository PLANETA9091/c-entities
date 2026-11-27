# Baseline run: `mobs-25k-native-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25065 -> 25015) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 20.059 / 43.322 / 117.904 / 140.156 |
| MSPT mean, ms | 28.507 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1335.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16571` |
| 2 | 3.27% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13915` |
| 3 | 2.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 4 | 1.83% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 5 | 1.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.34% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15291` |
| 8 | 1.31% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 9 | 1.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10761` |
| 10 | 1.21% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 11 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.88% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 0.88% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 0.88% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |

