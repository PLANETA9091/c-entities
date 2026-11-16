# Baseline run: `items-100k-mod`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62996 -> 62997) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.952 / 66.589 / 136.561 / 316.987 |
| MSPT mean, ms | 45.550 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1942.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8307` |
| 2 | 7.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 3 | 5.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.36% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 3.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8086` |
| 7 | 2.91% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 8 | 2.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5771` |
| 9 | 2.08% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13922` |
| 10 | 1.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5775` |
| 12 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8104` |
| 14 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7970` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14344` |

