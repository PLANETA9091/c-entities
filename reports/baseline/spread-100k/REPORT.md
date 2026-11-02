# Baseline run: `spread-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97369 -> 97369) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 581.658 / 657.970 / 868.552 / 951.780 |
| MSPT mean, ms | 596.532 |
| TPS estimate (1000/p50) | 1.7 |
| heap used / max, MB | 1990.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 33.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.81% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.18% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.03% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 11 | 0.99% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 13 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 14 | 0.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 0.83% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

