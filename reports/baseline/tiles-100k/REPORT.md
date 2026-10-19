# Baseline run: `tiles-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 68121 (hoppers 42269 / furnaces 25852) |
| MSPT p50 / p90 / p99 / max, ms | 22.748 / 24.221 / 31.907 / 44.496 |
| MSPT mean, ms | 23.096 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 877.8 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.26% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 4.97% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 3 | 4.05% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 4 | 3.77% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 5 | 3.46% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 6 | 3.29% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 7 | 3.22% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 3.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 2.23% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 10 | 2.09% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 11 | 1.89% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 12 | 1.78% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 13 | 1.75% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 14 | 1.68% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 15 | 1.54% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |

