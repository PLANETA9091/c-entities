# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25057 -> 25057) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 192.919 / 290.130 / 466.434 / 513.087 |
| MSPT mean, ms | 218.900 |
| TPS estimate (1000/p50) | 5.2 |
| heap used / max, MB | 1471.0 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.71% | `java.lang.ref.Reference.get() line: 361` |
| 2 | 2.83% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 2.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 5 | 2.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 6 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.83% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 9 | 1.75% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6832` |
| 10 | 1.58% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.55% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4402` |
| 12 | 1.26% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 13 | 1.23% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9532` |
| 15 | 1.17% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6764` |

