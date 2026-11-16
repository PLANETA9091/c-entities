# Baseline run: `items-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62922 -> 62922) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 25.073 / 40.206 / 106.283 / 236.707 |
| MSPT mean, ms | 31.362 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1687.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8055` |
| 2 | 5.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.26% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5523` |
| 5 | 2.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8898` |
| 6 | 2.59% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5519` |
| 8 | 2.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7834` |
| 9 | 1.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7852` |
| 11 | 1.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13818` |
| 12 | 1.38% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13670` |
| 13 | 1.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.04% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 15 | 0.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7718` |

