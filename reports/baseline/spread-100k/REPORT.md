# Baseline run: `spread-100k`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97334 -> 97334) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 386.149 / 457.360 / 654.255 / 727.894 |
| MSPT mean, ms | 395.196 |
| TPS estimate (1000/p50) | 2.6 |
| heap used / max, MB | 1782.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.70% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.49% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 5 | 1.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 9 | 1.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.60% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 12 | 1.45% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

