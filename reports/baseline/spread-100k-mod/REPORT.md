# Baseline run: `spread-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97332 -> 97332) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 299.264 / 470.850 / 591.360 / 805.765 |
| MSPT mean, ms | 329.485 |
| TPS estimate (1000/p50) | 3.3 |
| heap used / max, MB | 1935.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.40% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8657` |
| 2 | 3.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8516` |
| 4 | 3.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5552` |
| 6 | 2.04% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.76% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.67% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8463` |
| 9 | 1.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4761` |
| 11 | 1.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.11% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5407` |

