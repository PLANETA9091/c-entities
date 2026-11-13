# Baseline run: `mobs-25k`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24215 -> 24215) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 551.376 / 621.157 / 686.925 / 953.869 |
| MSPT mean, ms | 558.467 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 1474.1 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 36.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 2.73% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.89% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 0.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 0.88% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4094` |
| 9 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.83% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.80% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 12 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.56% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

