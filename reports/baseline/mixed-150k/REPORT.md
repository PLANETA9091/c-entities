# Baseline run: `mixed-150k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110459 -> 110459) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 626.623 / 770.279 / 1048.900 / 1130.409 |
| MSPT mean, ms | 649.605 |
| TPS estimate (1000/p50) | 1.6 |
| heap used / max, MB | 3180.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 33.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 6 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 9 | 1.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.15% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.79% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

