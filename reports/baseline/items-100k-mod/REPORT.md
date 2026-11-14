# Baseline run: `items-100k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62969 -> 62969) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 33.287 / 53.225 / 125.574 / 267.482 |
| MSPT mean, ms | 40.775 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1601.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7538` |
| 2 | 5.06% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 4.04% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int, int) line: 5585` |
| 4 | 3.67% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 2.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7434` |
| 8 | 2.66% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12933` |
| 9 | 2.32% | `dev.planeta9091.centities.CeHooks.rmDead(int, long) line: 2557` |
| 10 | 2.03% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8292` |
| 12 | 1.68% | `dev.planeta9091.centities.CeHooks.vec3CachedS(int, double, double, double) line: 2486` |
| 13 | 1.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7438` |
| 14 | 1.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8302` |
| 15 | 1.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12804` |

