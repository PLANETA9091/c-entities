# Baseline run: `mixed-150k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113916 -> 113868) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 91.617 / 158.843 / 277.061 / 447.189 |
| MSPT mean, ms | 110.074 |
| TPS estimate (1000/p50) | 10.9 |
| heap used / max, MB | 2096.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7255` |
| 2 | 11.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8008` |
| 3 | 7.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12640` |
| 4 | 5.04% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 4.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 4.46% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.65% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.80% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4904` |
| 9 | 2.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.09% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5320` |
| 11 | 1.62% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.42% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5070` |
| 14 | 1.30% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.01% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |

