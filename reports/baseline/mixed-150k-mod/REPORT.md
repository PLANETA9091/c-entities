# Baseline run: `mixed-150k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 100943 -> 100924) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 747.260 / 803.677 / 873.182 / 973.907 |
| MSPT mean, ms | 745.368 |
| TPS estimate (1000/p50) | 1.3 |
| heap used / max, MB | 3012.1 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 28.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 1.80% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 1.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.18% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.07% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 1.00% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 11 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.96% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 0.94% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 14 | 0.91% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 15 | 0.89% | `java.lang.ThreadLocal.get() line: 172` |

