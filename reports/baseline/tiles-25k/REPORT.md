# Baseline run: `tiles-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 24964 (hoppers 15000 / furnaces 9964) |
| MSPT p50 / p90 / p99 / max, ms | 8.349 / 9.219 / 10.143 / 20.035 |
| MSPT mean, ms | 8.286 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 885.7 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.99% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.41% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 4.06% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 2.90% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 2.64% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 2.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.87% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 8 | 1.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.48% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 10 | 1.42% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 11 | 1.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 12 | 1.29% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 13 | 1.29% | `java.util.HashMap.getNode(Object) line: 585` |
| 14 | 1.29% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 15 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

