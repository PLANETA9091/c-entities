# Baseline run: `mixed-150k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 113869 -> 113810) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 103.070 / 179.553 / 341.541 / 429.627 |
| MSPT mean, ms | 123.136 |
| TPS estimate (1000/p50) | 9.7 |
| heap used / max, MB | 2125.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7533` |
| 2 | 7.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8285` |
| 3 | 7.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 6.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13091` |
| 5 | 5.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.98% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.72% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.99% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 10 | 1.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.65% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.63% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7422` |
| 14 | 1.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5216` |
| 15 | 1.17% | `net.minecraft.world.entity.Entity.getType() line: 633` |

