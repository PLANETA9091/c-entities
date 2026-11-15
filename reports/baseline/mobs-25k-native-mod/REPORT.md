# Baseline run: `mobs-25k-native-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25077 -> 25065) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 24.967 / 42.511 / 111.367 / 192.332 |
| MSPT mean, ms | 31.760 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1744.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.98% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 4.92% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12989` |
| 3 | 4.07% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8321` |
| 5 | 2.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 2.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.80% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.80% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 9 | 1.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.06% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 12 | 1.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.99% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 0.99% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 0.92% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |

