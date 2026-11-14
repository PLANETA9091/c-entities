# JVM hotspots (JFR ExecutionSample)

Samples: 5016

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.38% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 4.55% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5141` |
| 3 | 3.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7770` |
| 4 | 3.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.47% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 3.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6767` |
| 8 | 2.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.11% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12242` |
| 10 | 1.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12392` |
| 11 | 1.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7008` |
| 12 | 1.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6955` |
| 13 | 1.42% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 1.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12377` |
| 15 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6553` |
| 17 | 1.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |
| 18 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 1.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6911` |
| 20 | 1.12% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.96% | `java.util.HashMap.getNode(Object) line: 586` |
| 22 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6557` |
| 23 | 0.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4891` |
| 24 | 0.58% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6897` |
| 26 | 0.54% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.52% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 28 | 0.46% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 29 | 0.36% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 30 | 0.34% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 31 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6895` |
| 32 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.30% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 35 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 36 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6740` |
| 37 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6758` |
| 39 | 0.24% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 40 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 41 | 0.24% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1916` |
| 42 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4823` |
| 43 | 0.24% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 44 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 45 | 0.24% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 46 | 0.22% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 47 | 0.22% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.setHeadPlain(MultiThreadedQueue$LinkedNode) line: 41` |
| 48 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 49 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 50 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 51 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5140` |
| 53 | 0.18% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 54 | 0.16% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 55 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 56 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 57 | 0.16% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 58 | 0.16% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7101` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12241` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.57% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 4.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4899` |
| 3 | 4.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 4.61% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 5 | 4.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6758` |
| 6 | 3.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 3.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 3.47% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5377` |
| 9 | 2.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4870` |
| 10 | 2.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12474` |
| 11 | 1.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6840` |
| 12 | 1.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6897` |
| 14 | 1.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7770` |
| 15 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 1.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4894` |
| 17 | 0.82% | `java.util.HashMap.containsKey(Object) line: 603` |
| 18 | 0.64% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 19 | 0.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6915` |
| 20 | 0.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 21 | 0.52% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 23 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6969` |
| 24 | 0.44% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.44% | `java.util.ArrayList.grow(int) line: 238` |
| 26 | 0.42% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12487` |
| 28 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.34% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |

GC pauses: 0, total 0 ms

