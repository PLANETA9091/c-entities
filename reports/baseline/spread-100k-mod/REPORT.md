# Baseline run: `spread-100k-mod`

Server: `26.3-236-f119026 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97443 -> 97443) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 254.803 / 382.266 / 509.483 / 614.288 |
| MSPT mean, ms | 279.962 |
| TPS estimate (1000/p50) | 3.9 |
| heap used / max, MB | 2801.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.68% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 2.70% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9061` |
| 4 | 2.67% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8920` |
| 6 | 2.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5943` |
| 7 | 2.15% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.90% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8867` |
| 9 | 1.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.79% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.77% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.77% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5789` |
| 14 | 1.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.09% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

