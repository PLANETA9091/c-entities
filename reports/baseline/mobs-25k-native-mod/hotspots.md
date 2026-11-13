# JVM hotspots (JFR ExecutionSample)

Samples: 3324

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.60% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11862` |
| 2 | 5.05% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.68% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 1.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 6 | 1.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7360` |
| 8 | 1.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.38% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.26% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 1.17% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 12 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.75% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 0.69% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11881` |
| 16 | 0.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 17 | 0.57% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 18 | 0.57% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 19 | 0.57% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 20 | 0.57% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11899` |
| 21 | 0.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12086` |
| 22 | 0.54% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 23 | 0.51% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 24 | 0.51% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.51% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 26 | 0.51% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 27 | 0.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11991` |
| 28 | 0.51% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 29 | 0.45% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 30 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 31 | 0.42% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 32 | 0.39% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 33 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 34 | 0.39% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1377` |
| 35 | 0.36% | `java.util.ArrayList$Itr.next() line: 1058` |
| 36 | 0.36% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 37 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.36% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 39 | 0.33% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 40 | 0.33% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1894` |
| 41 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 42 | 0.33% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 43 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 44 | 0.30% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 45 | 0.30% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 46 | 0.30% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 47 | 0.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2894` |
| 48 | 0.27% | `java.util.Collection.stream() line: 748` |
| 49 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 51 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 52 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 53 | 0.24% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 54 | 0.24% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10252` |
| 55 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 56 | 0.21% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 57 | 0.21% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 58 | 0.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 59 | 0.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 60 | 0.21% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10272` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12086` |
| 2 | 4.99% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10269` |
| 3 | 3.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 4 | 3.19% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10202` |
| 5 | 2.59% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 10 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6207` |
| 11 | 1.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7386` |
| 12 | 1.26% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 13 | 1.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 14 | 0.90% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5136` |
| 15 | 0.90% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 16 | 0.75% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 17 | 0.69% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 18 | 0.66% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2658` |
| 20 | 0.60% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 21 | 0.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 23 | 0.51% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11887` |
| 25 | 0.45% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isJoinNonEmpty(VoxelShape, VoxelShape, BooleanOp) line: 1336` |
| 26 | 0.45% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 27 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.45% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10222` |
| 29 | 0.45% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 30 | 0.45% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |

GC pauses: 0, total 0 ms

