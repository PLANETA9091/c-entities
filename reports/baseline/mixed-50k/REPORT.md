# Baseline run: `mixed-50k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 50000 (items 34897 / stands 10136 / zombies 4967) (merge shrink: 41464 -> 41464) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 140.468 / 159.457 / 205.964 / 228.866 |
| MSPT mean, ms | 144.655 |
| TPS estimate (1000/p50) | 7.1 |
| heap used / max, MB | 2645.0 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 15.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.32% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.14% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 1.42% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 8 | 1.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 10 | 1.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 11 | 1.19% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 12 | 1.09% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 13 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.01% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 15 | 0.97% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |

