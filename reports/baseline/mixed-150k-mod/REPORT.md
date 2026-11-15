# Baseline run: `mixed-150k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113994 -> 113969) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 97.051 / 174.997 / 330.570 / 434.658 |
| MSPT mean, ms | 116.844 |
| TPS estimate (1000/p50) | 10.3 |
| heap used / max, MB | 2038.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8285` |
| 2 | 9.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7533` |
| 3 | 6.93% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13091` |
| 4 | 4.78% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 4.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.64% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.54% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 8 | 2.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5216` |
| 11 | 1.47% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.26% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.24% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 14 | 1.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7422` |
| 15 | 1.08% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |

