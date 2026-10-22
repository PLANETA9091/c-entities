# Baseline run: `tiles-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 68121 (hoppers 42269 / furnaces 25852) |
| MSPT p50 / p90 / p99 / max, ms | 23.522 / 25.742 / 42.644 / 62.289 |
| MSPT mean, ms | 24.195 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1228.2 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.03% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 8.67% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 3 | 6.33% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 4 | 5.38% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 5 | 5.31% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 6 | 5.09% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 7 | 3.59% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 3.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 9 | 1.98% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 10 | 1.72% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 11 | 1.57% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 12 | 1.43% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 13 | 1.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.21% | `net.minecraft.core.NonNullList.get(int) line: 44` |

