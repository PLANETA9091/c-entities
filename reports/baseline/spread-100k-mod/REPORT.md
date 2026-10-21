# Baseline run: `spread-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97396 -> 97393) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 497.923 / 627.834 / 742.430 / 863.821 |
| MSPT mean, ms | 513.594 |
| TPS estimate (1000/p50) | 2.0 |
| heap used / max, MB | 2527.3 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 4 | 2.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.94% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 6 | 1.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.45% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 10 | 1.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 12 | 1.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 13 | 1.14% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 14 | 1.08% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 15 | 1.01% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |

