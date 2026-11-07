# Baseline run: `items-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62962 -> 62962) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 28.149 / 46.164 / 112.306 / 176.948 |
| MSPT mean, ms | 36.498 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1385.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6967` |
| 2 | 6.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10830` |
| 3 | 5.63% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 3.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6204` |
| 6 | 3.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 2.74% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4624` |
| 8 | 2.32% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 2.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6275` |
| 11 | 1.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4492` |
| 13 | 1.15% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6131` |
| 15 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

