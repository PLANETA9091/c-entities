# Baseline run: `items-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63011 -> 63011) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 90.778 / 100.312 / 139.762 / 180.832 |
| MSPT mean, ms | 94.169 |
| TPS estimate (1000/p50) | 11.0 |
| heap used / max, MB | 2094.5 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.42% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 3 | 5.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 4.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 3.66% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 7 | 2.79% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 2.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 2.44% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.66% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 12 | 1.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 1.34% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 1.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 15 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

