# Baseline run: `mixed-150k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109748 -> 109748) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 243.951 / 352.005 / 473.768 / 646.516 |
| MSPT mean, ms | 267.291 |
| TPS estimate (1000/p50) | 4.1 |
| heap used / max, MB | 2095.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.47% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8383` |
| 2 | 4.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.21% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 3.03% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8242` |
| 6 | 2.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5278` |
| 7 | 2.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8189` |
| 10 | 2.14% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5133` |
| 11 | 1.92% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |

