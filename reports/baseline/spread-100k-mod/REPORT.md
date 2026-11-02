# Baseline run: `spread-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97468 -> 97468) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 322.319 / 523.931 / 644.281 / 880.584 |
| MSPT mean, ms | 358.590 |
| TPS estimate (1000/p50) | 3.1 |
| heap used / max, MB | 1921.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.12% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.16% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9267` |
| 3 | 3.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9126` |
| 4 | 3.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 7 | 1.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.68% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9073` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5413` |
| 10 | 1.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.21% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5995` |
| 12 | 1.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.16% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.00% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

