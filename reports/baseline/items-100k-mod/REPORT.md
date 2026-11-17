# Baseline run: `items-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62958 -> 62958) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 11.581 / 20.513 / 87.335 / 308.078 |
| MSPT mean, ms | 15.926 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1684.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8660` |
| 3 | 4.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9503` |
| 4 | 3.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.38% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7701` |
| 6 | 2.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.99% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7688` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7703` |
| 9 | 1.44% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7697` |
| 10 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14509` |
| 12 | 1.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8446` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13781` |
| 14 | 0.90% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 15 | 0.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14782` |

