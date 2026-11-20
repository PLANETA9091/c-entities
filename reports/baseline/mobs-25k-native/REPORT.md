# Baseline run: `mobs-25k-native`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24164 -> 23993) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 408.278 / 479.676 / 637.604 / 738.858 |
| MSPT mean, ms | 419.978 |
| TPS estimate (1000/p50) | 2.4 |
| heap used / max, MB | 1722.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 31.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.78% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 2.81% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 2.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.35% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 8 | 1.34% | `java.util.HashMap.getNode(Object) line: 586` |
| 9 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |

