# Baseline run: `spread-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97363 -> 97358) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 472.381 / 550.888 / 645.794 / 693.640 |
| MSPT mean, ms | 479.756 |
| TPS estimate (1000/p50) | 2.1 |
| heap used / max, MB | 2070.2 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.49% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.46% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 1.44% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.24% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 11 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 12 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.01% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 15 | 0.97% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |

