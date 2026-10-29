# Baseline run: `mobs-25k-mod`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25055 -> 25055) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 178.505 / 371.654 / 644.804 / 809.211 |
| MSPT mean, ms | 216.956 |
| TPS estimate (1000/p50) | 5.6 |
| heap used / max, MB | 1939.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.94% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 1.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4896` |
| 7 | 1.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7836` |
| 8 | 1.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.22% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7977` |
| 11 | 1.16% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.10% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.03% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

