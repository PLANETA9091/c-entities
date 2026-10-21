# Baseline run: `tiles-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 24964 (hoppers 15000 / furnaces 9964) |
| MSPT p50 / p90 / p99 / max, ms | 8.343 / 9.615 / 15.403 / 23.183 |
| MSPT mean, ms | 8.644 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 546.2 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.56% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 3.52% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 3 | 2.48% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 4 | 2.31% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 5 | 2.31% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 2.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 2.20% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 8 | 1.87% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 9 | 1.76% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 10 | 1.76% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 11 | 1.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.54% | `java.util.HashMap.getNode(Object) line: 585` |
| 13 | 1.43% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 14 | 1.43% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 15 | 1.32% | `net.minecraft.core.NonNullList.get(int) line: 44` |

