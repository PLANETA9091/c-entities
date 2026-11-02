# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25056 -> 25056) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 231.816 / 395.232 / 527.052 / 671.673 |
| MSPT mean, ms | 266.066 |
| TPS estimate (1000/p50) | 4.3 |
| heap used / max, MB | 1301.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.83% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 2.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 1.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.71% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 5 | 1.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9597` |
| 6 | 1.52% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.45% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 8 | 1.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.36% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10199` |
| 11 | 1.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6311` |
| 12 | 1.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6886` |
| 13 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.13% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.07% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |

