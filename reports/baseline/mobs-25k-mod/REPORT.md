# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25068 -> 25068) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 204.732 / 325.198 / 457.625 / 523.098 |
| MSPT mean, ms | 231.137 |
| TPS estimate (1000/p50) | 4.9 |
| heap used / max, MB | 1393.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 2 | 2.97% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 2.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 5 | 2.13% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 6 | 1.97% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.87% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10216` |
| 10 | 1.67% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 11 | 1.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6903` |
| 12 | 1.54% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6835` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4443` |
| 14 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 1.22% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4380` |

