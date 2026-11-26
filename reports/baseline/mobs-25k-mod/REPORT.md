# Baseline run: `mobs-25k-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24619 -> 24619) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 2335.993 / 2464.637 / 2625.242 / 2746.073 |
| MSPT mean, ms | 1646.016 |
| TPS estimate (1000/p50) | 0.4 |
| heap used / max, MB | 1900.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 49.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4017` |
| 2 | 8.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4089` |
| 3 | 1.09% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4000` |
| 4 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 0.71% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4007` |
| 6 | 0.67% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4016` |
| 7 | 0.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 0.50% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3998` |
| 11 | 0.49% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.42% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16558` |
| 15 | 0.40% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11482` |

