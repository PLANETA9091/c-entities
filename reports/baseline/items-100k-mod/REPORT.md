# Baseline run: `items-100k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62984 -> 62984) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 23.095 / 39.332 / 98.695 / 265.298 |
| MSPT mean, ms | 29.628 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 2053.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 17.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7471` |
| 2 | 7.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5203` |
| 3 | 7.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5199` |
| 4 | 4.55% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7411` |
| 6 | 3.24% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13030` |
| 8 | 2.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 1.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8224` |
| 10 | 1.62% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 11 | 1.49% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7367` |
| 13 | 1.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5205` |
| 15 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

