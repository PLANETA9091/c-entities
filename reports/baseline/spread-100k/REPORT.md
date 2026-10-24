# Baseline run: `spread-100k`

Server: `26.3-218-1b9bf5c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97385 -> 97385) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 325.125 / 383.346 / 578.197 / 644.017 |
| MSPT mean, ms | 337.392 |
| TPS estimate (1000/p50) | 3.1 |
| heap used / max, MB | 2659.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 3.12% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 2.86% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 2.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.93% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 9 | 1.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 12 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.26% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 1.25% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |

