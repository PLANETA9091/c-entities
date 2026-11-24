# Baseline run: `mobs-25k-native-mod`

Server: `26.3-262-827eeeb (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25052 -> 25037) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 20.698 / 39.222 / 115.845 / 188.624 |
| MSPT mean, ms | 28.505 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1184.8 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.80% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16241` |
| 2 | 5.69% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.61% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 5 | 1.99% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.83% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15209` |
| 8 | 1.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10725` |
| 9 | 1.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.40% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16521` |
| 12 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.21% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 0.93% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |

