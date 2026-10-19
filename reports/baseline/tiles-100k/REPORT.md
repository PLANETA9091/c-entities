# Baseline run: `tiles-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 68121 (hoppers 42269 / furnaces 25852) |
| MSPT p50 / p90 / p99 / max, ms | 21.792 / 22.675 / 30.946 / 47.110 |
| MSPT mean, ms | 22.144 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1531.1 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.32% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.47% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 3 | 4.87% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 4.25% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 5 | 3.72% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 2.96% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 7 | 2.60% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 8 | 2.47% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 9 | 2.11% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 2.07% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.88% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 12 | 1.58% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 13 | 1.58% | `java.util.HashMap.getNode(Object) line: 585` |
| 14 | 1.45% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 15 | 1.28% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |

