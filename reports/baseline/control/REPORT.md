# Baseline run: `control`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 0.451 / 1.002 / 1.803 / 2.363 |
| MSPT mean, ms | 0.560 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 625.9 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.49% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 2 | 3.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 3 | 2.68% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 4 | 2.35% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 5 | 2.11% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1123` |
| 6 | 2.03% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 7 | 1.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 8 | 1.87% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 9 | 1.62% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 10 | 1.38% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 11 | 1.30% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 12 | 1.22% | `java.lang.String.hashCode() line: 2358` |
| 13 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.14% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 30` |
| 15 | 1.05% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |

