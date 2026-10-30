# Baseline run: `spread-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97356 -> 97356) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 287.697 / 444.895 / 542.739 / 674.630 |
| MSPT mean, ms | 316.014 |
| TPS estimate (1000/p50) | 3.5 |
| heap used / max, MB | 1862.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.28% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 2.97% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8282` |
| 4 | 2.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5318` |
| 6 | 2.58% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8423` |
| 7 | 2.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.12% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8229` |
| 9 | 2.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.54% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.51% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5173` |
| 14 | 1.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

