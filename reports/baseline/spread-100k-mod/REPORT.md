# Baseline run: `spread-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97466 -> 97466) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 232.025 / 340.273 / 412.850 / 553.298 |
| MSPT mean, ms | 252.814 |
| TPS estimate (1000/p50) | 4.3 |
| heap used / max, MB | 2521.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.99% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.84% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8474` |
| 5 | 2.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5510` |
| 6 | 2.41% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8615` |
| 7 | 2.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 2.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8421` |
| 9 | 1.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.82% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5365` |
| 13 | 1.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

