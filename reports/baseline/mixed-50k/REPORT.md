# Baseline run: `mixed-50k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 50000 (items 34897 / stands 10136 / zombies 4967) (merge shrink: 41478 -> 41478) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 155.985 / 176.363 / 218.607 / 251.794 |
| MSPT mean, ms | 160.457 |
| TPS estimate (1000/p50) | 6.4 |
| heap used / max, MB | 978.5 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 17.45% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.09% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 1.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 8 | 1.29% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 9 | 1.26% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 10 | 1.24% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.19% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 12 | 0.98% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 13 | 0.95% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.90% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 15 | 0.86% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |

