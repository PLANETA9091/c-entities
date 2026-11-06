# Baseline run: `mobs-25k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25060 -> 25060) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 350.345 / 556.116 / 717.856 / 804.266 |
| MSPT mean, ms | 391.246 |
| TPS estimate (1000/p50) | 2.9 |
| heap used / max, MB | 1996.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2747` |
| 2 | 4.47% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.06% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 4 | 2.82% | `net.minecraft.world.entity.Entity.moonrise$getSectionY() line: 453` |
| 5 | 1.68% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 11144` |
| 6 | 1.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.44% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2450` |
| 8 | 1.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10508` |
| 11 | 1.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 0.94% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 0.88% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7403` |
| 14 | 0.83% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 0.83% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |

