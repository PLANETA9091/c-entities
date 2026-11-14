# JVM hotspots (JFR ExecutionSample)

Samples: 6002

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7064` |
| 2 | 5.76% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 3 | 5.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7817` |
| 4 | 5.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 4.28% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5185` |
| 6 | 3.93% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.68% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.33% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12299` |
| 10 | 2.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7011` |
| 11 | 2.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12449` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6602` |
| 13 | 1.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.65% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 17 | 0.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.57% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2447` |
| 19 | 0.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7827` |
| 20 | 0.52% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.42% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 22 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6967` |
| 23 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 25 | 0.28% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 26 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 27 | 0.27% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 28 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 30 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6971` |
| 31 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4935` |
| 32 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.25% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 34 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 35 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 36 | 0.18% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 37 | 0.18% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 38 | 0.18% | `java.util.ArrayList$Itr.next() line: 1058` |
| 39 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6819` |
| 40 | 0.18% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5184` |
| 41 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 42 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 43 | 0.17% | `java.lang.String.equals(Object) line: 1926` |
| 44 | 0.17% | `java.lang.Class.cast(Object) line: 3491` |
| 45 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 46 | 0.17% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2442` |
| 47 | 0.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 48 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 49 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 50 | 0.15% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1863` |
| 51 | 0.15% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |
| 52 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6657` |
| 53 | 0.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2LongHashTable$TableEntry.setValueRelease(long) line: 1486` |
| 54 | 0.13% | `java.lang.Math.multiplyHigh(long, long) line: 1406` |
| 55 | 0.13% | `com.mojang.serialization.Encoder$1.encode(Object, DynamicOps, Object) line: 25` |
| 56 | 0.13% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 762` |
| 57 | 0.13% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 58 | 0.13% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2438` |
| 59 | 0.13% | `java.lang.Double.isFinite(double) line: 1023` |
| 60 | 0.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7175` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.89% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 6.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 5.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6953` |
| 4 | 5.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4943` |
| 6 | 4.32% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 3.93% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5421` |
| 8 | 2.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12531` |
| 9 | 2.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 2.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4914` |
| 11 | 1.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6954` |
| 14 | 0.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7827` |
| 15 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 17 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4938` |
| 18 | 0.43% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 22 | 0.27% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 23 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7025` |
| 24 | 0.27% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 25 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 26 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4917` |
| 27 | 0.23% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |
| 28 | 0.23% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, NoiseChunk, RandomState, BiomeManager, Set, MaterialRule, ServerLevel) line: 227` |
| 29 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.22% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

