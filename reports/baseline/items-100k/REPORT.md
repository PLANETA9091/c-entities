# Baseline run: `items-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62955 -> 62955) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 84.512 / 94.433 / 134.795 / 237.593 |
| MSPT mean, ms | 88.340 |
| TPS estimate (1000/p50) | 11.8 |
| heap used / max, MB | 934.5 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.50% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 3 | 5.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 4.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 4.15% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.69% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 2.16% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 9 | 1.91% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 10 | 1.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 11 | 1.68% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.55% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 13 | 1.45% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 14 | 1.37% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 15 | 1.36% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |

