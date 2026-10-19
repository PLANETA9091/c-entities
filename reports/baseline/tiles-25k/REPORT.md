# Baseline run: `tiles-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) |
| block entities | 24964 (hoppers 15000 / furnaces 9964) |
| MSPT p50 / p90 / p99 / max, ms | 10.482 / 11.411 / 14.396 / 26.145 |
| MSPT mean, ms | 10.675 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 894.7 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.66% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.61% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.isRemoved() line: 956` |
| 3 | 4.65% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 2.85% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 2.71% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 2.12% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 7 | 1.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 8 | 1.70% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 9 | 1.52% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 10 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.43% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 12 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.20% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 14 | 1.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 15 | 1.15% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |

