# Baseline run: `items-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62930 -> 62929) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 30.173 / 48.914 / 112.884 / 185.493 |
| MSPT mean, ms | 38.002 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1584.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10970` |
| 2 | 6.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7107` |
| 3 | 5.34% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 4.96% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 4.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6344` |
| 6 | 3.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.07% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4764` |
| 8 | 2.73% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 2.38% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 11 | 1.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4632` |
| 12 | 1.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6415` |
| 13 | 1.37% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 1.35% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10820` |
| 15 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

