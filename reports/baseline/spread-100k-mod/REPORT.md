# Baseline run: `spread-100k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97475 -> 97475) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 241.594 / 385.871 / 524.862 / 671.556 |
| MSPT mean, ms | 272.342 |
| TPS estimate (1000/p50) | 4.1 |
| heap used / max, MB | 2690.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.91% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9149` |
| 2 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.93% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9008` |
| 4 | 2.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.68% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6031` |
| 7 | 2.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.08% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8955` |
| 9 | 2.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.79% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5877` |
| 11 | 1.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.10% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |

