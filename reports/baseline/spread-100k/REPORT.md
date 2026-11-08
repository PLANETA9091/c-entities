# Baseline run: `spread-100k`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97482 -> 97482) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 591.890 / 710.266 / 961.636 / 1030.044 |
| MSPT mean, ms | 615.128 |
| TPS estimate (1000/p50) | 1.7 |
| heap used / max, MB | 2980.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 31.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.43% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 2.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.28% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 9 | 1.20% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 10 | 1.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.04% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 13 | 0.93% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 14 | 0.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 15 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |

