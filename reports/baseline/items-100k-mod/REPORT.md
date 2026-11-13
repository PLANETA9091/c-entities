# Baseline run: `items-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62876 -> 62876) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.355 / 53.696 / 124.949 / 276.307 |
| MSPT mean, ms | 38.132 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1742.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 15.10% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 5.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.86% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4916` |
| 4 | 4.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6728` |
| 5 | 4.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7473` |
| 8 | 2.77% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 2.33% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 10 | 1.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6315` |
| 11 | 1.57% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11955` |
| 12 | 1.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6657` |
| 13 | 1.24% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12105` |
| 15 | 0.96% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |

