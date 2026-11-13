# JVM hotspots (JFR ExecutionSample)

Samples: 5700

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 4.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7416` |
| 3 | 4.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6680` |
| 4 | 4.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.84% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4916` |
| 6 | 3.11% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12048` |
| 9 | 2.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 2.18% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11898` |
| 11 | 1.81% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 12 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6315` |
| 13 | 1.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6637` |
| 15 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.63% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 19 | 0.56% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 20 | 0.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6641` |
| 21 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6623` |
| 22 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7426` |
| 23 | 0.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 24 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 25 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 26 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3490` |
| 27 | 0.25% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 28 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 29 | 0.25% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.25% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 31 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 32 | 0.23% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 33 | 0.23% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 34 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 35 | 0.21% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 36 | 0.19% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 37 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 38 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 39 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 40 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 41 | 0.18% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 42 | 0.18% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 43 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3464` |
| 44 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6359` |
| 45 | 0.18% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4915` |
| 46 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6484` |
| 47 | 0.16% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 48 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6621` |
| 49 | 0.14% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 50 | 0.14% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 51 | 0.14% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 52 | 0.14% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 53 | 0.14% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1430` |
| 54 | 0.14% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 71` |
| 55 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6516` |
| 56 | 0.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 57 | 0.12% | `jdk.internal.classfile.impl.SplitConstantPool.tryFindClassOrInterface(int, ClassDesc) line: 427` |
| 58 | 0.12% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 59 | 0.12% | `java.lang.Double.isFinite(double) line: 1023` |
| 60 | 0.12% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 440` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.93% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6484` |
| 3 | 5.12% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 4.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6566` |
| 5 | 4.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 4.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4684` |
| 7 | 3.82% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 3.11% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5152` |
| 9 | 2.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 2.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4656` |
| 11 | 2.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12130` |
| 12 | 2.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6623` |
| 13 | 1.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.14% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2713` |
| 15 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 16 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4679` |
| 18 | 0.61% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 19 | 0.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 20 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7426` |
| 21 | 0.44% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.39% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.35% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 24 | 0.32% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 26 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.26% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 29 | 0.25% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 30 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6641` |

GC pauses: 0, total 0 ms

