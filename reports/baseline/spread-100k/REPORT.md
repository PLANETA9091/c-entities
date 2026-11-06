# Baseline run: `spread-100k`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97450 -> 97450) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 535.055 / 613.785 / 869.243 / 928.467 |
| MSPT mean, ms | 553.078 |
| TPS estimate (1000/p50) | 1.9 |
| heap used / max, MB | 2182.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 20.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.91% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 3 | 2.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.69% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 8 | 1.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.35% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.31% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 11 | 1.30% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.18% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 14 | 1.17% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 15 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |

