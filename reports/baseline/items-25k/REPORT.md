# Baseline run: `items-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 25000 / stands 0 / zombies 0) (merge shrink: 19922 -> 19922) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 23.365 / 26.425 / 36.179 / 52.182 |
| MSPT mean, ms | 24.405 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1614.0 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.49% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 2 | 2.79% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 3 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.03% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 2.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 6 | 1.82% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 7 | 1.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 1.65% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.22% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 12 | 1.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.16% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 14 | 1.14% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 15 | 1.08% | `net.minecraft.util.Mth.floor(double) line: 67` |

