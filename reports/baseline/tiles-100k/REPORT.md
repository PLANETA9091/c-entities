# Baseline run: `tiles-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 68121 (hoppers 42269 / furnaces 25852) |
| MSPT p50 / p90 / p99 / max, ms | 23.615 / 24.589 / 29.975 / 57.347 |
| MSPT mean, ms | 23.882 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1233.4 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.98% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.84% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 4.13% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 3.58% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 5 | 3.26% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 2.67% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 7 | 1.99% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 8 | 1.91% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.85% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 10 | 1.67% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 11 | 1.55% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 12 | 1.44% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 13 | 1.41% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 14 | 1.35% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 15 | 1.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |

