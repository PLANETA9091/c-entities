# Baseline run: `mobs-25k-native`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24158 -> 24099) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 493.537 / 565.978 / 708.375 / 871.182 |
| MSPT mean, ms | 503.676 |
| TPS estimate (1000/p50) | 2.0 |
| heap used / max, MB | 1272.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 31.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.40% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 2.07% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 1.79% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 7 | 1.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.53% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 14 | 0.52% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

