# Baseline run: `items-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 50652 -> 50652) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.856 / 45.364 / 124.367 / 241.785 |
| MSPT mean, ms | 37.325 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1451.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.11% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 2 | 5.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11991` |
| 3 | 5.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.79% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4900` |
| 5 | 3.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7360` |
| 6 | 3.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 2.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.97% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.72% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6299` |
| 11 | 1.44% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11841` |
| 12 | 1.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6621` |
| 15 | 1.06% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |

