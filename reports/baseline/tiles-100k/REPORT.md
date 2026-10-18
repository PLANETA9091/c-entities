# Baseline run: `tiles-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) |
| block entities | 68121 (hoppers 42269 / furnaces 25852) |
| MSPT p50 / p90 / p99 / max, ms | 27.634 / 28.697 / 30.896 / 51.152 |
| MSPT mean, ms | 27.772 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1041.7 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.40% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.isRemoved() line: 956` |
| 2 | 7.67% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 3 | 7.25% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 4.46% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 4.35% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 2.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 2.03% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 8 | 2.00% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 9 | 1.86% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 10 | 1.81% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 11 | 1.61% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 12 | 1.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 13 | 1.47% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 14 | 1.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 15 | 1.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |

