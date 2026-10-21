# Baseline run: `items-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 25000 / stands 0 / zombies 0) (merge shrink: 20031 -> 20031) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 31.984 / 36.202 / 51.702 / 59.913 |
| MSPT mean, ms | 32.980 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1217.6 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.99% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 3 | 3.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 3.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.55% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 7 | 2.14% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 2.01% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 9 | 1.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 11 | 1.31% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 12 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 13 | 1.22% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 14 | 1.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 0.96% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |

