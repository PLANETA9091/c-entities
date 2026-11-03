# Baseline run: `mobs-25k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25071 -> 25071) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 316.525 / 557.130 / 664.121 / 796.179 |
| MSPT mean, ms | 363.016 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 1917.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 2 | 5.08% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 4.47% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 4 | 3.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 5 | 1.82% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.49% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.11% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 1.11% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10643` |
| 11 | 1.07% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.05% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1291` |
| 15 | 0.87% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4832` |

