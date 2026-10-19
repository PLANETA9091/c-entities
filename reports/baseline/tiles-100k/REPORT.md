# Baseline run: `tiles-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) |
| block entities | 68121 (hoppers 42269 / furnaces 25852) |
| MSPT p50 / p90 / p99 / max, ms | 23.625 / 24.486 / 30.628 / 49.231 |
| MSPT mean, ms | 23.970 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1382.1 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.76% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.71% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 3 | 4.05% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 4 | 4.05% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 5 | 2.99% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 6 | 2.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 2.60% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 2.02% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 9 | 1.87% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 10 | 1.72% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 11 | 1.69% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 12 | 1.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 14 | 1.33% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 15 | 1.09% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |

