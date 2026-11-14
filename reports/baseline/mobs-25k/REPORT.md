# Baseline run: `mobs-25k`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24161 -> 24074) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 490.949 / 560.093 / 648.119 / 820.065 |
| MSPT mean, ms | 498.802 |
| TPS estimate (1000/p50) | 2.0 |
| heap used / max, MB | 1074.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 32.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.17% | `java.util.HashMap.getNode(Object) line: 586` |
| 5 | 1.84% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 0.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 0.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.77% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 11 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.66% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 14 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

