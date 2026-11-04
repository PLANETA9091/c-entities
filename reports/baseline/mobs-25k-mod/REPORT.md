# Baseline run: `mobs-25k-mod`

Server: `26.3-240-6b68358 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25056 -> 25056) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 280.342 / 458.479 / 579.857 / 695.395 |
| MSPT mean, ms | 314.756 |
| TPS estimate (1000/p50) | 3.6 |
| heap used / max, MB | 1656.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 2 | 5.51% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 5.40% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 4 | 3.54% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.57% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 7 | 1.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 10 | 1.23% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 11 | 1.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.16% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 13 | 1.15% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7316` |

