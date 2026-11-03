# Baseline run: `spread-100k`

Server: `26.3-238-df120fd (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97403 -> 97403) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 426.384 / 485.619 / 681.370 / 722.935 |
| MSPT mean, ms | 436.324 |
| TPS estimate (1000/p50) | 2.3 |
| heap used / max, MB | 2153.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 18.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.21% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.80% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.83% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 1.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 10 | 1.60% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.35% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 1.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |

