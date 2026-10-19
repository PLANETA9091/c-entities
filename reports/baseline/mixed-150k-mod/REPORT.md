# Baseline run: `mixed-150k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110410 -> 110410) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 821.839 / 893.989 / 973.662 / 1019.449 |
| MSPT mean, ms | 826.243 |
| TPS estimate (1000/p50) | 1.2 |
| heap used / max, MB | 1651.8 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 38.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.14% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 1.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.10% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 0.97% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 0.95% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 9 | 0.83% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 10 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 12 | 0.78% | `java.util.HashMap.getNode(Object) line: 585` |
| 13 | 0.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |

