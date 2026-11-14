# Baseline run: `spread-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88831 -> 88719) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 139.564 / 194.604 / 325.821 / 369.491 |
| MSPT mean, ms | 154.735 |
| TPS estimate (1000/p50) | 7.2 |
| heap used / max, MB | 2851.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3654` |
| 2 | 4.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 4.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.82% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3598` |
| 5 | 2.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7008` |
| 6 | 2.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12392` |
| 7 | 2.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3615` |
| 8 | 2.13% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3612` |
| 9 | 2.06% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3618` |
| 10 | 1.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3628` |
| 11 | 1.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.75% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3627` |
| 15 | 1.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |

