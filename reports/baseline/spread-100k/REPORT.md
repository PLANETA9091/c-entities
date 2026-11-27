# Baseline run: `spread-100k`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88644 -> 88644) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 424.058 / 518.527 / 610.292 / 706.964 |
| MSPT mean, ms | 431.430 |
| TPS estimate (1000/p50) | 2.4 |
| heap used / max, MB | 2006.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 18.43% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.17% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 7 | 1.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.17% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 11 | 1.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.06% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.96% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |

