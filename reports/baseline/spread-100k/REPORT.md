# Baseline run: `spread-100k`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97434 -> 97434) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 447.756 / 517.050 / 727.735 / 768.985 |
| MSPT mean, ms | 462.836 |
| TPS estimate (1000/p50) | 2.2 |
| heap used / max, MB | 2327.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 22.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.76% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.58% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 4 | 2.33% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 1.94% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.36% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.05% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.97% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 14 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 0.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |

