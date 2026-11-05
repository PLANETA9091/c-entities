# Baseline run: `mobs-25k-mod`

Server: `26.3-242-fd28d65 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25062 -> 25062) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 232.515 / 397.533 / 523.343 / 632.998 |
| MSPT mean, ms | 263.750 |
| TPS estimate (1000/p50) | 4.3 |
| heap used / max, MB | 1942.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 4.18% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 3 | 3.03% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 4 | 2.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.12% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 2.07% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 2.00% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 8 | 1.68% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 9 | 1.60% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 10 | 1.55% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.51% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 12 | 1.36% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7291` |
| 13 | 1.25% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 1.09% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4834` |

