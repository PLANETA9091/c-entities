# Baseline run: `mobs-25k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25055 -> 25055) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 292.450 / 520.213 / 677.398 / 744.186 |
| MSPT mean, ms | 340.664 |
| TPS estimate (1000/p50) | 3.4 |
| heap used / max, MB | 1377.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.19% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10202` |
| 2 | 2.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 1.63% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 4 | 1.59% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9600` |
| 8 | 1.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.26% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 1.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.04% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6821` |
| 14 | 0.96% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6889` |
| 15 | 0.88% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |

