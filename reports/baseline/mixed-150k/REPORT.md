# Baseline run: `mixed-150k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110370 -> 110370) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 774.259 / 843.834 / 909.067 / 946.202 |
| MSPT mean, ms | 774.741 |
| TPS estimate (1000/p50) | 1.3 |
| heap used / max, MB | 1763.8 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 35.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.26% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 5 | 2.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.70% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 0.97% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 0.90% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 10 | 0.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 11 | 0.87% | `java.util.HashMap.getNode(Object) line: 585` |
| 12 | 0.83% | `java.lang.ThreadLocal.get() line: 172` |
| 13 | 0.77% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 14 | 0.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.73% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |

