# Baseline run: `items-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63007 -> 63007) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 91.341 / 98.673 / 137.961 / 171.922 |
| MSPT mean, ms | 94.635 |
| TPS estimate (1000/p50) | 10.9 |
| heap used / max, MB | 1070.9 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.52% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 3 | 4.99% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 4.16% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 3.23% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 3.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 7 | 2.31% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 2.10% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 10 | 1.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.51% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 12 | 1.48% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 13 | 1.46% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 14 | 1.38% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 15 | 1.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |

