# Baseline run: `items-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 25000 / stands 0 / zombies 0) (merge shrink: 19936 -> 19936) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 26.382 / 29.303 / 40.048 / 52.284 |
| MSPT mean, ms | 27.133 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1362.6 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.59% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 2 | 3.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 3 | 2.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.63% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 2.31% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 6 | 2.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 2.02% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 1.65% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 9 | 1.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 10 | 1.52% | `java.util.HashMap.getNode(Object) line: 585` |
| 11 | 1.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.06% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 15 | 1.06% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |

