# Baseline run: `spread-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97355 -> 97355) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 308.387 / 476.792 / 567.259 / 741.058 |
| MSPT mean, ms | 336.518 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 2016.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.83% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.59% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8729` |
| 3 | 3.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8588` |
| 4 | 3.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.47% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5624` |
| 8 | 2.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.78% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8535` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4833` |
| 11 | 1.30% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5479` |
| 13 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

