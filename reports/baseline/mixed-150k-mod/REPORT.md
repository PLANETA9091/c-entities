# Baseline run: `mixed-150k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113945 -> 113912) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 90.555 / 138.749 / 337.411 / 423.020 |
| MSPT mean, ms | 107.678 |
| TPS estimate (1000/p50) | 11.0 |
| heap used / max, MB | 2232.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8335` |
| 2 | 10.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7583` |
| 3 | 7.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13141` |
| 4 | 5.26% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 5.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 4.44% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.76% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 9 | 2.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.85% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 11 | 1.71% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.45% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.40% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 14 | 1.13% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 0.97% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |

