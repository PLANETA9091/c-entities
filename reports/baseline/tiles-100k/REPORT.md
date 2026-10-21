# Baseline run: `tiles-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 68121 (hoppers 42269 / furnaces 25852) |
| MSPT p50 / p90 / p99 / max, ms | 31.827 / 32.819 / 38.554 / 52.708 |
| MSPT mean, ms | 32.034 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1038.4 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.01% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 7.15% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 6.74% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 3.85% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 3.77% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 2.87% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 2.74% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 8 | 2.71% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 9 | 2.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 2.20% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 11 | 2.14% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 12 | 1.76% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 13 | 1.70% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 1.58% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 15 | 1.52% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |

