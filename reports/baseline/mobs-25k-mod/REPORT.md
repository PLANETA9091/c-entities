# Baseline run: `mobs-25k-mod`

Server: `26.3-241-5050b0f (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25060 -> 25060) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 266.541 / 422.213 / 552.802 / 592.851 |
| MSPT mean, ms | 298.625 |
| TPS estimate (1000/p50) | 3.8 |
| heap used / max, MB | 1653.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.78% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 2.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 3 | 2.34% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 4 | 1.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.94% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 6 | 1.90% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.77% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 8 | 1.73% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.43% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.42% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 11 | 1.41% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 1.40% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7291` |
| 13 | 1.31% | `java.util.TimSort.minRunLength(int) line: 405` |
| 14 | 1.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10280` |
| 15 | 1.20% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4834` |

