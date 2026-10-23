# Baseline run: `mixed-150k`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110292 -> 110292) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 439.953 / 523.420 / 743.389 / 819.671 |
| MSPT mean, ms | 453.218 |
| TPS estimate (1000/p50) | 2.3 |
| heap used / max, MB | 2502.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 23.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.31% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 3 | 4.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 3.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.85% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 7 | 2.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 8 | 2.09% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.18% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 11 | 1.18% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 12 | 1.18% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.17% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 14 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 15 | 1.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |

