# Baseline run: `mobs-25k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25055 -> 25055) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 640.218 / 844.700 / 1033.856 / 1192.144 |
| MSPT mean, ms | 658.092 |
| TPS estimate (1000/p50) | 1.6 |
| heap used / max, MB | 1651.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2797` |
| 2 | 2.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 1.64% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10701` |
| 6 | 1.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.20% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.09% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2488` |
| 9 | 1.09% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 1.05% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.02% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 12 | 0.89% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7487` |
| 13 | 0.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 0.83% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |

