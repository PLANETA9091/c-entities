# Baseline run: `items-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 50666 -> 50664) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 28.089 / 42.273 / 96.802 / 177.650 |
| MSPT mean, ms | 34.029 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1396.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.14% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 5.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6651` |
| 3 | 4.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11978` |
| 4 | 3.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 3.53% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4887` |
| 6 | 2.76% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7357` |
| 8 | 2.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.73% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 10 | 1.71% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.69% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6608` |
| 12 | 1.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 13 | 1.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7347` |
| 15 | 1.16% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11828` |

