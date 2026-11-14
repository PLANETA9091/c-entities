# Baseline run: `items-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62998 -> 62998) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 25.048 / 41.281 / 102.427 / 262.141 |
| MSPT mean, ms | 31.620 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1630.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 16.60% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 8.57% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5141` |
| 3 | 6.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7008` |
| 4 | 4.27% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 2.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12392` |
| 8 | 2.08% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 2.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |
| 10 | 1.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 11 | 1.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6553` |
| 12 | 1.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7770` |
| 13 | 1.50% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 1.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6911` |
| 15 | 1.36% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12242` |

