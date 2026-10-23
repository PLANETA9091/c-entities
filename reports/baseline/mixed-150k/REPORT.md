# Baseline run: `mixed-150k`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110413 -> 110413) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 378.171 / 463.758 / 649.641 / 745.368 |
| MSPT mean, ms | 392.466 |
| TPS estimate (1000/p50) | 2.6 |
| heap used / max, MB | 2573.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 17.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 2.61% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 7 | 2.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 8 | 2.37% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.65% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 12 | 1.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.34% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 15 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |

