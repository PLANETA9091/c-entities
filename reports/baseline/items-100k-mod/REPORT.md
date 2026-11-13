# Baseline run: `items-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 50698 -> 50697) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 22.605 / 33.842 / 102.576 / 174.589 |
| MSPT mean, ms | 28.219 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1743.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 15.17% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 6.59% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4886` |
| 3 | 5.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11977` |
| 4 | 4.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7346` |
| 6 | 2.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6463` |
| 8 | 2.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 1.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6285` |
| 10 | 1.63% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7356` |
| 12 | 1.40% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11827` |
| 13 | 1.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6289` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

