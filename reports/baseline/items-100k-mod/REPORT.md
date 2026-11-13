# Baseline run: `items-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 50678 -> 50677) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 23.578 / 35.849 / 103.808 / 216.864 |
| MSPT mean, ms | 29.673 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1551.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.85% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 5.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11991` |
| 3 | 5.37% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4900` |
| 4 | 3.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7360` |
| 6 | 3.33% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6477` |
| 9 | 2.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6299` |
| 10 | 2.14% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.68% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11841` |
| 12 | 1.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6303` |
| 13 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.94% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

