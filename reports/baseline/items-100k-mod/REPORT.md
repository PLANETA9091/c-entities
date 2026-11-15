# Baseline run: `items-100k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62943 -> 62943) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.603 / 62.130 / 125.554 / 201.948 |
| MSPT mean, ms | 42.186 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1609.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7583` |
| 2 | 4.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7454` |
| 3 | 4.20% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 4.18% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 8 | 2.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13141` |
| 9 | 1.82% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12855` |
| 10 | 1.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8335` |
| 11 | 1.20% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 12 | 1.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7406` |

