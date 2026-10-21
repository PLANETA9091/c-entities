# Baseline run: `tiles-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 68121 (hoppers 42269 / furnaces 25852) |
| MSPT p50 / p90 / p99 / max, ms | 20.227 / 22.681 / 28.184 / 47.807 |
| MSPT mean, ms | 20.731 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1633.9 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.08% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.12% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 3.70% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 4 | 3.26% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 5 | 2.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 2.81% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 7 | 2.31% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 1.74% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 9 | 1.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 10 | 1.52% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 11 | 1.49% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 12 | 1.42% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 13 | 1.39% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 14 | 1.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 15 | 1.33% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |

