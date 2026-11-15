# Baseline run: `mobs-25k-native-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25064 -> 25019) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 26.547 / 47.115 / 121.257 / 255.627 |
| MSPT mean, ms | 34.804 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1679.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13538` |
| 2 | 4.24% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.62% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.41% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12913` |
| 5 | 1.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8646` |
| 7 | 1.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 8 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.32% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.15% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 0.88% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 12 | 0.82% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 0.76% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 14 | 0.74% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 0.65% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

