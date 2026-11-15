# JVM hotspots (JFR ExecutionSample)

Samples: 2827

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.98% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 4.92% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12989` |
| 3 | 4.07% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8321` |
| 5 | 2.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 2.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.80% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.80% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 9 | 1.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.06% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 12 | 1.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.99% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 0.99% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 0.92% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 16 | 0.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.74% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 18 | 0.74% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 19 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13008` |
| 20 | 0.64% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 21 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 22 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 23 | 0.53% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 24 | 0.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 25 | 0.50% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 26 | 0.50% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1927` |
| 27 | 0.46% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 28 | 0.42% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.compareExchangeNextVolatile(MultiThreadedQueue$LinkedNode, MultiThreadedQueue$LinkedNode) line: 1319` |
| 29 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 30 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 31 | 0.39% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 32 | 0.39% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 33 | 0.35% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 34 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 35 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13026` |
| 36 | 0.32% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 37 | 0.32% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 38 | 0.32% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 39 | 0.32% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 40 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.28% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 42 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 43 | 0.28% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 44 | 0.28% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11233` |
| 45 | 0.25% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 46 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 47 | 0.25% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 48 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 49 | 0.25% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 50 | 0.25% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 51 | 0.25% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1907` |
| 52 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 53 | 0.21% | `java.util.HashMap.putVal(int, Object, Object, boolean, boolean) line: 647` |
| 54 | 0.21% | `net.minecraft.world.level.biome.Climate$RTree.buildParameterSpace(List) line: 455` |
| 55 | 0.21% | `java.util.ArrayList$Itr.next() line: 1058` |
| 56 | 0.21% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 57 | 0.21% | `java.lang.Class.cast(Object) line: 3491` |
| 58 | 0.21% | `java.util.ArrayList.isEmpty() line: 263` |
| 59 | 0.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13127` |
| 60 | 0.21% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13231` |
| 2 | 5.70% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11230` |
| 3 | 5.06% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11163` |
| 4 | 3.08% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 5 | 3.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 7 | 2.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.19% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.80% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 10 | 1.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.38% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5771` |
| 12 | 1.17% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 13 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6858` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8347` |
| 15 | 1.06% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 16 | 0.96% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 17 | 0.96% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 18 | 0.92% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 19 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13014` |
| 20 | 0.60% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.60% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 22 | 0.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 23 | 0.50% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |
| 26 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 27 | 0.42% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.42% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.appendList(MultiThreadedQueue$LinkedNode, MultiThreadedQueue$LinkedNode) line: 977` |
| 29 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 30 | 0.42% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11183` |

GC pauses: 0, total 0 ms

