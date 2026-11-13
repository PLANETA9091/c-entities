# Baseline run: `mobs-25k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24144 -> 24036) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 229.526 / 428.543 / 694.707 / 844.686 |
| MSPT mean, ms | 275.107 |
| TPS estimate (1000/p50) | 4.4 |
| heap used / max, MB | 2278.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 2.14% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 2 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 1.51% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 1.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11848` |
| 5 | 1.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 9 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 10 | 1.20% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.13% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7890` |
| 12 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7346` |
| 14 | 0.94% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5267` |

