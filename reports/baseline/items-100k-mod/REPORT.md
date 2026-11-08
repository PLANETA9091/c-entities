# Baseline run: `items-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62997 -> 62997) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.062 / 49.414 / 117.534 / 223.687 |
| MSPT mean, ms | 39.764 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1475.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10970` |
| 2 | 5.72% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 5.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7107` |
| 4 | 4.83% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 4.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6344` |
| 6 | 3.18% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4764` |
| 7 | 3.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.78% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6415` |
| 11 | 1.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4632` |
| 12 | 1.45% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10820` |
| 13 | 1.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10832` |

