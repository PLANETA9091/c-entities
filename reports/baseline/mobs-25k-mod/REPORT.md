# Baseline run: `mobs-25k-mod`

Server: `26.3-259-c7097a9 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24725 -> 24725) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 3397.715 / 3558.367 / 3740.066 / 3822.569 |
| MSPT mean, ms | 2613.856 |
| TPS estimate (1000/p50) | 0.3 |
| heap used / max, MB | 1488.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 64.09% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3952` |
| 2 | 6.16% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4024` |
| 3 | 1.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3966` |
| 4 | 1.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3950` |
| 5 | 1.02% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3933` |
| 6 | 0.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3934` |
| 7 | 0.74% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3942` |
| 8 | 0.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3984` |
| 9 | 0.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3935` |
| 10 | 0.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 0.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 0.29% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 14 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15992` |
| 15 | 0.22% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |

