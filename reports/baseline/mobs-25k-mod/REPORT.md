# Baseline run: `mobs-25k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25058 -> 25058) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 266.834 / 423.158 / 564.543 / 605.912 |
| MSPT mean, ms | 301.706 |
| TPS estimate (1000/p50) | 3.7 |
| heap used / max, MB | 1329.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2657` |
| 2 | 3.00% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 3 | 2.51% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2388` |
| 4 | 2.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.89% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.62% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 7 | 1.51% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10632` |
| 8 | 1.49% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 9 | 1.49% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 10 | 1.33% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7300` |
| 11 | 1.24% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7232` |
| 12 | 1.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.16% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2386` |
| 14 | 1.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

