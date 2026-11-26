# Baseline run: `mobs-25k-native-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25062 -> 25054) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 21.911 / 41.517 / 115.130 / 130.657 |
| MSPT mean, ms | 29.191 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1270.8 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.81% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16558` |
| 2 | 4.56% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13912` |
| 3 | 2.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 1.89% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 5 | 1.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10758` |
| 6 | 1.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 7 | 1.54% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15288` |
| 8 | 1.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.26% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 0.88% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 11 | 0.88% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 0.79% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 13 | 0.76% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.72% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 0.69% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |

