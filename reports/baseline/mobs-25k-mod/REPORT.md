# Baseline run: `mobs-25k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25061 -> 25061) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 159.798 / 358.494 / 602.215 / 676.803 |
| MSPT mean, ms | 202.760 |
| TPS estimate (1000/p50) | 6.3 |
| heap used / max, MB | 1342.6 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.05% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 1.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5020` |
| 5 | 1.67% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 6 | 1.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7984` |
| 9 | 1.27% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.26% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8125` |
| 11 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.10% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.03% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2142` |
| 14 | 0.98% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

