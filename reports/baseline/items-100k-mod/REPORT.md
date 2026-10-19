# Baseline run: `items-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62989 -> 62989) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 88.502 / 94.926 / 125.418 / 166.753 |
| MSPT mean, ms | 91.170 |
| TPS estimate (1000/p50) | 11.3 |
| heap used / max, MB | 2034.6 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.92% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.62% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 3 | 5.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 4.52% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 3.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 2.56% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 1.98% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 9 | 1.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 10 | 1.90% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 11 | 1.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 12 | 1.84% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 13 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.get(int) line: 102` |
| 14 | 1.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |

