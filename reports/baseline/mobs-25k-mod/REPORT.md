# Baseline run: `mobs-25k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24805 -> 24805) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 2422.432 / 2610.300 / 2769.104 / 2925.929 |
| MSPT mean, ms | 2432.331 |
| TPS estimate (1000/p50) | 0.4 |
| heap used / max, MB | 1885.7 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.04% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1410` |
| 2 | 4.23% | `java.util.ArrayList.elementData(int) line: 412` |
| 3 | 3.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3142` |
| 4 | 3.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3143` |
| 5 | 1.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 1.09% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.01% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 10 | 0.96% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12953` |
| 12 | 0.91% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 13 | 0.83% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 14 | 0.66% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8829` |
| 15 | 0.65% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |

