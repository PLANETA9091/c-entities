# Baseline run: `tiles-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 24964 (hoppers 15000 / furnaces 9964) |
| MSPT p50 / p90 / p99 / max, ms | 7.100 / 9.226 / 12.599 / 20.638 |
| MSPT mean, ms | 7.434 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1153.1 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.71% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 2.88% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 3 | 2.82% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 4 | 2.71% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 2.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 6 | 2.41% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 7 | 2.29% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 8 | 2.12% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 9 | 1.94% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 10 | 1.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 12 | 1.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 13 | 1.18% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 14 | 1.18% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 15 | 1.00% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |

