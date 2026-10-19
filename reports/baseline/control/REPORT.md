# Baseline run: `control`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 0 (items 0 / stands 0 / zombies 0) (merge shrink: 1 -> 1) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 0.830 / 1.122 / 1.339 / 1.643 |
| MSPT mean, ms | 0.852 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 613.7 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 2 | 3.31% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 3 | 2.57% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 4 | 2.35% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 5 | 2.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 6 | 1.98% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 7 | 1.98% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 8 | 1.54% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 9 | 1.47% | `org.objectweb.asm.SymbolTable.addConstantUtf8(String) line: 798` |
| 10 | 1.47% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 11 | 1.40% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 12 | 1.40% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 13 | 1.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 14 | 1.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 15 | 1.25% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |

