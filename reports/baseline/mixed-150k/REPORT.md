# Baseline run: `mixed-150k`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110040 -> 110040) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 460.467 / 639.644 / 829.643 / 956.163 |
| MSPT mean, ms | 491.489 |
| TPS estimate (1000/p50) | 2.2 |
| heap used / max, MB | 3082.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 27.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 7 | 1.56% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 8 | 1.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 13 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 14 | 1.11% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 1.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

