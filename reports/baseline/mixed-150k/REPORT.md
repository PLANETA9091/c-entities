# Baseline run: `mixed-150k`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110378 -> 110378) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 458.267 / 538.017 / 782.751 / 930.693 |
| MSPT mean, ms | 473.606 |
| TPS estimate (1000/p50) | 2.2 |
| heap used / max, MB | 2196.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 23.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 4 | 3.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.36% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 7 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 9 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 11 | 1.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.34% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 14 | 1.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 15 | 1.10% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

