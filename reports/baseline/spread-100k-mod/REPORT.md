# Baseline run: `spread-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88799 -> 88672) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 167.549 / 253.986 / 378.365 / 460.811 |
| MSPT mean, ms | 185.575 |
| TPS estimate (1000/p50) | 6.0 |
| heap used / max, MB | 2753.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.93% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 6.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3654` |
| 3 | 3.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |
| 4 | 3.13% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 2.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3598` |
| 7 | 2.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3628` |
| 8 | 2.54% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12392` |
| 9 | 1.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.78% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3615` |
| 12 | 1.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.49% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 14 | 1.40% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 15 | 1.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3627` |

