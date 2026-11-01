# Baseline run: `spread-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97408 -> 97408) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 246.056 / 353.366 / 449.479 / 542.145 |
| MSPT mean, ms | 265.848 |
| TPS estimate (1000/p50) | 4.1 |
| heap used / max, MB | 2242.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.84% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.59% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9080` |
| 3 | 3.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8939` |
| 5 | 2.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5962` |
| 6 | 2.57% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 2.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.03% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8886` |
| 11 | 1.86% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.82% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5808` |
| 14 | 1.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

