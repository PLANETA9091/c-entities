# Baseline run: `items-100k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62940 -> 62941) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.275 / 59.886 / 128.879 / 288.245 |
| MSPT mean, ms | 41.510 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1792.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7569` |
| 2 | 8.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8321` |
| 3 | 5.46% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 5.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 2.66% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.12% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 8 | 2.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7440` |
| 9 | 1.92% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12841` |
| 10 | 1.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 11 | 1.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13127` |
| 12 | 1.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5234` |
| 13 | 1.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.92% | `java.util.Objects.checkIndex(int, int) line: 365` |

