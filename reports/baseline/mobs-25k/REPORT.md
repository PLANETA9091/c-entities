# Baseline run: `mobs-25k`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24237 -> 24074) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 445.812 / 524.850 / 692.367 / 794.693 |
| MSPT mean, ms | 458.000 |
| TPS estimate (1000/p50) | 2.2 |
| heap used / max, MB | 1218.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 29.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 1.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 1.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.26% | `java.util.HashMap.getNode(Object) line: 586` |
| 8 | 1.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 9 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.02% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 13 | 0.67% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 14 | 0.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.65% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

