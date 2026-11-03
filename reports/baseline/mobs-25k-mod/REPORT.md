# Baseline run: `mobs-25k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25059 -> 25059) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 300.322 / 544.406 / 676.805 / 753.981 |
| MSPT mean, ms | 349.426 |
| TPS estimate (1000/p50) | 3.3 |
| heap used / max, MB | 1851.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.95% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 2 | 1.93% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10235` |
| 3 | 1.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.49% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.26% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9633` |
| 10 | 1.19% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 11 | 1.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.06% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.01% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 0.97% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6854` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6922` |

