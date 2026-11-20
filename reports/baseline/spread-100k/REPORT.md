# Baseline run: `spread-100k`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88668 -> 88668) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 548.358 / 638.668 / 761.125 / 831.133 |
| MSPT mean, ms | 556.259 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 1864.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 28.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 1.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 1.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.07% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 12 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.00% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 15 | 0.98% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |

