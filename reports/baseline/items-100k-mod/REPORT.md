# Baseline run: `items-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 50658 -> 50656) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 28.456 / 45.386 / 111.418 / 188.418 |
| MSPT mean, ms | 35.850 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1684.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.76% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 4.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11977` |
| 3 | 4.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7346` |
| 4 | 4.09% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4886` |
| 5 | 3.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.20% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.14% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6650` |
| 9 | 1.76% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 10 | 1.60% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.27% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11827` |
| 12 | 1.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6607` |
| 15 | 1.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

