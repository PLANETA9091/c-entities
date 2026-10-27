# Baseline run: `mixed-150k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110253 -> 110253) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 307.830 / 398.843 / 534.899 / 628.177 |
| MSPT mean, ms | 325.480 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 1822.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 3 | 3.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 4 | 2.65% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 2.65% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.03% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 7 | 1.94% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.86% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 11 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.68% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4459` |
| 13 | 1.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.32% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 15 | 1.15% | `java.util.EnumMap.get(Object) line: 248` |

