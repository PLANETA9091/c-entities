# Baseline run: `spread-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97439 -> 97439) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 369.652 / 507.226 / 694.873 / 857.680 |
| MSPT mean, ms | 395.423 |
| TPS estimate (1000/p50) | 2.7 |
| heap used / max, MB | 3200.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.96% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 4.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3401` |
| 3 | 3.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.80% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1331` |
| 5 | 2.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10970` |
| 6 | 1.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7107` |
| 7 | 1.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3345` |
| 8 | 1.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.45% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3359` |
| 14 | 1.18% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 15 | 1.06% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

