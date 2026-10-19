# Baseline run: `tiles-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 68121 (hoppers 42269 / furnaces 25852) |
| MSPT p50 / p90 / p99 / max, ms | 29.385 / 30.618 / 39.287 / 48.610 |
| MSPT mean, ms | 29.759 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 938.4 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.45% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 2 | 9.03% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 3 | 8.77% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 4 | 4.43% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 5 | 4.04% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 6 | 2.95% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 7 | 2.63% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 8 | 2.51% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 9 | 2.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 2.13% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 11 | 1.98% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 12 | 1.86% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.68% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 14 | 1.62% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 15 | 1.54% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |

