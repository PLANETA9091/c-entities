# Baseline run: `items-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 50667 -> 50665) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.719 / 51.798 / 128.836 / 237.298 |
| MSPT mean, ms | 40.684 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1500.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 16.12% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 7.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7401` |
| 3 | 4.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6665` |
| 5 | 3.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.42% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.81% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11883` |
| 8 | 2.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12033` |
| 9 | 2.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.75% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4901` |
| 11 | 1.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.29% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6300` |

