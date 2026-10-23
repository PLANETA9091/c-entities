# Baseline run: `items-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63010 -> 63010) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 101.185 / 119.616 / 155.864 / 234.955 |
| MSPT mean, ms | 105.762 |
| TPS estimate (1000/p50) | 9.9 |
| heap used / max, MB | 1164.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 4.52% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 4.06% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 6 | 2.52% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 2.25% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 10 | 1.70% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 11 | 1.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 12 | 1.52% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 13 | 1.51% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.20% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |

