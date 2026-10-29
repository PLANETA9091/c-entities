# Baseline run: `spread-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97378 -> 97378) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 440.745 / 507.723 / 721.626 / 784.901 |
| MSPT mean, ms | 454.111 |
| TPS estimate (1000/p50) | 2.3 |
| heap used / max, MB | 2230.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 17.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.51% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.09% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.72% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 8 | 1.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.60% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 12 | 1.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.47% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.38% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 15 | 1.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |

