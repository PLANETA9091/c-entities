# Baseline run: `mobs-25k-native-mod`

Server: `26.3-255-c22378f (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25071 -> 25064) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 26.162 / 56.330 / 133.425 / 236.649 |
| MSPT mean, ms | 36.311 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1675.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.08% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 3.14% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 3 | 2.82% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14975` |
| 4 | 2.17% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14210` |
| 5 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9860` |
| 7 | 1.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 8 | 1.57% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 9 | 1.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.31% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 11 | 1.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.00% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 0.91% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 15013` |
| 15 | 0.91% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |

