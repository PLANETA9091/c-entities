# Baseline run: `items-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63017 -> 63017) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 104.540 / 115.257 / 159.376 / 212.246 |
| MSPT mean, ms | 108.973 |
| TPS estimate (1000/p50) | 9.6 |
| heap used / max, MB | 1385.0 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 5.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 5.08% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 4.14% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.81% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 8 | 2.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.52% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 10 | 1.51% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 11 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 14 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 15 | 1.13% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |

