# Baseline run: `mixed-150k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110367 -> 110354) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 783.046 / 908.069 / 981.911 / 1127.810 |
| MSPT mean, ms | 799.156 |
| TPS estimate (1000/p50) | 1.3 |
| heap used / max, MB | 2202.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 35.91% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 9.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 2.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.39% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 8 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 9 | 1.27% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 10 | 1.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 11 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 0.92% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 13 | 0.92% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 14 | 0.87% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |

