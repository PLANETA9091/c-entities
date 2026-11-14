# Baseline run: `items-100k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63025 -> 63024) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 22.476 / 38.673 / 100.293 / 193.592 |
| MSPT mean, ms | 29.091 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1686.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 16.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7189` |
| 2 | 8.76% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5297` |
| 3 | 4.15% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7096` |
| 5 | 3.32% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 6 | 2.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12574` |
| 8 | 2.12% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.98% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7942` |
| 11 | 1.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7952` |
| 12 | 1.54% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6714` |
| 14 | 1.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7092` |
| 15 | 1.20% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12424` |

