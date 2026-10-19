# Baseline run: `tiles-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 24964 (hoppers 15000 / furnaces 9964) |
| MSPT p50 / p90 / p99 / max, ms | 9.948 / 10.392 / 11.646 / 20.731 |
| MSPT mean, ms | 9.991 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1382.3 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.92% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 6.56% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 5.51% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 4 | 5.31% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 5 | 3.15% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 3.02% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 2.82% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 8 | 2.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.97% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 10 | 1.51% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 11 | 1.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 12 | 1.38% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 13 | 1.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.25% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 15 | 1.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |

