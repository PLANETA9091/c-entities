# Baseline run: `mobs-25k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25062 -> 25062) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 269.556 / 423.208 / 542.266 / 616.413 |
| MSPT mean, ms | 299.763 |
| TPS estimate (1000/p50) | 3.7 |
| heap used / max, MB | 1730.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.66% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.66% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 1.77% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8481` |
| 6 | 1.61% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8622` |
| 7 | 1.60% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 11 | 1.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5517` |
| 12 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 0.96% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.94% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 15 | 0.91% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

