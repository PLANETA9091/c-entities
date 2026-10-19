# Baseline run: `mixed-150k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110318 -> 110318) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 475.508 / 526.693 / 580.952 / 758.844 |
| MSPT mean, ms | 471.680 |
| TPS estimate (1000/p50) | 2.1 |
| heap used / max, MB | 1701.2 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 24.22% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.21% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 6 | 1.88% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 1.65% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 9 | 1.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.29% | `java.util.HashMap.getNode(Object) line: 585` |
| 13 | 1.20% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 14 | 1.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 15 | 1.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |

