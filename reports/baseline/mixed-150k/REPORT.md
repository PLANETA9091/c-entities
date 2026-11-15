# Baseline run: `mixed-150k`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110052 -> 110052) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 469.006 / 652.290 / 835.718 / 981.746 |
| MSPT mean, ms | 499.360 |
| TPS estimate (1000/p50) | 2.1 |
| heap used / max, MB | 3173.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 26.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.81% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 2.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 9 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 12 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 13 | 1.18% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.13% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 1.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

