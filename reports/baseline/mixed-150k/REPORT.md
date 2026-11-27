# Baseline run: `mixed-150k`

Server: `26.3-263-894b2e1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109985 -> 109985) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 569.497 / 756.278 / 966.770 / 1054.972 |
| MSPT mean, ms | 601.741 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 1929.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 37.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.26% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.72% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 2.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 11 | 1.14% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 12 | 0.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 13 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 14 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 15 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |

