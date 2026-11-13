# Baseline run: `spread-100k`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88682 -> 88682) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 451.849 / 544.776 / 636.668 / 696.936 |
| MSPT mean, ms | 461.548 |
| TPS estimate (1000/p50) | 2.2 |
| heap used / max, MB | 2007.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 21.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.39% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 3 | 2.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.69% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 10 | 1.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.09% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.01% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 13 | 1.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.93% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

