# Baseline run: `items-100k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62895 -> 62894) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.544 / 55.879 / 160.970 / 342.987 |
| MSPT mean, ms | 42.618 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1911.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 17.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7533` |
| 2 | 6.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8285` |
| 3 | 4.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 3.76% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 3.59% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5216` |
| 8 | 3.44% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 9 | 2.19% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12805` |
| 10 | 1.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13091` |
| 11 | 1.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5220` |
| 12 | 1.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7378` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5222` |
| 15 | 1.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |

