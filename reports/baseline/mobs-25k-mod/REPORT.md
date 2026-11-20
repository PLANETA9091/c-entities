# Baseline run: `mobs-25k-mod`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24449 -> 24449) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 1993.669 / 2227.067 / 2342.951 / 2483.849 |
| MSPT mean, ms | 1380.023 |
| TPS estimate (1000/p50) | 0.5 |
| heap used / max, MB | 1805.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 36.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3838` |
| 2 | 4.96% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3910` |
| 3 | 1.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3820` |
| 4 | 1.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3852` |
| 5 | 1.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 1.37% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3837` |
| 7 | 1.26% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 0.92% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 11 | 0.86% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 0.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 0.65% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 14 | 0.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10604` |
| 15 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |

