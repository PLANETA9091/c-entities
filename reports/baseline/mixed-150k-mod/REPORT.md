# Baseline run: `mixed-150k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110301 -> 110301) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 864.640 / 970.629 / 1108.337 / 1205.123 |
| MSPT mean, ms | 871.215 |
| TPS estimate (1000/p50) | 1.2 |
| heap used / max, MB | 1721.5 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 42.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 1.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 0.89% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 7 | 0.89% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 0.85% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 9 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 11 | 0.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 12 | 0.68% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 0.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 0.63% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |

