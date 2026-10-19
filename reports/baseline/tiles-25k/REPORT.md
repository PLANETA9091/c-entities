# Baseline run: `tiles-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) |
| block entities | 24964 (hoppers 15000 / furnaces 9964) |
| MSPT p50 / p90 / p99 / max, ms | 10.032 / 10.312 / 16.784 / 25.405 |
| MSPT mean, ms | 10.149 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 530.7 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.29% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 8.15% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 8.08% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 5.52% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 4.65% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 3.03% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 7 | 1.82% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 8 | 1.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 10 | 1.41% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 11 | 1.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 12 | 1.35% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 13 | 1.28% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 14 | 1.14% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1173` |
| 15 | 1.08% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |

