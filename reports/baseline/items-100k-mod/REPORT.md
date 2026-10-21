# Baseline run: `items-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 53475 -> 53475) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 45.562 / 100.138 / 116.696 / 153.665 |
| MSPT mean, ms | 66.446 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1261.0 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 7.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 3 | 6.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 5.86% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 5.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 6 | 5.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 7 | 3.05% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 2.84% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 9 | 2.46% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 2.08% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 11 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 12 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 13 | 0.95% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 219` |
| 14 | 0.94% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 15 | 0.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |

