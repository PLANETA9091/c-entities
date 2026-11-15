# Baseline run: `items-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62978 -> 62978) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.340 / 60.356 / 129.741 / 286.082 |
| MSPT mean, ms | 41.514 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1742.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 15.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7936` |
| 2 | 4.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5374` |
| 4 | 3.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8698` |
| 6 | 2.57% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.99% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 8 | 1.85% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13470` |
| 9 | 1.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5378` |
| 10 | 1.20% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12962` |
| 11 | 1.14% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.02% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5380` |
| 14 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7696` |

