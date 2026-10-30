# Baseline run: `mixed-150k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110285 -> 110285) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 720.821 / 847.584 / 1335.326 / 1446.031 |
| MSPT mean, ms | 752.748 |
| TPS estimate (1000/p50) | 1.4 |
| heap used / max, MB | 2103.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 37.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 2.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.79% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.12% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.04% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 14 | 0.85% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |

