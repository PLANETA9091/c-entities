# Baseline run: `items-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63010 -> 63010) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 30.265 / 46.347 / 112.489 / 235.875 |
| MSPT mean, ms | 38.402 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1536.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 2 | 7.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 3 | 5.44% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 4.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6309` |
| 5 | 4.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 2.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.68% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4729` |
| 9 | 2.62% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6380` |
| 11 | 1.76% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.20% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10785` |
| 13 | 1.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4597` |
| 14 | 1.06% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 15 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

