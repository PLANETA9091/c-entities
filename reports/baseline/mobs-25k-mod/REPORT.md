# Baseline run: `mobs-25k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25073 -> 25073) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 196.608 / 378.378 / 635.766 / 720.562 |
| MSPT mean, ms | 227.962 |
| TPS estimate (1000/p50) | 5.1 |
| heap used / max, MB | 1438.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.45% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.84% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.49% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5078` |
| 8 | 1.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8042` |
| 10 | 1.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8183` |
| 12 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.13% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2195` |
| 14 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

