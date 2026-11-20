# JVM hotspots (JFR ExecutionSample)

Samples: 2335

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.81% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15127` |
| 2 | 2.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15404` |
| 3 | 2.44% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 4 | 2.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 5 | 1.93% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 1.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 7 | 1.67% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9932` |
| 10 | 1.37% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 11 | 1.33% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 12 | 1.33% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 13 | 1.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.77% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 16 | 0.64% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14282` |
| 17 | 0.60% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 18 | 0.56% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 19 | 0.56% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 20 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 21 | 0.51% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 22 | 0.47% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 23 | 0.47% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 24 | 0.47% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 25 | 0.43% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.43% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 27 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 28 | 0.43% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 29 | 0.39% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 30 | 0.39% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.compareExchangeNextVolatile(MultiThreadedQueue$LinkedNode, MultiThreadedQueue$LinkedNode) line: 1319` |
| 31 | 0.39% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 32 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.39% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 34 | 0.39% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 35 | 0.34% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 36 | 0.34% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 37 | 0.34% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12907` |
| 38 | 0.34% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 39 | 0.34% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 40 | 0.34% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 41 | 0.34% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2511` |
| 42 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 43 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 44 | 0.30% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 45 | 0.30% | `java.lang.String.hashCode() line: 2433` |
| 46 | 0.30% | `java.util.Objects.equals(Object, Object) line: 62` |
| 47 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 48 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 49 | 0.30% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2505` |
| 50 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 51 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 52 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 53 | 0.26% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 54 | 0.26% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 55 | 0.26% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |
| 56 | 0.26% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 57 | 0.21% | `java.math.BigInteger.montReduce(int[], int[], int, int) line: 3293` |
| 58 | 0.21% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 59 | 0.21% | `java.util.jar.JarFile.getEntry(String) line: 501` |
| 60 | 0.21% | `java.lang.Class.cast(Object) line: 3492` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15404` |
| 2 | 3.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 3 | 2.78% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 5 | 2.06% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.88% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12857` |
| 7 | 1.88% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12924` |
| 8 | 1.84% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12877` |
| 9 | 1.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.63% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 11 | 1.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.37% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12885` |
| 14 | 1.24% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 15 | 1.11% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 16 | 0.99% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 17 | 0.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7917` |
| 18 | 0.90% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.90% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 20 | 0.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9947` |
| 21 | 0.69% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.60% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.56% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.51% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 25 | 0.47% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.light(LightChunkGetter, ChunkAccess, Boolean[]) line: 914` |
| 26 | 0.43% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 27 | 0.43% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isJoinNonEmpty(VoxelShape, VoxelShape, BooleanOp) line: 1320` |
| 28 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15130` |
| 29 | 0.43% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 30 | 0.39% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |

GC pauses: 0, total 0 ms

