# Baseline run: `spread-100k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97491 -> 97491) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 381.141 / 449.500 / 637.376 / 915.664 |
| MSPT mean, ms | 392.583 |
| TPS estimate (1000/p50) | 2.6 |
| heap used / max, MB | 2008.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 1.99% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 6 | 1.91% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.72% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 9 | 1.71% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 10 | 1.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.59% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 14 | 1.41% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 1.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |

