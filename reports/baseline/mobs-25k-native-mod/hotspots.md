# JVM hotspots (JFR ExecutionSample)

Samples: 3161

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12311` |
| 2 | 3.67% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.82% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7808` |
| 6 | 1.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 7 | 1.58% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.23% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 1.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 0.89% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.66% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12535` |
| 14 | 0.66% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 0.63% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 16 | 0.63% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 17 | 0.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 18 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 19 | 0.54% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 20 | 0.54% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 21 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12330` |
| 22 | 0.47% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 23 | 0.47% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 24 | 0.47% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 25 | 0.47% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 26 | 0.44% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 27 | 0.44% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 28 | 0.41% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 29 | 0.41% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 30 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 31 | 0.41% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 32 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 33 | 0.38% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 34 | 0.38% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1890` |
| 35 | 0.38% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 36 | 0.35% | `java.util.HashMap.getNode(Object) line: 586` |
| 37 | 0.35% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 38 | 0.32% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 39 | 0.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12440` |
| 40 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 41 | 0.28% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 42 | 0.28% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 43 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 44 | 0.28% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 45 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 46 | 0.25% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 47 | 0.25% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9404` |
| 48 | 0.25% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1910` |
| 49 | 0.25% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12302` |
| 50 | 0.22% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 51 | 0.22% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.setTailPlain(MultiThreadedQueue$LinkedNode) line: 66` |
| 52 | 0.22% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 53 | 0.22% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 54 | 0.22% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 55 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 56 | 0.19% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 57 | 0.19% | `java.util.HashMap.getNode(Object) line: 588` |
| 58 | 0.19% | `jdk.internal.classfile.impl.EntryMap.firstToken(int) line: 92` |
| 59 | 0.19% | `java.lang.String.equals(Object) line: 1926` |
| 60 | 0.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12535` |
| 2 | 3.57% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10717` |
| 3 | 3.51% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10650` |
| 4 | 3.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 2.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.46% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 9 | 1.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6494` |
| 10 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 1.20% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 12 | 1.17% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 13 | 1.01% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 14 | 0.92% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 15 | 0.89% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 16 | 0.85% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5421` |
| 17 | 0.82% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.73% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.73% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 20 | 0.73% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10670` |
| 21 | 0.63% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.57% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 23 | 0.54% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 24 | 0.54% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 25 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 26 | 0.51% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.51% | `java.util.ArrayList.grow(int) line: 238` |
| 28 | 0.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7834` |
| 29 | 0.44% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 30 | 0.41% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |

GC pauses: 0, total 0 ms

