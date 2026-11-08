# JVM hotspots (JFR ExecutionSample)

Samples: 5467

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10970` |
| 2 | 6.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7107` |
| 3 | 5.34% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 4.96% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 4.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6344` |
| 6 | 3.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.07% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4764` |
| 8 | 2.73% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 2.38% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 11 | 1.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4632` |
| 12 | 1.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6415` |
| 13 | 1.37% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 1.35% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10820` |
| 15 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.77% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10832` |
| 17 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7117` |
| 19 | 0.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3401` |
| 20 | 0.57% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 21 | 0.49% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 22 | 0.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6271` |
| 23 | 0.49% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4712` |
| 24 | 0.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 25 | 0.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6401` |
| 26 | 0.42% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 27 | 0.42% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 28 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 29 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10836` |
| 30 | 0.33% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 31 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.29% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 312` |
| 33 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.27% | `java.util.ArrayList$Itr.next() line: 1058` |
| 35 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 36 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 37 | 0.26% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 427` |
| 39 | 0.24% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 40 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 41 | 0.22% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 42 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 43 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 44 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 45 | 0.20% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6262` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6448` |
| 48 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 49 | 0.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 50 | 0.16% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 51 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3359` |
| 52 | 0.16% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 411` |
| 53 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 54 | 0.15% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 55 | 0.15% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 56 | 0.15% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3353` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3375` |
| 59 | 0.15% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 60 | 0.15% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.94% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 7.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 4.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6262` |
| 6 | 4.02% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 3.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4665` |
| 8 | 2.73% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4984` |
| 9 | 2.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4529` |
| 10 | 2.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7117` |
| 11 | 1.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.87% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2638` |
| 13 | 1.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6401` |
| 14 | 1.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11052` |
| 15 | 1.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11065` |
| 16 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.68% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4663` |
| 19 | 0.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4632` |
| 20 | 0.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 21 | 0.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3380` |
| 22 | 0.48% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 23 | 0.44% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4633` |
| 25 | 0.40% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 460` |
| 26 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.37% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 28 | 0.35% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 29 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.29% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |

GC pauses: 0, total 0 ms

