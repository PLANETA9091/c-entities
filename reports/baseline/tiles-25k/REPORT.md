# Baseline run: `tiles-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 24964 (hoppers 15000 / furnaces 9964) |
| MSPT p50 / p90 / p99 / max, ms | 8.820 / 9.454 / 11.185 / 21.943 |
| MSPT mean, ms | 8.854 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 969.8 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.58% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 3.56% | `java.util.HashMap.getNode(Object) line: 585` |
| 3 | 2.60% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 4 | 2.26% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 5 | 2.12% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 6 | 1.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 1.78% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 8 | 1.73% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 9 | 1.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 10 | 1.54% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 11 | 1.49% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 12 | 1.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 13 | 1.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 15 | 1.11% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |

