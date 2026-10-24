# Baseline run: `mixed-150k`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110403 -> 110403) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 467.263 / 567.007 / 767.118 / 867.653 |
| MSPT mean, ms | 483.847 |
| TPS estimate (1000/p50) | 2.1 |
| heap used / max, MB | 2965.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 32.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 3 | 4.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 4 | 2.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.79% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 9 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 0.88% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 13 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.82% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 15 | 0.82% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |

