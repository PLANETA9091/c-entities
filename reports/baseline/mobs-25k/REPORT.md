# Baseline run: `mobs-25k`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25070 -> 25070) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 552.464 / 623.815 / 725.546 / 956.021 |
| MSPT mean, ms | 560.691 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 1243.8 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 36.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 2.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.20% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 2.08% | `java.util.HashMap.getNode(Object) line: 586` |
| 7 | 1.85% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 0.87% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.76% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4094` |
| 12 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.63% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.59% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

