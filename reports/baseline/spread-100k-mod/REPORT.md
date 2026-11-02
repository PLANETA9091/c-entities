# Baseline run: `spread-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97439 -> 97439) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 274.529 / 426.692 / 558.549 / 758.678 |
| MSPT mean, ms | 303.295 |
| TPS estimate (1000/p50) | 3.6 |
| heap used / max, MB | 2345.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.44% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9116` |
| 3 | 2.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.86% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.71% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9257` |
| 6 | 2.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6139` |
| 7 | 2.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.01% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9063` |
| 9 | 1.72% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5985` |
| 10 | 1.57% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5403` |
| 15 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

