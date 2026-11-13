# JVM hotspots (JFR ExecutionSample)

Samples: 3929

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.85% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 5.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11991` |
| 3 | 5.37% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4900` |
| 4 | 3.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7360` |
| 6 | 3.33% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6477` |
| 9 | 2.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6299` |
| 10 | 2.14% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.68% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11841` |
| 12 | 1.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6303` |
| 13 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.94% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.79% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 17 | 0.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6621` |
| 18 | 0.71% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11853` |
| 19 | 0.64% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 20 | 0.53% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 21 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 22 | 0.41% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 23 | 0.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7370` |
| 24 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 25 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 26 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6468` |
| 27 | 0.38% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 28 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6664` |
| 30 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11857` |
| 31 | 0.31% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 32 | 0.28% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 33 | 0.28% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 34 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 35 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 36 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 37 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 38 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 39 | 0.23% | `java.util.ArrayList$Itr.next() line: 1058` |
| 40 | 0.23% | `java.lang.Class.cast(Object) line: 3491` |
| 41 | 0.23% | `dev.planeta9091.centities.CeHooks.nativeMergeScan(int, double, double, double, double, double, long, long[])` |
| 42 | 0.20% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 43 | 0.20% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 44 | 0.20% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 45 | 0.20% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6343` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3474` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3448` |
| 49 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4660` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11862` |
| 51 | 0.18% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 52 | 0.18% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6605` |
| 54 | 0.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6620` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6500` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6451` |
| 58 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 59 | 0.15% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 60 | 0.15% | `java.util.HashMap.getNode(Object) line: 586` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.05% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6468` |
| 3 | 5.93% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 5.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4668` |
| 5 | 3.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 3.05% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4640` |
| 9 | 2.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6550` |
| 10 | 2.14% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5136` |
| 11 | 1.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4663` |
| 13 | 1.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12073` |
| 14 | 1.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12086` |
| 15 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.97% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2697` |
| 17 | 0.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 18 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7370` |
| 21 | 0.48% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.43% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 23 | 0.41% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6607` |
| 25 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.38% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 27 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.31% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isJoinNonEmpty(VoxelShape, VoxelShape, BooleanOp) line: 1320` |
| 29 | 0.31% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 30 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |

GC pauses: 0, total 0 ms

