# Baseline run: `mobs-25k`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24180 -> 23491) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 309.862 / 383.090 / 473.012 / 557.718 |
| MSPT mean, ms | 322.016 |
| TPS estimate (1000/p50) | 3.2 |
| heap used / max, MB | 1709.3 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 24.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.83% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 3.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.96% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 2.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 7 | 1.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 9 | 1.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.07% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.94% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 0.93% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 15 | 0.86% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |

