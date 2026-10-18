# Baseline run: `tiles-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) |
| block entities | 24964 (hoppers 15000 / furnaces 9964) |
| MSPT p50 / p90 / p99 / max, ms | 6.427 / 8.167 / 10.467 / 25.042 |
| MSPT mean, ms | 6.736 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 614.0 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.10% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 3.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 3 | 2.26% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 2.19% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 5 | 2.06% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 6 | 2.06% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 7 | 2.06% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 8 | 1.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.86% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.53% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 11 | 1.39% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 12 | 1.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 13 | 1.13% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 14 | 1.06% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 15 | 1.06% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |

