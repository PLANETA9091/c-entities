# Baseline run: `mixed-150k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110401 -> 110390) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 869.404 / 1010.195 / 1107.113 / 1214.770 |
| MSPT mean, ms | 884.240 |
| TPS estimate (1000/p50) | 1.2 |
| heap used / max, MB | 3131.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 36.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 2.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.04% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 7 | 1.04% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 8 | 0.95% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 9 | 0.87% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 10 | 0.84% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 13 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 14 | 0.64% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 15 | 0.64% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |

