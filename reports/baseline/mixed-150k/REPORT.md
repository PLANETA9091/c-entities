# Baseline run: `mixed-150k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110405 -> 110405) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 614.251 / 742.314 / 1031.382 / 1116.722 |
| MSPT mean, ms | 638.040 |
| TPS estimate (1000/p50) | 1.6 |
| heap used / max, MB | 3260.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 33.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.65% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.56% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 5 | 2.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.72% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.16% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.07% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 0.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 0.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |

