# Baseline run: `mobs-25k`

Server: `26.3-259-c7097a9 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24287 -> 24287) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 743.615 / 832.186 / 910.380 / 1007.270 |
| MSPT mean, ms | 751.894 |
| TPS estimate (1000/p50) | 1.3 |
| heap used / max, MB | 1136.4 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 51.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.62% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 1.28% | `java.util.HashMap.getNode(Object) line: 586` |
| 5 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 0.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 9 | 0.62% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4094` |
| 10 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.51% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 15 | 0.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |

