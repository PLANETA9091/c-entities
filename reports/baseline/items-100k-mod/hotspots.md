# JVM hotspots (JFR ExecutionSample)

Samples: 4194

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10231` |
| 2 | 7.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9023` |
| 3 | 5.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.12% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.05% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 6 | 2.65% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 7 | 1.67% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14783` |
| 8 | 1.48% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14569` |
| 9 | 1.41% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6541` |
| 10 | 1.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 11 | 1.36% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.98% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.95% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 15 | 0.95% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 16 | 0.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8784` |
| 17 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 18 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 19 | 0.60% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1022` |
| 20 | 0.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8508` |
| 21 | 0.55% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3162` |
| 22 | 0.52% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 23 | 0.52% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6550` |
| 24 | 0.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15702` |
| 25 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9204` |
| 26 | 0.36% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 27 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 28 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.36% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3158` |
| 30 | 0.36% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3157` |
| 31 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.33% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 33 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8778` |
| 34 | 0.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 35 | 0.26% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.compareExchangeNextVolatile(MultiThreadedQueue$LinkedNode, MultiThreadedQueue$LinkedNode) line: 1319` |
| 36 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 37 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 38 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getListenerRegistry(int) line: 272` |
| 39 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatchWalk(ItemEntity, int) line: 6128` |
| 40 | 0.24% | `java.util.ArrayList$Itr.next() line: 1058` |
| 41 | 0.21% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 42 | 0.21% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 43 | 0.21% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 46 | 0.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15700` |
| 47 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 48 | 0.19% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 49 | 0.19% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 50 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 51 | 0.19% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 52 | 0.19% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 53 | 0.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 54 | 0.19% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1430` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1041` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int) line: 6212` |
| 57 | 0.17% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 58 | 0.17% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 59 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8758` |
| 60 | 0.17% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.87% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 10.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 5.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 5.60% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 5 | 2.98% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int) line: 6160` |
| 6 | 2.12% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int) line: 6276` |
| 7 | 1.96% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 8 | 1.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 1.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15756` |
| 10 | 1.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 11 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 1.24% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14561` |
| 13 | 1.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8762` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8758` |
| 15 | 1.07% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1049` |
| 16 | 0.98% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.95% | `dev.planeta9091.centities.CeHooks.syncVelRead(Entity) line: 15350` |
| 18 | 0.93% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 19 | 0.91% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 20 | 0.88% | `dev.planeta9091.centities.CeHooks.itemDispatchWalk(ItemEntity, int) line: 6128` |
| 21 | 0.55% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.55% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int) line: 6189` |
| 23 | 0.55% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1038` |
| 24 | 0.52% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 25 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.36% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 28 | 0.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 29 | 0.31% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 30 | 0.31% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

