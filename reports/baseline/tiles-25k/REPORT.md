# Baseline run: `tiles-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 24964 (hoppers 15000 / furnaces 9964) |
| MSPT p50 / p90 / p99 / max, ms | 5.245 / 7.918 / 11.851 / 19.616 |
| MSPT mean, ms | 5.825 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1214.0 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.02% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 3.01% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 2.92% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 4 | 2.51% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 2.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 6 | 2.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 2.01% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 8 | 1.92% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 9 | 1.50% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 10 | 1.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.42% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 12 | 1.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 1.34% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 14 | 1.34% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 15 | 1.25% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1175` |

