# Baseline run: `tiles-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 68121 (hoppers 42269 / furnaces 25852) |
| MSPT p50 / p90 / p99 / max, ms | 29.186 / 32.985 / 42.811 / 55.041 |
| MSPT mean, ms | 29.851 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 770.7 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.13% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 2 | 8.85% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 3 | 7.26% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 4 | 4.37% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 4.00% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 2.98% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 2.64% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 8 | 2.52% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 9 | 2.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.79% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 11 | 1.64% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 12 | 1.50% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 13 | 1.42% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 14 | 1.36% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 15 | 1.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

