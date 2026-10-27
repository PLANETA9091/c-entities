# Baseline run: `tiles-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 68121 (hoppers 42269 / furnaces 25852) |
| MSPT p50 / p90 / p99 / max, ms | 22.980 / 24.015 / 30.465 / 49.699 |
| MSPT mean, ms | 23.226 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1679.6 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.30% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.02% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 3 | 4.13% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 4 | 3.72% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 5 | 3.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 2.86% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 7 | 2.30% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 2.10% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 9 | 2.07% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 10 | 1.98% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 11 | 1.71% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 12 | 1.62% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 13 | 1.39% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 1.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 15 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

