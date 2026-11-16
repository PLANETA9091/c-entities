# Baseline run: `mixed-150k`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110019 -> 110019) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 468.016 / 626.379 / 814.105 / 920.842 |
| MSPT mean, ms | 496.518 |
| TPS estimate (1000/p50) | 2.1 |
| heap used / max, MB | 1770.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 27.33% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.35% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 2.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.68% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 8 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 13 | 1.11% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 14 | 1.09% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.84% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |

