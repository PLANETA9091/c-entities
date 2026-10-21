# Baseline run: `tiles-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 24964 (hoppers 15000 / furnaces 9964) |
| MSPT p50 / p90 / p99 / max, ms | 9.560 / 10.571 / 12.912 / 23.324 |
| MSPT mean, ms | 9.217 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1092.0 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.18% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.63% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 4.48% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 3.45% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 3.27% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 2.73% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 7 | 2.00% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 8 | 1.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.88% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.57% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 12 | 1.45% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 13 | 1.45% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 14 | 1.45% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 15 | 1.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |

