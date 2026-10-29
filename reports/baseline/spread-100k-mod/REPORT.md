# Baseline run: `spread-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97454 -> 97454) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 163.853 / 279.426 / 432.840 / 536.961 |
| MSPT mean, ms | 188.379 |
| TPS estimate (1000/p50) | 6.1 |
| heap used / max, MB | 1905.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.15% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5100` |
| 3 | 2.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8064` |
| 4 | 2.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.60% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.42% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8205` |
| 9 | 2.41% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8011` |
| 10 | 1.97% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4955` |
| 11 | 1.94% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.56% | `java.util.ImmutableCollections$ListItr.next() line: 397` |

