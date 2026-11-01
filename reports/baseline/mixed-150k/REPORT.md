# Baseline run: `mixed-150k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110350 -> 110350) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 627.112 / 741.943 / 956.197 / 1046.831 |
| MSPT mean, ms | 646.058 |
| TPS estimate (1000/p50) | 1.6 |
| heap used / max, MB | 3133.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 39.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.33% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 10 | 1.01% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 0.97% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 12 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 13 | 0.80% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |

