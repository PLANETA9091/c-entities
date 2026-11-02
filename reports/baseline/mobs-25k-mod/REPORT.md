# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25050 -> 25050) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 294.775 / 586.139 / 742.026 / 896.222 |
| MSPT mean, ms | 352.769 |
| TPS estimate (1000/p50) | 3.4 |
| heap used / max, MB | 1687.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.58% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 3 | 2.45% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 2.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9877` |
| 7 | 1.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 9 | 1.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9541` |
| 10 | 1.44% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.07% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 13 | 1.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 0.96% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 15 | 0.92% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |

