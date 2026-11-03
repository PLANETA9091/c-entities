# Baseline run: `mobs-25k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25047 -> 25047) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 192.170 / 312.728 / 450.779 / 574.476 |
| MSPT mean, ms | 220.140 |
| TPS estimate (1000/p50) | 5.2 |
| heap used / max, MB | 1718.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 2 | 3.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 3 | 2.77% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 4 | 2.72% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 5 | 2.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.40% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10361` |
| 7 | 2.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.99% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 10 | 1.87% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6980` |
| 11 | 1.58% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4569` |
| 12 | 1.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9759` |
| 15 | 1.26% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4506` |

