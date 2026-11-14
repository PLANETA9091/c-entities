# Baseline run: `items-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63004 -> 63004) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 30.864 / 56.295 / 126.073 / 217.255 |
| MSPT mean, ms | 39.216 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1724.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 15.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7056` |
| 2 | 5.21% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 5.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7808` |
| 4 | 4.82% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5185` |
| 5 | 4.51% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 4.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 2.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.83% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 9 | 1.74% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12290` |
| 10 | 1.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12440` |
| 11 | 1.14% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6597` |
| 13 | 1.10% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

