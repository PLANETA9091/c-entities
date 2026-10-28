# Baseline run: `mixed-150k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110248 -> 110248) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 432.482 / 516.920 / 734.459 / 931.846 |
| MSPT mean, ms | 446.524 |
| TPS estimate (1000/p50) | 2.3 |
| heap used / max, MB | 2211.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 26.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.62% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 3.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 3.22% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.44% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 2.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.36% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.33% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.15% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |

