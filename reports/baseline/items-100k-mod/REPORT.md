# Baseline run: `items-100k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62955 -> 62955) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 36.214 / 62.370 / 163.714 / 250.756 |
| MSPT mean, ms | 46.255 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1552.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7533` |
| 2 | 10.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8285` |
| 3 | 5.40% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 4.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7422` |
| 6 | 3.07% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 7 | 2.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.44% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.39% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12805` |
| 10 | 2.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13091` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5216` |
| 12 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 0.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.80% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 0.70% | `java.util.HashMap.getNode(Object) line: 586` |

