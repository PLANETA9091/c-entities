# Baseline run: `mobs-25k-native-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25056 -> 25038) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 19.740 / 40.089 / 109.805 / 220.681 |
| MSPT mean, ms | 28.026 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1286.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.81% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 3.16% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 3 | 3.16% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15343` |
| 4 | 2.61% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14477` |
| 5 | 2.21% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.09% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 7 | 2.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10127` |
| 8 | 1.58% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.50% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 11 | 1.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.22% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.19% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 995` |
| 14 | 0.99% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 15 | 0.75% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |

