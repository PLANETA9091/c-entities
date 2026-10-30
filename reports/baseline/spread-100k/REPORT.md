# Baseline run: `spread-100k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97365 -> 97365) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 651.403 / 742.813 / 1033.231 / 1203.186 |
| MSPT mean, ms | 668.826 |
| TPS estimate (1000/p50) | 1.5 |
| heap used / max, MB | 2695.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 30.39% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 2.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.38% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 8 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.19% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 10 | 1.12% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 1.08% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 12 | 0.93% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 13 | 0.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 0.85% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |

