# Baseline run: `mixed-150k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 112276 -> 112204) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 87.458 / 136.740 / 275.304 / 426.316 |
| MSPT mean, ms | 102.108 |
| TPS estimate (1000/p50) | 11.4 |
| heap used / max, MB | 2232.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8581` |
| 2 | 10.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7819` |
| 3 | 7.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13598` |
| 4 | 4.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 4.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 3.63% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.69% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5031` |
| 8 | 2.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5291` |
| 9 | 2.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.24% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12838` |
| 11 | 1.44% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.04% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 14 | 1.02% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 0.84% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |

