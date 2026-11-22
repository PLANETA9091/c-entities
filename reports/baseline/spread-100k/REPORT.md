# Baseline run: `spread-100k`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88547 -> 88540) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 419.839 / 508.607 / 606.378 / 689.427 |
| MSPT mean, ms | 429.180 |
| TPS estimate (1000/p50) | 2.4 |
| heap used / max, MB | 1793.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 20.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.09% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 2.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 2.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 2.01% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 8 | 1.93% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.41% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.25% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 12 | 1.04% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 13 | 1.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.02% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 15 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |

