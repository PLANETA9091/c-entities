# JVM hotspots (JFR ExecutionSample)

Samples: 1775

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 2 | 2.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 3 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 4 | 0.79% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 5 | 0.79% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 6 | 0.79% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 7 | 0.68% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 8 | 0.62% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 9 | 0.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 10 | 0.56% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 11 | 0.51% | `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock.unlock(ReentrantAreaLock$Node) line: 266` |
| 12 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 13 | 0.45% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 14 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getCurrentGenStatus() line: 1322` |
| 15 | 0.39% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 16 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 17 | 0.39% | `java.lang.String.hashCode() line: 2433` |
| 18 | 0.39% | `java.util.HashMap.getNode(Object) line: 586` |
| 19 | 0.34% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 20 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 21 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 23 | 0.28% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 24 | 0.28% | `jdk.internal.misc.Unsafe.allocateInstance(Class)` |
| 25 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 27 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 28 | 0.28% | `net.minecraft.world.level.chunk.storage.SerializableChunkData.copyOf(ServerLevel, ChunkAccess) line: 445` |
| 29 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 30 | 0.28% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 31 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 32 | 0.23% | `sun.security.provider.SHA2.implCompress0(byte[], int) line: 193` |
| 33 | 0.23% | `java.util.Objects.equals(Object, Object) line: 62` |
| 34 | 0.23% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.23% | `java.util.Objects.hashCode(Object) line: 97` |
| 36 | 0.23% | `net.minecraft.util.parsing.packrat.CachedParseState$PositionCache.findKeyIndex(Atom) line: 125` |
| 37 | 0.23% | `net.minecraft.world.level.biome.Climate$RTree.lambda$comparator$0(int, boolean, Climate$RTree$Node) line: 405` |
| 38 | 0.23% | `java.math.BigInteger.montReduce(int[], int[], int, int) line: 3290` |
| 39 | 0.23% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1863` |
| 40 | 0.23% | `java.util.Collection.stream() line: 748` |
| 41 | 0.23% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 42 | 0.23% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.getBlockState(int, int, int) line: 299` |
| 43 | 0.23% | `jdk.internal.foreign.HeapMemorySegmentImpl$OfByte.unsafeGetBase() line: 114` |
| 44 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.remove(Object) line: 489` |
| 45 | 0.23% | `jdk.internal.util.DecimalDigits.uncheckedGetCharsLatin1(int, int, byte[]) line: 168` |
| 46 | 0.23% | `java.lang.Class.getEnumConstantsShared() line: 3436` |
| 47 | 0.23% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1024` |
| 48 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 49 | 0.17% | `java.lang.invoke.VarHandleByteArrayAsInts$ArrayHandle.get(VarHandle, Object, int) line: 108` |
| 50 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 692` |
| 51 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 52 | 0.17% | `paperclip.libs.io.sigpipe.jbsdiff.Patch.patch(byte[], byte[], OutputStream) line: 94` |
| 53 | 0.17% | `java.lang.StringBuilder.<init>() line: 106` |
| 54 | 0.17% | `java.util.Map.computeIfAbsent(Object, Function) line: 1067` |
| 55 | 0.17% | `java.lang.StringLatin1.replace(byte[], char, char) line: 311` |
| 56 | 0.17% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1873` |
| 57 | 0.17% | `java.util.HashMap.getNode(Object) line: 579` |
| 58 | 0.17% | `java.util.Objects.requireNonNull(Object) line: 220` |
| 59 | 0.17% | `net.minecraft.world.phys.shapes.DiscreteVoxelShape.forAllBoxes(DiscreteVoxelShape$IntLineConsumer, boolean) line: 284` |
| 60 | 0.17% | `net.minecraft.world.phys.shapes.VoxelShape.moonrise$isFullBlock() line: 399` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 2 | 2.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 3 | 1.07% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 4 | 0.85% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 5 | 0.85% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 6 | 0.85% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 7 | 0.68% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 126` |
| 8 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.saveChunk(ChunkAccess, boolean, Completable[]) line: 1719` |
| 9 | 0.56% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 885` |
| 11 | 0.51% | `net.minecraft.world.level.chunk.status.ChunkStatusTasks.lambda$buildTerrain$0(ChunkAccess) line: 130` |
| 12 | 0.51% | `net.minecraft.world.level.levelgen.synth.NoiseStack$SmearedPerlin.addToVolume(DensityBuffer, DensityVolume, double, double, float) line: 153` |
| 13 | 0.51% | `net.minecraft.server.MinecraftServer.lambda$spin$0(AtomicReference) line: 299` |
| 14 | 0.45% | `java.lang.String.replace(char, char) line: 2998` |
| 15 | 0.45% | `java.util.HashMap.get(Object) line: 565` |
| 16 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 17 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.light(LightChunkGetter, ChunkAccess, Boolean[]) line: 914` |
| 18 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 19 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 788` |
| 20 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.lambda$setGenerationTask$0(ChunkProgressionTask, ChunkStatus, List, ChunkAccess, Throwable) line: 1620` |
| 21 | 0.39% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 213` |
| 22 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.34% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.lambda$buildTerrain$1(ChunkAccess, StructureManager, Blender, RandomState, NoiseSettings, CustomChunkGenerator, BiomeManager, Set, WorldGenRegion) line: 412` |
| 24 | 0.34% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, NoiseChunk, RandomState, BiomeManager, Set, MaterialRule, ServerLevel) line: 227` |
| 25 | 0.34% | `net.minecraft.world.level.chunk.PalettedContainer.pack(Strategy) line: 364` |
| 26 | 0.34% | `net.minecraft.server.MinecraftServer.tickServer(BooleanSupplier) line: 1604` |
| 27 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |
| 28 | 0.28% | `sun.security.provider.SHA2.implCompress(byte[], int) line: 135` |
| 29 | 0.28% | `io.papermc.paperclip.PatchEntry.applyPatch(Map, Path, Path) line: 136` |
| 30 | 0.28% | `net.minecraft.world.phys.shapes.Shapes.joinIsNotEmpty(VoxelShape, VoxelShape, BooleanOp) line: 227` |

GC pauses: 0, total 0 ms

