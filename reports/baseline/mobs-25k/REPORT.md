# Baseline run: `mobs-25k`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24227 -> 24097) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 512.744 / 580.937 / 672.614 / 792.006 |
| MSPT mean, ms | 518.951 |
| TPS estimate (1000/p50) | 2.0 |
| heap used / max, MB | 1616.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 34.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.29% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.26% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 2.10% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 11 | 0.66% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 574` |
| 12 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.62% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 14 | 0.57% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 15 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |

