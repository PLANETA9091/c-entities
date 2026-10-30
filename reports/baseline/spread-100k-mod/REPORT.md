# Baseline run: `spread-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97429 -> 97429) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 258.915 / 375.582 / 458.613 / 630.615 |
| MSPT mean, ms | 280.997 |
| TPS estimate (1000/p50) | 3.9 |
| heap used / max, MB | 1908.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8558` |
| 2 | 3.21% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8699` |
| 3 | 2.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5594` |
| 4 | 2.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 2.27% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 2.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.01% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8505` |
| 10 | 1.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.66% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5449` |
| 12 | 1.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4803` |
| 14 | 1.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.18% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

