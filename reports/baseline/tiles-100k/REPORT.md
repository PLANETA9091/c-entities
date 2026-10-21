# Baseline run: `tiles-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 68121 (hoppers 42269 / furnaces 25852) |
| MSPT p50 / p90 / p99 / max, ms | 25.483 / 28.325 / 42.226 / 57.965 |
| MSPT mean, ms | 25.950 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 595.1 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.63% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.49% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 5.38% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 5.11% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 5 | 2.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 2.76% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 7 | 1.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 8 | 1.78% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 9 | 1.69% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 10 | 1.69% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 11 | 1.58% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 12 | 1.56% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 13 | 1.53% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 14 | 1.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 15 | 1.45% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |

