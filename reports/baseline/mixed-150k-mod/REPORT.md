# Baseline run: `mixed-150k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113934 -> 113894) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 88.514 / 150.587 / 306.685 / 388.239 |
| MSPT mean, ms | 106.300 |
| TPS estimate (1000/p50) | 11.3 |
| heap used / max, MB | 2209.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8218` |
| 2 | 10.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7465` |
| 3 | 7.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13024` |
| 4 | 5.11% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 4.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 4.11% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.92% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4986` |
| 9 | 2.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5199` |
| 11 | 1.57% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.37% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.23% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 14 | 1.05% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 0.96% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |

