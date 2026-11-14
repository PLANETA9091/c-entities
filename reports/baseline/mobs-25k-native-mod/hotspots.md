# JVM hotspots (JFR ExecutionSample)

Samples: 2103

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.90% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12263` |
| 2 | 2.90% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 4 | 1.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |
| 5 | 1.81% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12282` |
| 6 | 1.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 7 | 1.66% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 8 | 1.62% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.43% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 12 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.09% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12487` |
| 15 | 0.90% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 16 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 0.67% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 18 | 0.57% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 19 | 0.52% | `java.util.HashMap.getNode(Object) line: 586` |
| 20 | 0.52% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 21 | 0.48% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 22 | 0.48% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 23 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 24 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 25 | 0.48% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 26 | 0.43% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 27 | 0.43% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 29 | 0.38% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 30 | 0.38% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 31 | 0.38% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 32 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 33 | 0.33% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 34 | 0.33% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 35 | 0.33% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 36 | 0.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12392` |
| 37 | 0.33% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 38 | 0.33% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 39 | 0.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 40 | 0.33% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 41 | 0.29% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 42 | 0.29% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 43 | 0.29% | `java.lang.Math.multiplyHigh(long, long) line: 1406` |
| 44 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 45 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 46 | 0.29% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 47 | 0.29% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1916` |
| 48 | 0.29% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1890` |
| 49 | 0.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7786` |
| 50 | 0.24% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 51 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 52 | 0.24% | `java.util.HashMap.getNode(Object) line: 579` |
| 53 | 0.24% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 54 | 0.24% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 55 | 0.24% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 56 | 0.24% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5188` |
| 57 | 0.24% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 58 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 59 | 0.24% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1393` |
| 60 | 0.24% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.99% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12487` |
| 2 | 3.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 3 | 2.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 4 | 2.71% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10669` |
| 5 | 2.57% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.04% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10602` |
| 8 | 1.43% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 10 | 1.43% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 11 | 1.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6450` |
| 12 | 1.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10622` |
| 14 | 0.95% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 15 | 0.95% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 16 | 0.90% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 17 | 0.81% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 18 | 0.81% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 19 | 0.67% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 20 | 0.67% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10630` |
| 21 | 0.62% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5377` |
| 22 | 0.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.52% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.48% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5195` |
| 25 | 0.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 26 | 0.43% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 27 | 0.43% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 28 | 0.43% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 126` |
| 29 | 0.43% | `java.util.HashMap.containsKey(Object) line: 603` |
| 30 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12290` |

GC pauses: 0, total 0 ms

