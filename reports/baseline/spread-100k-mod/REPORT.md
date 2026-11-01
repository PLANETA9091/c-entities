# Baseline run: `spread-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97351 -> 97351) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 308.289 / 457.211 / 608.125 / 734.895 |
| MSPT mean, ms | 337.083 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 2208.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.86% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9129` |
| 2 | 3.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8988` |
| 3 | 2.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 2.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.52% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6011` |
| 8 | 1.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.61% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8935` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5275` |
| 11 | 1.26% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.22% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5857` |
| 14 | 1.14% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.96% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

