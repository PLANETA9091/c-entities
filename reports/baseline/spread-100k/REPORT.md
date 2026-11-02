# Baseline run: `spread-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97443 -> 97443) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 540.946 / 617.861 / 877.626 / 971.713 |
| MSPT mean, ms | 554.944 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 2104.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 20.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 1.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 5 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.63% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 8 | 1.52% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 9 | 1.32% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.25% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.11% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 13 | 1.11% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 14 | 1.10% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |

