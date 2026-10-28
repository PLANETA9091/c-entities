# Baseline run: `mixed-150k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110415 -> 110415) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 370.608 / 443.626 / 630.673 / 723.668 |
| MSPT mean, ms | 384.644 |
| TPS estimate (1000/p50) | 2.7 |
| heap used / max, MB | 2261.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 17.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 4.01% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 5 | 3.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 9 | 1.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 12 | 1.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.41% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 14 | 1.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.26% | `java.util.EnumMap.get(Object) line: 248` |

