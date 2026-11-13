# Baseline run: `mobs-25k-native-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25051 -> 25043) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 45.469 / 77.241 / 223.696 / 252.263 |
| MSPT mean, ms | 56.349 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1631.8 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.60% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11862` |
| 2 | 5.05% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.68% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 1.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 6 | 1.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7360` |
| 8 | 1.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.38% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.26% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 1.17% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 12 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.75% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 0.69% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11881` |

