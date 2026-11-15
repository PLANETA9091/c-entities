# Baseline run: `mixed-150k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113884 -> 113849) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 85.795 / 150.180 / 273.767 / 450.920 |
| MSPT mean, ms | 103.451 |
| TPS estimate (1000/p50) | 11.7 |
| heap used / max, MB | 2160.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7547` |
| 2 | 9.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8299` |
| 3 | 7.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13105` |
| 4 | 5.10% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 4.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.95% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 3.84% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 2.70% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 9 | 2.31% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 11 | 1.77% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.52% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.40% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 1.03% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

