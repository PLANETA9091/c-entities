# Baseline run: `spread-100k`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88635 -> 88635) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 441.339 / 533.220 / 635.800 / 756.367 |
| MSPT mean, ms | 451.106 |
| TPS estimate (1000/p50) | 2.3 |
| heap used / max, MB | 1955.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 20.21% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.00% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 1.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 1.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.61% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 8 | 1.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.02% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.94% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 0.94% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

