# Baseline run: `spread-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97453 -> 97453) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 337.862 / 401.625 / 577.500 / 652.312 |
| MSPT mean, ms | 344.244 |
| TPS estimate (1000/p50) | 3.0 |
| heap used / max, MB | 1901.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.93% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 6 | 2.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 8 | 1.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.72% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 11 | 1.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 14 | 1.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.39% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

