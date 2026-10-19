# Baseline run: `control`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 1.029 / 1.377 / 1.669 / 1.822 |
| MSPT mean, ms | 1.079 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 646.2 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 2 | 3.26% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 3 | 2.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 4 | 2.42% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 5 | 2.42% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 6 | 2.04% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 7 | 1.74% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 8 | 1.59% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 9 | 1.44% | `java.lang.String.hashCode() line: 2358` |
| 10 | 1.29% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 11 | 1.21% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 12 | 1.14% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 13 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 14 | 1.06% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 15 | 0.91% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |

