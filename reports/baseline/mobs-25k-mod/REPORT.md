# Baseline run: `mobs-25k-mod`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24728 -> 24728) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 3124.613 / 3430.535 / 3698.409 / 4001.723 |
| MSPT mean, ms | 2930.185 |
| TPS estimate (1000/p50) | 0.3 |
| heap used / max, MB | 1729.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 60.81% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4059` |
| 2 | 6.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4131` |
| 3 | 1.79% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4073` |
| 4 | 1.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4058` |
| 5 | 1.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4049` |
| 6 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 0.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4041` |
| 9 | 0.43% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 0.27% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11537` |
| 12 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16623` |
| 13 | 0.27% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 0.26% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |

