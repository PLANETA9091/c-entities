# Baseline run: `spread-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97416 -> 97416) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 279.888 / 456.158 / 661.755 / 796.675 |
| MSPT mean, ms | 315.250 |
| TPS estimate (1000/p50) | 3.6 |
| heap used / max, MB | 1998.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.79% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.88% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8837` |
| 3 | 3.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5732` |
| 5 | 2.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8696` |
| 6 | 2.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.22% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8643` |
| 9 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.62% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.57% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 12 | 1.52% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5587` |
| 13 | 1.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4941` |
| 14 | 1.38% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

