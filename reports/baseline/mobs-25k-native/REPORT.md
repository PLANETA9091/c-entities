# Baseline run: `mobs-25k-native`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24165 -> 24165) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 533.181 / 604.163 / 730.596 / 930.431 |
| MSPT mean, ms | 541.073 |
| TPS estimate (1000/p50) | 1.9 |
| heap used / max, MB | 926.8 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 36.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 2.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.00% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 1.21% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 0.94% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.78% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4095` |
| 12 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.67% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 0.61% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |

