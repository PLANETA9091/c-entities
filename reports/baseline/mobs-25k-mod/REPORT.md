# Baseline run: `mobs-25k-mod`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25052 -> 25052) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 227.805 / 399.026 / 524.669 / 592.951 |
| MSPT mean, ms | 263.548 |
| TPS estimate (1000/p50) | 4.4 |
| heap used / max, MB | 1419.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.62% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.07% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 3 | 1.89% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10301` |
| 4 | 1.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.61% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 6 | 1.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.54% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9699` |
| 12 | 1.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6988` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6413` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4509` |

