# Baseline run: `items-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63018 -> 63019) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.204 / 58.103 / 143.051 / 250.602 |
| MSPT mean, ms | 41.871 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1817.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7064` |
| 2 | 5.76% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 3 | 5.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7817` |
| 4 | 5.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 4.28% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5185` |
| 6 | 3.93% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.68% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.33% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12299` |
| 10 | 2.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7011` |
| 11 | 2.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12449` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6602` |
| 13 | 1.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

