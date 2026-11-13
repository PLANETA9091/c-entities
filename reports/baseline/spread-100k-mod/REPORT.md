# Baseline run: `spread-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88841 -> 88720) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 173.986 / 269.383 / 410.160 / 447.785 |
| MSPT mean, ms | 196.235 |
| TPS estimate (1000/p50) | 5.7 |
| heap used / max, MB | 2904.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.31% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 5.57% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3490` |
| 3 | 3.82% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3434` |
| 4 | 3.22% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 2.96% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7473` |
| 6 | 2.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3463` |
| 7 | 2.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12105` |
| 8 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 2.05% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3448` |
| 10 | 1.88% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.67% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3451` |
| 12 | 1.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3464` |
| 14 | 1.50% | `java.util.ArrayList.<init>(int) line: 157` |
| 15 | 1.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

