# Baseline run: `spread-100k-mod`

Server: `26.3-235-bb10cf0 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97383 -> 97383) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 264.672 / 397.292 / 460.263 / 705.339 |
| MSPT mean, ms | 287.754 |
| TPS estimate (1000/p50) | 3.8 |
| heap used / max, MB | 1969.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.64% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.72% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8965` |
| 3 | 3.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 4 | 2.97% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5860` |
| 6 | 2.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 2.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.96% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8771` |
| 10 | 1.71% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.65% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5715` |
| 13 | 1.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.23% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.21% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

