# Baseline run: `mixed-150k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113933 -> 113898) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 108.842 / 200.003 / 365.071 / 655.870 |
| MSPT mean, ms | 132.427 |
| TPS estimate (1000/p50) | 9.2 |
| heap used / max, MB | 2062.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8292` |
| 2 | 10.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7538` |
| 3 | 5.48% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 5.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12804` |
| 5 | 3.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.10% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.54% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5147` |
| 8 | 2.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12933` |
| 9 | 2.46% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int, int) line: 5585` |
| 10 | 1.93% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5326` |
| 13 | 1.27% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 14 | 1.22% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.19% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |

