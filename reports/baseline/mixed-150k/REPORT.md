# Baseline run: `mixed-150k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110303 -> 110303) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 525.669 / 632.385 / 844.942 / 955.500 |
| MSPT mean, ms | 544.763 |
| TPS estimate (1000/p50) | 1.9 |
| heap used / max, MB | 2910.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 29.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 3.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.93% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 5 | 2.83% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.63% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 11 | 1.31% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.09% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 1.05% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |

