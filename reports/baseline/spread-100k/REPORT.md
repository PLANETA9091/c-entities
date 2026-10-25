# Baseline run: `spread-100k`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97273 -> 97273) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 366.193 / 410.553 / 587.579 / 634.154 |
| MSPT mean, ms | 370.822 |
| TPS estimate (1000/p50) | 2.7 |
| heap used / max, MB | 1766.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.98% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.16% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 6 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 1.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.72% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.17% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 12 | 1.09% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 13 | 1.08% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 14 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 15 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |

