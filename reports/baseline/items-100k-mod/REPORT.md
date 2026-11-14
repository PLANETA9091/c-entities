# Baseline run: `items-100k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62972 -> 62971) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 38.611 / 75.846 / 165.594 / 411.835 |
| MSPT mean, ms | 49.968 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1741.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 16.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7407` |
| 2 | 5.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8160` |
| 3 | 5.42% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 4.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 4.24% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5413` |
| 6 | 3.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7347` |
| 9 | 2.63% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 2.34% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12680` |
| 11 | 2.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12966` |
| 12 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7303` |
| 13 | 1.02% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |

