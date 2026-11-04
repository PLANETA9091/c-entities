# Baseline run: `mobs-25k-mod`

Server: `26.3-239-13565bc (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25059 -> 25059) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 358.935 / 561.549 / 678.862 / 781.963 |
| MSPT mean, ms | 395.429 |
| TPS estimate (1000/p50) | 2.8 |
| heap used / max, MB | 1719.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 2 | 4.06% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 2.87% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 4 | 2.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 5 | 1.76% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 6 | 1.65% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10676` |
| 7 | 1.54% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 8 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.07% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 12 | 1.02% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10074` |
| 13 | 0.95% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 14 | 0.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7316` |

