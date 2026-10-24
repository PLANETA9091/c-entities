# Baseline run: `mixed-150k`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110307 -> 110307) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 433.343 / 527.538 / 730.860 / 828.440 |
| MSPT mean, ms | 451.517 |
| TPS estimate (1000/p50) | 2.3 |
| heap used / max, MB | 2602.1 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 20.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.99% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 4 | 3.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.65% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 7 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.30% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.29% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 15 | 1.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |

