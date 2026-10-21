# Baseline run: `mixed-150k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110441 -> 110430) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 644.612 / 722.835 / 800.627 / 865.973 |
| MSPT mean, ms | 646.351 |
| TPS estimate (1000/p50) | 1.6 |
| heap used / max, MB | 1879.1 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 27.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 5 | 2.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 2.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.85% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 1.54% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 11 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.23% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 13 | 1.09% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 14 | 1.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 1.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |

