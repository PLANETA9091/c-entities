# Baseline run: `spread-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97433 -> 97429) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 430.426 / 505.703 / 611.961 / 656.782 |
| MSPT mean, ms | 437.335 |
| TPS estimate (1000/p50) | 2.3 |
| heap used / max, MB | 2175.3 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.43% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 4 | 2.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.47% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 7 | 1.44% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 8 | 1.38% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.27% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 10 | 1.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 12 | 1.15% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 13 | 1.15% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 14 | 1.11% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 15 | 1.09% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |

