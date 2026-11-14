# Baseline run: `items-100k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62915 -> 62915) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 20.376 / 37.140 / 91.471 / 139.407 |
| MSPT mean, ms | 26.009 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1713.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7255` |
| 2 | 3.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8018` |
| 3 | 3.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8008` |
| 4 | 2.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7195` |
| 5 | 2.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12640` |
| 6 | 2.21% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.18% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.62% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5320` |
| 11 | 1.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5329` |
| 13 | 1.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5070` |
| 14 | 1.09% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

