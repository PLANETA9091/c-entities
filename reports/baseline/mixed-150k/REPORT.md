# Baseline run: `mixed-150k`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110090 -> 110090) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 490.088 / 658.137 / 930.592 / 1048.174 |
| MSPT mean, ms | 519.085 |
| TPS estimate (1000/p50) | 2.0 |
| heap used / max, MB | 3092.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 27.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.28% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.26% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 3.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.91% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 8 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.20% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 12 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 13 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 14 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |

