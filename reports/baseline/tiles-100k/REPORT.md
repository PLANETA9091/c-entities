# Baseline run: `tiles-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) |
| block entities | 68121 (hoppers 42269 / furnaces 25852) |
| MSPT p50 / p90 / p99 / max, ms | 21.338 / 22.385 / 35.915 / 46.439 |
| MSPT mean, ms | 21.553 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1340.9 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.79% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 7.95% | `java.util.HashMap.getNode(Object) line: 585` |
| 3 | 7.05% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 6.31% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 5 | 4.47% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 6 | 3.41% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 7 | 2.59% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 8 | 2.23% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 9 | 2.12% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 10 | 1.92% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 11 | 1.84% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.61% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 13 | 1.25% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 14 | 1.25% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 15 | 1.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |

