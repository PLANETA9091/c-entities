# JVM hotspots (JFR ExecutionSample)

Samples: 5297

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6967` |
| 2 | 6.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10830` |
| 3 | 5.63% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 3.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6204` |
| 6 | 3.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 2.74% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4624` |
| 8 | 2.32% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 2.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6275` |
| 11 | 1.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4492` |
| 13 | 1.15% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6131` |
| 15 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.89% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10680` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3289` |
| 18 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3250` |
| 20 | 0.70% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 21 | 0.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6977` |
| 22 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10692` |
| 23 | 0.53% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4572` |
| 24 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 25 | 0.49% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 26 | 0.43% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 27 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 28 | 0.34% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 29 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.32% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 31 | 0.32% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 32 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 33 | 0.30% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 34 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4485` |
| 35 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 37 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 38 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 39 | 0.25% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 40 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 41 | 0.25% | `java.util.ArrayList$Itr.next() line: 1058` |
| 42 | 0.23% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 43 | 0.21% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 44 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 45 | 0.21% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.compareExchangeNextVolatile(MultiThreadedQueue$LinkedNode, MultiThreadedQueue$LinkedNode) line: 1319` |
| 46 | 0.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 47 | 0.19% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 48 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 49 | 0.17% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6261` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3247` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6025` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2797` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4378` |
| 55 | 0.17% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 56 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 57 | 0.15% | `net.minecraft.world.level.chunk.LevelChunk.getListenerRegistry(int) line: 272` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10696` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 410` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4567` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.87% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 6.25% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 5.63% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.00% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 3.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6122` |
| 7 | 3.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4525` |
| 8 | 2.74% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2597` |
| 9 | 2.30% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4844` |
| 10 | 2.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6977` |
| 11 | 1.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4389` |
| 13 | 1.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6261` |
| 14 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 1.04% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4523` |
| 16 | 0.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10912` |
| 17 | 0.93% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10925` |
| 18 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4492` |
| 20 | 0.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 21 | 0.49% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4493` |
| 23 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3268` |
| 24 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.36% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 26 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 27 | 0.34% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 28 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 266` |
| 29 | 0.32% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 30 | 0.30% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |

GC pauses: 0, total 0 ms

