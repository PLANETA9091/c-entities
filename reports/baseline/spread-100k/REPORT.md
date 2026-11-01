# Baseline run: `spread-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97469 -> 97469) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 524.052 / 596.527 / 821.198 / 987.435 |
| MSPT mean, ms | 539.130 |
| TPS estimate (1000/p50) | 1.9 |
| heap used / max, MB | 2119.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 22.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.82% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 3 | 2.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.02% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.99% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.26% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 1.20% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 0.97% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

