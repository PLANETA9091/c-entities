# Baseline run: `tiles-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 24964 (hoppers 15000 / furnaces 9964) |
| MSPT p50 / p90 / p99 / max, ms | 9.522 / 10.558 / 13.276 / 36.174 |
| MSPT mean, ms | 9.720 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 951.7 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.98% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 4.65% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 3 | 3.12% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 4 | 2.70% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 5 | 2.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 6 | 1.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 1.95% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 8 | 1.77% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 9 | 1.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.30% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 11 | 1.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 12 | 1.16% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 13 | 1.12% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 14 | 1.07% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 15 | 1.07% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |

