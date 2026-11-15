# Baseline run: `items-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62944 -> 62944) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.052 / 56.835 / 121.366 / 227.728 |
| MSPT mean, ms | 41.830 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1612.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7923` |
| 2 | 6.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7743` |
| 4 | 3.96% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.31% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 2.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5394` |
| 7 | 2.25% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13506` |
| 8 | 2.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8734` |
| 9 | 1.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5398` |
| 10 | 1.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13928` |
| 11 | 1.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.34% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.99% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 0.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7727` |

