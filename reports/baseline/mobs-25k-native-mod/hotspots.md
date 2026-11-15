# JVM hotspots (JFR ExecutionSample)

Samples: 3294

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.92% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13003` |
| 2 | 4.04% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.22% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8335` |
| 6 | 2.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 1.58% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.24% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 0.94% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 0.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13245` |
| 13 | 0.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.73% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 15 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 16 | 0.58% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 17 | 0.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 18 | 0.58% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 19 | 0.58% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 20 | 0.55% | `java.util.HashMap.getNode(Object) line: 586` |
| 21 | 0.52% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1927` |
| 22 | 0.49% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 23 | 0.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 24 | 0.46% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 25 | 0.46% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 26 | 0.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 27 | 0.43% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 28 | 0.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 30 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 31 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 32 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 33 | 0.36% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.36% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 35 | 0.36% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 36 | 0.33% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 37 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 38 | 0.30% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 39 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 40 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 41 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 42 | 0.27% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.<init>(Object, MultiThreadedQueue$LinkedNode) line: 1250` |
| 43 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 44 | 0.27% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 45 | 0.27% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 46 | 0.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13141` |
| 47 | 0.27% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9931` |
| 48 | 0.27% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 49 | 0.27% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 50 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 51 | 0.24% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 52 | 0.24% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 53 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13022` |
| 54 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.removeEntity(Entity, int) line: 314` |
| 55 | 0.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8361` |
| 56 | 0.24% | `ca.spottedleaf.ioutil.buffer.Buffer.advanceWrite(long) line: 343` |
| 57 | 0.21% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 58 | 0.21% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 59 | 0.21% | `java.lang.Class.cast(Object) line: 3491` |
| 60 | 0.21% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11244` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13245` |
| 2 | 3.86% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11244` |
| 3 | 3.70% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11177` |
| 4 | 3.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 2.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.40% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.28% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 9 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.24% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.21% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 12 | 1.06% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 13 | 0.94% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 14 | 0.91% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6858` |
| 16 | 0.88% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5771` |
| 17 | 0.88% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 18 | 0.73% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.70% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11197` |
| 20 | 0.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8361` |
| 21 | 0.61% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 22 | 0.61% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.58% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 25 | 0.52% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.49% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.46% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isJoinNonEmpty(VoxelShape, VoxelShape, BooleanOp) line: 1336` |
| 28 | 0.46% | `java.util.HashMap.containsKey(Object) line: 603` |
| 29 | 0.43% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 30 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |

GC pauses: 0, total 0 ms

