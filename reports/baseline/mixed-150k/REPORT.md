# Baseline run: `mixed-150k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110310 -> 110300) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 1093.581 / 1167.320 / 1264.076 / 1331.054 |
| MSPT mean, ms | 1091.900 |
| TPS estimate (1000/p50) | 0.9 |
| heap used / max, MB | 1668.4 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 42.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 2.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.55% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.23% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.18% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 0.98% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 9 | 0.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 0.79% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 11 | 0.76% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 12 | 0.68% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 13 | 0.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 0.64% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 15 | 0.62% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |

