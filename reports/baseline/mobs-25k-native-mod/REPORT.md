# Baseline run: `mobs-25k-native-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25065 -> 25057) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 25.342 / 57.817 / 126.083 / 148.924 |
| MSPT mean, ms | 33.940 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1580.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.93% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 5.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13460` |
| 3 | 2.82% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.53% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12838` |
| 5 | 2.27% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8581` |
| 8 | 1.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.76% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.14% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.06% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13479` |
| 14 | 1.06% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 0.99% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

