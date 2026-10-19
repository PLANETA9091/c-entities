# Baseline run: `control`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 0.355 / 0.729 / 1.494 / 3.857 |
| MSPT mean, ms | 0.451 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 916.2 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 2 | 3.87% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 3 | 3.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 4 | 3.60% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 5 | 1.87% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 6 | 1.60% | `net.minecraft.util.Mth.lerp2(double, double, double, double, double, double) line: 545` |
| 7 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 8 | 1.34% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 9 | 1.34% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 10 | 1.34% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 225` |
| 11 | 1.07% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 12 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 13 | 1.07% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 14 | 1.07% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 15 | 1.07% | `net.minecraft.world.level.levelgen.NoiseChunk.updateForZ(int, double) line: 325` |

