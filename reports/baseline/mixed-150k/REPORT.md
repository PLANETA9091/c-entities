# Baseline run: `mixed-150k`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110373 -> 110373) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 422.679 / 508.813 / 700.760 / 787.183 |
| MSPT mean, ms | 435.254 |
| TPS estimate (1000/p50) | 2.4 |
| heap used / max, MB | 2233.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 20.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.26% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 4.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.47% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 1.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.24% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 14 | 1.17% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |

