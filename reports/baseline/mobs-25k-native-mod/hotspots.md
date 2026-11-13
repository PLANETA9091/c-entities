# JVM hotspots (JFR ExecutionSample)

Samples: 3432

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11976` |
| 2 | 3.90% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.26% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7473` |
| 5 | 2.07% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 1.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 1.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.96% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 13 | 0.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12200` |
| 14 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11995` |
| 15 | 0.55% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 16 | 0.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.52% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1910` |
| 18 | 0.52% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 0.50% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 20 | 0.50% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 21 | 0.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 22 | 0.47% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 23 | 0.47% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.47% | `java.util.HashMap.getNode(Object) line: 586` |
| 25 | 0.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12105` |
| 26 | 0.44% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 27 | 0.44% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 28 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 29 | 0.41% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 30 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 31 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 32 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.38% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 34 | 0.38% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 35 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 36 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 37 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 39 | 0.32% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 40 | 0.32% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 41 | 0.32% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 42 | 0.29% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 43 | 0.29% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 44 | 0.29% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 45 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 46 | 0.26% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 47 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 48 | 0.26% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 49 | 0.26% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10365` |
| 50 | 0.26% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5200` |
| 51 | 0.26% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10385` |
| 52 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 53 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 54 | 0.23% | `java.util.ArrayList$Itr.next() line: 1058` |
| 55 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getCurrentGenStatus() line: 1328` |
| 57 | 0.23% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 58 | 0.23% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 59 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 60 | 0.20% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9069` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12200` |
| 2 | 3.79% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10382` |
| 3 | 3.76% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10315` |
| 4 | 3.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 2.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.19% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 1.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.49% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5152` |
| 10 | 1.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.40% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 12 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6223` |
| 13 | 1.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7499` |
| 15 | 0.90% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 16 | 0.76% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 17 | 0.70% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.61% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.58% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10335` |
| 20 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 21 | 0.52% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 22 | 0.52% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.50% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 25 | 0.47% | `java.util.ArrayList.grow(int) line: 238` |
| 26 | 0.44% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 27 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.44% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 29 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.41% | `java.util.HashMap.containsKey(Object) line: 603` |

GC pauses: 0, total 0 ms

