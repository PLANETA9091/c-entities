# Baseline run: `mixed-150k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 110376 -> 110360) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 837.587 / 905.928 / 993.069 / 1071.353 |
| MSPT mean, ms | 834.896 |
| TPS estimate (1000/p50) | 1.2 |
| heap used / max, MB | 1980.2 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 32.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 4.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 2.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.61% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 1.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.24% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 8 | 1.13% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 9 | 1.02% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 10 | 0.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 11 | 0.87% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.77% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 14 | 0.69% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 0.67% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |

