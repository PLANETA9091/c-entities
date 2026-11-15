# Baseline run: `mobs-25k-native-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25063 -> 25057) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 18.895 / 34.736 / 108.749 / 235.850 |
| MSPT mean, ms | 26.150 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1554.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.89% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13654` |
| 2 | 4.30% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.33% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 1.97% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8734` |
| 7 | 1.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.65% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 10 | 1.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.60% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13001` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13673` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13932` |
| 14 | 1.05% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 1.01% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |

