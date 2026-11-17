# Baseline run: `mobs-25k-native-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25052 -> 25030) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 26.235 / 63.885 / 163.046 / 326.167 |
| MSPT mean, ms | 38.291 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1781.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.47% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 2 | 4.42% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 4.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14420` |
| 4 | 3.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9414` |
| 5 | 2.96% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13695` |
| 6 | 2.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 8 | 1.17% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 9 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.12% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 0.99% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 0.82% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 13 | 0.80% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 0.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14439` |

