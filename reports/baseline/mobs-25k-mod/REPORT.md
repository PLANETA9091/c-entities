# Baseline run: `mobs-25k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24776 -> 24776) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 2357.552 / 2537.066 / 2658.730 / 2741.012 |
| MSPT mean, ms | 2349.825 |
| TPS estimate (1000/p50) | 0.4 |
| heap used / max, MB | 2061.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.74% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1393` |
| 2 | 2.97% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3075` |
| 3 | 2.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3074` |
| 4 | 2.08% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5876` |
| 5 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12263` |
| 8 | 0.91% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.86% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 10 | 0.85% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 0.83% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 0.83% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 13 | 0.78% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 14 | 0.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 0.69% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |

