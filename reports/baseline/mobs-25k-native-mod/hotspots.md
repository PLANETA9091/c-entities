# JVM hotspots (JFR ExecutionSample)

Samples: 3417

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.18% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 4.98% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11849` |
| 3 | 3.04% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 2.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7347` |
| 6 | 1.81% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.20% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 12 | 0.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 0.82% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 0.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12073` |
| 15 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 16 | 0.59% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 17 | 0.56% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 18 | 0.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 19 | 0.56% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 20 | 0.56% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.53% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 604` |
| 22 | 0.53% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 23 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 24 | 0.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 0.50% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 26 | 0.47% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 27 | 0.44% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 28 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 29 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 30 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11868` |
| 31 | 0.44% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1892` |
| 32 | 0.44% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1377` |
| 33 | 0.41% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.41% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 36 | 0.38% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 37 | 0.38% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 38 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 39 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 40 | 0.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2949` |
| 41 | 0.32% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 42 | 0.32% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 43 | 0.32% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 44 | 0.32% | `java.util.ArrayList$Itr.next() line: 1058` |
| 45 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 46 | 0.29% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10259` |
| 47 | 0.26% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 48 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 49 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 50 | 0.26% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 51 | 0.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11978` |
| 52 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 53 | 0.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2881` |
| 54 | 0.26% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 55 | 0.23% | `com.mojang.serialization.Encoder$1.encode(Object, DynamicOps, Object) line: 25` |
| 56 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 740` |
| 57 | 0.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 58 | 0.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11886` |
| 59 | 0.23% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 60 | 0.20% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12073` |
| 2 | 5.09% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10256` |
| 3 | 3.60% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10189` |
| 4 | 3.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 3.22% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 2.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 10 | 1.52% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5123` |
| 11 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6194` |
| 12 | 1.20% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 13 | 1.05% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 14 | 1.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7373` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2645` |
| 16 | 0.79% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 17 | 0.73% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 18 | 0.67% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 19 | 0.67% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 20 | 0.59% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 21 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.56% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 23 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getChunkIfPresentUnchecked(ChunkStatus) line: 475` |
| 24 | 0.53% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.50% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 27 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 28 | 0.47% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 29 | 0.44% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

