# Baseline run: `mobs-25k-native-mod`

Server: `26.3-259-c7097a9 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25062 -> 25054) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 17.813 / 33.171 / 107.482 / 144.369 |
| MSPT mean, ms | 24.701 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1421.8 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.79% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 4.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16008` |
| 3 | 3.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.37% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15128` |
| 5 | 2.20% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.68% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 8 | 1.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.38% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 10 | 1.29% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 11 | 1.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16288` |
| 12 | 1.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.21% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.21% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 1.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10644` |

