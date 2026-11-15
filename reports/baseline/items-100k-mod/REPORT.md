# Baseline run: `items-100k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62900 -> 62900) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 31.766 / 52.823 / 143.918 / 218.490 |
| MSPT mean, ms | 40.850 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1714.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7583` |
| 2 | 7.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8335` |
| 3 | 5.05% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.46% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13141` |
| 8 | 2.86% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 9 | 2.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7454` |
| 10 | 2.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5234` |
| 12 | 1.48% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12855` |
| 13 | 1.01% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7406` |

