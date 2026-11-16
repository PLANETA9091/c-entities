# Baseline run: `items-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63045 -> 63046) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 35.234 / 62.938 / 125.766 / 317.916 |
| MSPT mean, ms | 43.981 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1673.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8055` |
| 2 | 6.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8182` |
| 4 | 4.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.80% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 2.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5519` |
| 7 | 2.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8898` |
| 8 | 2.04% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13670` |
| 9 | 1.82% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5523` |
| 10 | 1.61% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 11 | 1.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14092` |
| 13 | 1.24% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13162` |
| 14 | 1.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5525` |
| 15 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

