# Baseline run: `mixed-150k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110440 -> 110424) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 1437.695 / 1575.324 / 1672.718 / 1840.809 |
| MSPT mean, ms | 1439.290 |
| TPS estimate (1000/p50) | 0.7 |
| heap used / max, MB | 1766.4 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 50.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 1.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.46% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.29% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.28% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 0.77% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.73% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 11 | 0.64% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 12 | 0.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 13 | 0.58% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 0.56% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 15 | 0.56% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |

