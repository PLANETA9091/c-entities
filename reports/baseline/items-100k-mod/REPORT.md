# Baseline run: `items-100k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63037 -> 63037) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 11.593 / 21.282 / 92.369 / 198.860 |
| MSPT mean, ms | 15.618 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1332.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.75% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 2 | 4.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10331` |
| 4 | 3.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9082` |
| 5 | 2.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 1.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.82% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14965` |
| 8 | 1.61% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1025` |
| 9 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.23% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1022` |
| 11 | 1.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8843` |
| 13 | 0.96% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6600` |
| 14 | 0.84% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8817` |

