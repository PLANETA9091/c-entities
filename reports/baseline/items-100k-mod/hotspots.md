# JVM hotspots (JFR ExecutionSample)

Samples: 4943

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.21% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 5.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10094` |
| 3 | 4.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5978` |
| 4 | 4.65% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 3.99% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 6 | 3.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 2.67% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10047` |
| 9 | 1.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6049` |
| 10 | 1.58% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10050` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10087` |
| 12 | 1.36% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 14 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |
| 16 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 1.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5952` |
| 18 | 0.99% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3136` |
| 19 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 20 | 0.75% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 21 | 0.75% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.73% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4415` |
| 23 | 0.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 24 | 0.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6751` |
| 25 | 0.57% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10055` |
| 26 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6035` |
| 27 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 28 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5757` |
| 29 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 30 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 31 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5756` |
| 32 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3089` |
| 33 | 0.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10270` |
| 34 | 0.30% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 35 | 0.30% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 36 | 0.30% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 37 | 0.28% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 38 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 39 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 40 | 0.24% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.24% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 42 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5849` |
| 43 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 44 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3110` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4221` |
| 47 | 0.20% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 48 | 0.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4410` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3088` |
| 50 | 0.18% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5936` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6048` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5890` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5975` |
| 56 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 57 | 0.16% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 58 | 0.16% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5961` |
| 60 | 0.16% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.20% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10224` |
| 3 | 8.21% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.11% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4368` |
| 5 | 3.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |
| 6 | 3.05% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 2.99% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 8 | 2.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4232` |
| 9 | 2.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6751` |
| 11 | 1.70% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.36% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4366` |
| 14 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 0.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 16 | 0.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6035` |
| 18 | 0.49% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.47% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.47% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 21 | 0.42% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 22 | 0.40% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 25 | 0.30% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4336` |
| 27 | 0.26% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 28 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.24% | `net.minecraft.world.level.chunk.status.ChunkStatusTasks.lambda$buildTerrain$0(ChunkAccess) line: 130` |
| 30 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

