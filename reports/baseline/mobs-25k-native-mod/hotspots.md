# JVM hotspots (JFR ExecutionSample)

Samples: 3409

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.87% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16623` |
| 2 | 4.75% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13967` |
| 3 | 2.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16901` |
| 4 | 2.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.08% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15343` |
| 6 | 2.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10813` |
| 8 | 1.73% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 9 | 1.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.47% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 1.44% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 12 | 0.88% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.79% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 0.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.65% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 17 | 0.65% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 18 | 0.59% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 19 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 20 | 0.59% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 21 | 0.59% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 22 | 0.56% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 23 | 0.56% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 24 | 0.53% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 25 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.47% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 27 | 0.47% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 28 | 0.44% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.44% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 30 | 0.41% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 31 | 0.41% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 32 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 33 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 35 | 0.32% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 36 | 0.32% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 37 | 0.32% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 38 | 0.29% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 39 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 40 | 0.29% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 41 | 0.29% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 42 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 934` |
| 43 | 0.29% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2754` |
| 44 | 0.29% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 45 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 46 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 47 | 0.26% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 48 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 49 | 0.26% | `java.util.HashMap.getNode(Object) line: 586` |
| 50 | 0.23% | `it.unimi.dsi.fastutil.ints.Int2IntOpenHashMap.remove(int) line: 347` |
| 51 | 0.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 52 | 0.21% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 53 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 765` |
| 54 | 0.21% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15360` |
| 55 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 56 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 954` |
| 57 | 0.18% | `java.util.HashMap.getNode(Object) line: 588` |
| 58 | 0.18% | `jdk.internal.misc.Unsafe.allocateInstance(Class)` |
| 59 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getCurrentGenStatus() line: 1328` |
| 60 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5188` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8553` |
| 2 | 5.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16901` |
| 3 | 2.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 2.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 2.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.32% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13878` |
| 8 | 2.17% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 10 | 1.73% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16627` |
| 11 | 1.53% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.53% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 13 | 1.47% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 14 | 0.79% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 15 | 0.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10908` |
| 16 | 0.76% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 17 | 0.65% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.65% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15351` |
| 19 | 0.59% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 20 | 0.56% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 21 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.53% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.47% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.47% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13906` |
| 25 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16630` |
| 26 | 0.44% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 27 | 0.44% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2207` |
| 28 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 29 | 0.41% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 30 | 0.41% | `net.minecraft.world.entity.Entity.isPassenger() line: 3733` |

GC pauses: 0, total 0 ms

