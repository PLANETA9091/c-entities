# Baseline run: `mixed-150k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110359 -> 110359) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 675.096 / 846.403 / 1245.785 / 1421.124 |
| MSPT mean, ms | 710.839 |
| TPS estimate (1000/p50) | 1.5 |
| heap used / max, MB | 1943.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 34.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.93% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.15% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.12% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 0.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.95% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 0.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |

