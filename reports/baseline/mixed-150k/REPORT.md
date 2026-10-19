# Baseline run: `mixed-150k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110308 -> 110308) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 742.936 / 820.932 / 876.382 / 951.723 |
| MSPT mean, ms | 747.262 |
| TPS estimate (1000/p50) | 1.3 |
| heap used / max, MB | 1733.3 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 37.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 2.22% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.06% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 0.98% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 9 | 0.93% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 0.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 0.73% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 14 | 0.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |

