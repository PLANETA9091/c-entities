# Baseline run: `spread-100k`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97480 -> 97480) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 363.874 / 423.204 / 617.135 / 644.142 |
| MSPT mean, ms | 378.728 |
| TPS estimate (1000/p50) | 2.7 |
| heap used / max, MB | 2049.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 5 | 2.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 6 | 1.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.71% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.31% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 11 | 1.31% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 12 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.09% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 14 | 1.05% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 15 | 0.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

