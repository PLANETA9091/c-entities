# Baseline run: `mixed-150k`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110334 -> 110334) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 437.295 / 534.249 / 743.157 / 833.166 |
| MSPT mean, ms | 450.531 |
| TPS estimate (1000/p50) | 2.3 |
| heap used / max, MB | 2802.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 29.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.29% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 2.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 2.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.04% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 2.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 1.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 12 | 1.03% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 0.90% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |

