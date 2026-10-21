# Baseline run: `items-100k-mod`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 53271 -> 53271) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 88.309 / 101.794 / 136.924 / 190.098 |
| MSPT mean, ms | 92.025 |
| TPS estimate (1000/p50) | 11.3 |
| heap used / max, MB | 1168.8 / 8192.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.32% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 3 | 4.47% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 4 | 3.90% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 5 | 3.55% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 7 | 2.26% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 2.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 9 | 2.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 2.00% | `java.util.HashMap.getNode(Object) line: 585` |
| 11 | 1.94% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 12 | 1.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 1.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.15% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 15 | 1.09% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |

