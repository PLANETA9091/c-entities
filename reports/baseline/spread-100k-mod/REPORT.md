# Baseline run: `spread-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97377 -> 97377) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 280.885 / 436.553 / 573.075 / 763.040 |
| MSPT mean, ms | 312.271 |
| TPS estimate (1000/p50) | 3.6 |
| heap used / max, MB | 2106.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.63% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8242` |
| 3 | 3.07% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8383` |
| 4 | 2.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5278` |
| 8 | 1.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.65% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8189` |
| 10 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4560` |
| 11 | 1.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.28% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.22% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.20% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.16% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5133` |

