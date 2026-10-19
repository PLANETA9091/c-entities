# Baseline run: `tiles-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 24964 (hoppers 15000 / furnaces 9964) |
| MSPT p50 / p90 / p99 / max, ms | 12.867 / 13.981 / 15.318 / 33.988 |
| MSPT mean, ms | 12.985 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 862.0 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.09% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 2 | 5.20% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 3 | 5.16% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 3.96% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 2.30% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 6 | 2.26% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 7 | 2.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 8 | 1.93% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 9 | 1.84% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 10 | 1.80% | `java.util.HashMap.getNode(Object) line: 585` |
| 11 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.34% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 13 | 1.20% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 14 | 1.20% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 15 | 1.20% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |

