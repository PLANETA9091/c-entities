# Baseline run: `items-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 25000 / stands 0 / zombies 0) (merge shrink: 19968 -> 19968) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 20.956 / 22.812 / 35.017 / 58.650 |
| MSPT mean, ms | 21.880 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 938.8 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 2 | 3.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.68% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 4 | 2.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 1.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 6 | 1.59% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 1.53% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 8 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.21% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 12 | 1.21% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.18% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 14 | 1.18% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 15 | 1.07% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |

