# Baseline run: `mobs-25k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25066 -> 25066) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 631.464 / 940.647 / 1067.686 / 1118.076 |
| MSPT mean, ms | 659.841 |
| TPS estimate (1000/p50) | 1.6 |
| heap used / max, MB | 1560.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 1.61% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 1.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 3 | 1.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10836` |
| 4 | 1.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 5 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.08% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.02% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 11 | 0.95% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 0.95% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7622` |
| 13 | 0.92% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2726` |
| 14 | 0.87% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1326` |
| 15 | 0.83% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |

