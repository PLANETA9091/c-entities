# Baseline run: `spread-100k`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88619 -> 88599) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 408.683 / 496.051 / 612.741 / 705.390 |
| MSPT mean, ms | 418.555 |
| TPS estimate (1000/p50) | 2.4 |
| heap used / max, MB | 1701.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 21.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.32% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 3.15% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 2.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 1.79% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 8 | 1.65% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 12 | 1.08% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 13 | 1.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 0.86% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 15 | 0.83% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |

