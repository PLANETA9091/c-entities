# Baseline run: `mixed-150k`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110277 -> 110277) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 512.587 / 649.081 / 992.365 / 1079.317 |
| MSPT mean, ms | 540.596 |
| TPS estimate (1000/p50) | 2.0 |
| heap used / max, MB | 1677.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 26.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.98% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 6 | 2.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.63% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 9 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.00% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 0.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

