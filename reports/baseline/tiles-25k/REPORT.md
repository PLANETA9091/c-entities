# Baseline run: `tiles-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) |
| block entities | 24964 (hoppers 15000 / furnaces 9964) |
| MSPT p50 / p90 / p99 / max, ms | 10.634 / 11.076 / 15.247 / 29.895 |
| MSPT mean, ms | 10.836 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1106.0 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.07% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 2 | 9.00% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 3 | 7.18% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 5.02% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 4.37% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 2.61% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 7 | 2.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 8 | 1.70% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 9 | 1.70% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 10 | 1.50% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 11 | 1.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.37% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 13 | 1.30% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 14 | 1.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 15 | 1.17% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |

