# Baseline run: `tiles-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 24964 (hoppers 15000 / furnaces 9964) |
| MSPT p50 / p90 / p99 / max, ms | 9.382 / 10.852 / 15.520 / 32.317 |
| MSPT mean, ms | 9.712 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 605.3 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 2 | 4.03% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 3 | 3.00% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 4 | 2.75% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 5 | 2.36% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 2.21% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 7 | 2.21% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 8 | 2.06% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 9 | 1.72% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 10 | 1.67% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 11 | 1.67% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 12 | 1.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.43% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 14 | 1.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 1.13% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |

