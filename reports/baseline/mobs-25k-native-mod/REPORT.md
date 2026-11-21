# Baseline run: `mobs-25k-native-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25083 -> 25073) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 23.096 / 45.277 / 121.134 / 198.169 |
| MSPT mean, ms | 31.877 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1231.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.96% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15622` |
| 2 | 4.25% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.58% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 4 | 2.26% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.17% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14742` |
| 6 | 1.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10286` |
| 7 | 1.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 8 | 1.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.28% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 10 | 1.16% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 1.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.92% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 0.83% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 0.77% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 15 | 0.77% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |

