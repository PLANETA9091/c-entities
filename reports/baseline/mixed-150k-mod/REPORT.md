# Baseline run: `mixed-150k-mod`

Server: `26.3-251-da964e3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113908 -> 113851) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 73.618 / 137.698 / 224.990 / 319.914 |
| MSPT mean, ms | 88.640 |
| TPS estimate (1000/p50) | 13.6 |
| heap used / max, MB | 2410.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12966` |
| 2 | 7.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7407` |
| 3 | 6.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8160` |
| 4 | 6.66% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 4.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.37% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5413` |
| 8 | 3.37% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 2.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.41% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4981` |
| 11 | 1.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.52% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.42% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.31% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.25% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |

