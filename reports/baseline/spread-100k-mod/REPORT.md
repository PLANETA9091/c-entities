# Baseline run: `spread-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97423 -> 97423) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 161.212 / 269.312 / 384.405 / 454.927 |
| MSPT mean, ms | 183.625 |
| TPS estimate (1000/p50) | 6.2 |
| heap used / max, MB | 2604.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.30% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.99% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8056` |
| 3 | 2.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5092` |
| 4 | 2.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.76% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.62% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8197` |
| 7 | 2.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8003` |
| 9 | 2.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.96% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4947` |
| 11 | 1.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.68% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.48% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4374` |

