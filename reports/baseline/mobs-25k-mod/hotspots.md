# JVM hotspots (JFR ExecutionSample)

Samples: 7554

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 60.37% | `dev.planeta9091.centities.CeHooks.enchTickEffects$(ServerLevel, LivingEntity) line: 9335` |
| 2 | 6.69% | `java.lang.AbstractStringBuilder.ensureCapacityNewCoder(byte[], byte, int, int, byte) line: 287` |
| 3 | 2.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.tickEffects(ServerLevel, LivingEntity)` |
| 4 | 2.17% | `java.lang.StringBuilder.append(String) line: 179` |
| 5 | 1.72% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 1.46% | `java.util.EnumMap.get(Object) line: 248` |
| 7 | 1.14% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 8 | 0.81% | `java.lang.AbstractStringBuilder.append(String) line: 642` |
| 9 | 0.65% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 10 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.44% | `java.lang.StringUTF16.compress(byte[], int, int) line: 240` |
| 13 | 0.40% | `java.util.EnumMap.containsKey(Object) line: 223` |
| 14 | 0.33% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 15 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 16 | 0.20% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 17 | 0.20% | `java.lang.String.valueOf(Object) line: 4530` |
| 18 | 0.19% | `dev.planeta9091.centities.CeHooks.enchTickEffects$(ServerLevel, LivingEntity) line: 9319` |
| 19 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 20 | 0.17% | `org.apache.logging.log4j.core.pattern.ThrowableStackTraceRenderer.renderStackTraceElements(StringBuilder, ThrowableStackTraceRenderer$Context, ThrowableStackTraceRenderer$Context$Metadata, String, String) line: 153` |
| 21 | 0.17% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 22 | 0.17% | `java.lang.AbstractStringBuilder.append(char) line: 894` |
| 23 | 0.17% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 24 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 25 | 0.16% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 26 | 0.16% | `java.util.HashMap.getNode(Object) line: 577` |
| 27 | 0.15% | `java.lang.AbstractStringBuilder.<init>(int) line: 101` |
| 28 | 0.15% | `org.apache.logging.log4j.core.pattern.ThrowableStackTraceRenderer.renderStackTraceElements(StringBuilder, ThrowableStackTraceRenderer$Context, ThrowableStackTraceRenderer$Context$Metadata, String, String) line: 152` |
| 29 | 0.13% | `java.lang.System$1.uncheckedEncodeASCII(char[], int, byte[], int, int) line: 2158` |
| 30 | 0.12% | `java.lang.String.length() line: 1583` |
| 31 | 0.12% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 32 | 0.12% | `java.lang.AbstractStringBuilder.append(char) line: 893` |
| 33 | 0.11% | `com.google.common.collect.RegularImmutableMap$KeySet.get(int) line: 326` |
| 34 | 0.11% | `net.minecraft.core.Holder$Reference.value() line: 168` |
| 35 | 0.11% | `org.apache.logging.log4j.core.layout.TextEncoderHelper.encodeChunkedText(CharsetEncoder, CharBuffer, ByteBuffer, StringBuilder, ByteBufferDestination) line: 169` |
| 36 | 0.11% | `java.lang.StringBuilder.toString() line: 478` |
| 37 | 0.11% | `jdk.internal.util.DecimalDigits.uncheckedPutPairLatin1(byte[], int, int) line: 416` |
| 38 | 0.11% | `java.util.Map.getOrDefault(Object, Object) line: 692` |
| 39 | 0.09% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 40 | 0.09% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 41 | 0.09% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 43 | 0.09% | `java.lang.AbstractStringBuilder.append(char) line: 905` |
| 44 | 0.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 45 | 0.08% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 46 | 0.08% | `java.util.Comparator.lambda$comparingLong$6043328a$1(ToLongFunction, Object, Object) line: 515` |
| 47 | 0.08% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.lambda$setGenerationTask$0(ChunkProgressionTask, ChunkStatus, List, ChunkAccess, Throwable) line: 1622` |
| 48 | 0.08% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 49 | 0.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 50 | 0.08% | `java.lang.StringUTF16.compress(byte[], int, byte[], int, int) line: 401` |
| 51 | 0.08% | `java.lang.StackTraceElement.toString() line: 369` |
| 52 | 0.08% | `java.util.Arrays.copyOfRange(byte[], int, int) line: 3849` |
| 53 | 0.08% | `org.apache.logging.log4j.core.pattern.ThrowableExtendedStackTraceRenderer.renderStackTraceElement(StringBuilder, StackTraceElement, ThrowableStackTraceRenderer$Context, String, String) line: 34` |
| 54 | 0.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 55 | 0.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 56 | 0.07% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 57 | 0.07% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 123` |
| 58 | 0.07% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 59 | 0.07% | `net.minecraft.resources.Identifier.toString() line: 147` |
| 60 | 0.07% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1151` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 60.55% | `net.minecraft.world.item.enchantment.EnchantmentHelper.tickEffects(ServerLevel, LivingEntity)` |
| 2 | 6.69% | `java.lang.AbstractStringBuilder.append(String) line: 650` |
| 3 | 2.42% | `dev.planeta9091.centities.CeHooks.enchTickEffects$(ServerLevel, LivingEntity) line: 9335` |
| 4 | 1.92% | `java.util.HashMap.get(Object) line: 565` |
| 5 | 1.85% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 6 | 1.26% | `java.lang.StringBuilder.append(Object) line: 173` |
| 7 | 1.06% | `dev.planeta9091.centities.CeHooks.enchTickEffects$(ServerLevel, LivingEntity) line: 9324` |
| 8 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 0.81% | `java.lang.StringBuilder.append(String) line: 179` |
| 10 | 0.65% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2890` |
| 11 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 12 | 0.48% | `java.lang.String.<init>(AbstractStringBuilder, Void) line: 4867` |
| 13 | 0.45% | `java.lang.StackTraceElement.toString() line: 385` |
| 14 | 0.44% | `java.lang.StringBuilder.append(char) line: 246` |
| 15 | 0.34% | `org.apache.logging.log4j.core.pattern.ThrowableStackTraceRenderer.renderThrowable(StringBuilder, Throwable, ThrowableStackTraceRenderer$Context, Set, String, String, String) line: 101` |
| 16 | 0.33% | `java.lang.String.valueOf(Object) line: 4530` |
| 17 | 0.33% | `java.util.EnumMap.get(Object) line: 247` |
| 18 | 0.28% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.28% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 20 | 0.28% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 21 | 0.25% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.21% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 23 | 0.17% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.16% | `org.apache.logging.log4j.core.pattern.ClassResourceInfo.lambda$new$1(String, String, String, StringBuilder) line: 55` |
| 25 | 0.15% | `java.lang.StringBuilder.<init>(int) line: 119` |
| 26 | 0.15% | `java.lang.StackTraceElement.toString() line: 391` |
| 27 | 0.15% | `java.lang.StackTraceElement.of(StackTraceElement[]) line: 581` |
| 28 | 0.13% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.13% | `net.minecraft.server.MinecraftServer.lambda$spin$0(AtomicReference) line: 299` |
| 30 | 0.13% | `org.apache.logging.log4j.core.layout.TextEncoderHelper.encodeText(CharsetEncoder, CharBuffer, ByteBuffer, StringBuilder, ByteBufferDestination) line: 59` |

GC pauses: 0, total 0 ms

