# Baseline run: `mixed-150k`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110337 -> 110337) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 549.782 / 693.749 / 1062.047 / 1198.439 |
| MSPT mean, ms | 579.537 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 1745.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 28.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.53% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.70% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.06% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 6 | 1.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 8 | 1.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 0.94% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 0.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |

