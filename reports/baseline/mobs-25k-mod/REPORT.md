# Baseline run: `mobs-25k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25062 -> 25062) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 435.719 / 655.104 / 788.729 / 895.640 |
| MSPT mean, ms | 467.408 |
| TPS estimate (1000/p50) | 2.3 |
| heap used / max, MB | 1833.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 2 | 2.50% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 4 | 2.15% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 5 | 2.15% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 6 | 2.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.80% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.30% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.27% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5129` |
| 11 | 1.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10841` |
| 12 | 1.23% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7705` |
| 14 | 1.20% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7627` |
| 15 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |

