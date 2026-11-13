# JVM hotspots (JFR ExecutionSample)

Samples: 5283

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.11% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 2 | 5.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11991` |
| 3 | 5.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.79% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4900` |
| 5 | 3.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7360` |
| 6 | 3.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 2.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.97% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.72% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6299` |
| 11 | 1.44% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11841` |
| 12 | 1.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6621` |
| 15 | 1.06% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11853` |
| 17 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6477` |
| 19 | 0.68% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 20 | 0.61% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 21 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 22 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3474` |
| 23 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11857` |
| 24 | 0.34% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 25 | 0.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7370` |
| 26 | 0.34% | `dev.planeta9091.centities.CeHooks.nativeMergeScan(int, double, double, double, double, double, long, long[])` |
| 27 | 0.32% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 28 | 0.30% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 29 | 0.28% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 30 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 31 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 32 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6620` |
| 33 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.25% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 36 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6605` |
| 37 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 38 | 0.21% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 39 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6607` |
| 40 | 0.21% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 41 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 42 | 0.19% | `java.lang.String.length() line: 1583` |
| 43 | 0.19% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 44 | 0.19% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 45 | 0.19% | `java.util.ArrayList$Itr.next() line: 1058` |
| 46 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6664` |
| 47 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 48 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 49 | 0.17% | `java.util.HashMap.getNode(Object) line: 586` |
| 50 | 0.17% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 51 | 0.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 441` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6650` |
| 54 | 0.15% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 55 | 0.15% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 56 | 0.15% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 57 | 0.15% | `net.minecraft.server.level.WorldGenRegion.getChunk(int, int) line: 175` |
| 58 | 0.15% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 59 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 60 | 0.13% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.11% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 2 | 6.19% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 6.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 5.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4668` |
| 6 | 4.03% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 3.96% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4640` |
| 9 | 1.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.72% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5136` |
| 11 | 1.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12086` |
| 12 | 1.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12073` |
| 13 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6607` |
| 14 | 1.02% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2697` |
| 15 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 16 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4663` |
| 18 | 0.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6550` |
| 19 | 0.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 20 | 0.55% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 21 | 0.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7370` |
| 22 | 0.44% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.44% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 24 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6468` |
| 25 | 0.38% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6650` |
| 28 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6625` |
| 29 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

