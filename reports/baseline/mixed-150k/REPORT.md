# Baseline run: `mixed-150k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110462 -> 110462) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 505.837 / 577.145 / 648.826 / 699.294 |
| MSPT mean, ms | 509.710 |
| TPS estimate (1000/p50) | 2.0 |
| heap used / max, MB | 1587.2 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 24.99% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 4.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.84% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 7 | 1.58% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.56% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 9 | 1.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.22% | `java.util.HashMap.getNode(Object) line: 585` |
| 12 | 1.20% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 13 | 1.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 14 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 15 | 1.13% | `java.lang.ThreadLocal.get() line: 172` |

