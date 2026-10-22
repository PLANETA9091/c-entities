# Baseline run: `tiles-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 24964 (hoppers 15000 / furnaces 9964) |
| MSPT p50 / p90 / p99 / max, ms | 6.650 / 7.817 / 10.251 / 22.989 |
| MSPT mean, ms | 6.876 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 734.8 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.80% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 2.63% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 3 | 2.48% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 4 | 2.27% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 5 | 2.01% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 1.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.70% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 8 | 1.60% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 9 | 1.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 12 | 1.24% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 13 | 1.14% | `java.lang.String.hashCode() line: 2358` |
| 14 | 1.14% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 15 | 1.08% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |

