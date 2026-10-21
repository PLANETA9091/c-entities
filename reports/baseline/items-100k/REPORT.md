# Baseline run: `items-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63132 -> 63132) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 90.490 / 98.736 / 142.268 / 193.770 |
| MSPT mean, ms | 94.352 |
| TPS estimate (1000/p50) | 11.1 |
| heap used / max, MB | 1150.1 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.78% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 2 | 5.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 4.07% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 3.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 6 | 3.03% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 2.27% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 2.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.41% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 12 | 1.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 1.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 14 | 1.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 15 | 1.18% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |

