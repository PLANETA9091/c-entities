# Baseline run: `spread-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97429 -> 97429) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 273.334 / 418.532 / 513.500 / 685.454 |
| MSPT mean, ms | 296.653 |
| TPS estimate (1000/p50) | 3.7 |
| heap used / max, MB | 3017.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5517` |
| 2 | 2.94% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8481` |
| 5 | 2.74% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.57% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8622` |
| 7 | 2.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.80% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8428` |
| 10 | 1.60% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5372` |
| 11 | 1.58% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4726` |
| 15 | 1.21% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

