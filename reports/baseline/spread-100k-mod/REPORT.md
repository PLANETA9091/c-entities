# Baseline run: `spread-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97376 -> 97376) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 268.209 / 406.163 / 514.204 / 676.416 |
| MSPT mean, ms | 294.442 |
| TPS estimate (1000/p50) | 3.7 |
| heap used / max, MB | 1826.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.12% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.76% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.09% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9345` |
| 4 | 2.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9204` |
| 5 | 2.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6163` |
| 7 | 2.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.12% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.02% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9118` |
| 10 | 1.88% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.84% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.82% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 6009` |
| 14 | 1.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

