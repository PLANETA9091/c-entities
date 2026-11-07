# Baseline run: `items-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63009 -> 63009) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 28.309 / 42.517 / 127.385 / 335.550 |
| MSPT mean, ms | 35.813 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1579.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.76% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 7.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 3 | 5.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 4 | 4.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 4.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 4.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6309` |
| 7 | 3.95% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 3.36% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4729` |
| 9 | 2.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4597` |
| 10 | 1.93% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.81% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6380` |
| 13 | 1.21% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10785` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6236` |
| 15 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

