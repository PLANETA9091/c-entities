# Baseline run: `items-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 50670 -> 50666) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 33.218 / 51.264 / 130.816 / 245.314 |
| MSPT mean, ms | 40.287 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1547.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 4.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7416` |
| 3 | 4.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6680` |
| 4 | 4.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.84% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4916` |
| 6 | 3.11% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12048` |
| 9 | 2.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 2.18% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11898` |
| 11 | 1.81% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 12 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6315` |
| 13 | 1.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6637` |
| 15 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

