# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25059 -> 25059) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 187.864 / 291.583 / 456.848 / 488.825 |
| MSPT mean, ms | 213.467 |
| TPS estimate (1000/p50) | 5.3 |
| heap used / max, MB | 1833.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.90% | `java.lang.ref.Reference.get() line: 361` |
| 2 | 2.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 3 | 2.59% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 4 | 2.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.27% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10097` |
| 6 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 2.01% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6811` |
| 9 | 1.70% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 10 | 1.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.55% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.43% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4400` |
| 13 | 1.43% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 14 | 1.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4337` |
| 15 | 1.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9499` |

