# JVM hotspots (JFR ExecutionSample)

Samples: 4963

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.85% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 5.62% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 5.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 4 | 4.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9777` |
| 5 | 3.79% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 6 | 3.04% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 1.83% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9770` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9730` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9733` |
| 11 | 1.19% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5781` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5684` |
| 14 | 1.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 15 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.93% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 17 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 19 | 0.83% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2835` |
| 21 | 0.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 22 | 0.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2873` |
| 23 | 0.48% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 24 | 0.48% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4152` |
| 25 | 0.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5489` |
| 26 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9738` |
| 27 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 28 | 0.42% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 29 | 0.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 30 | 0.36% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 31 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 32 | 0.34% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 33 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2826` |
| 34 | 0.32% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4154` |
| 35 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5581` |
| 37 | 0.28% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 38 | 0.28% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 39 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5622` |
| 40 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 41 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 42 | 0.26% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 43 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3976` |
| 44 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 45 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 46 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 47 | 0.22% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 48 | 0.22% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 49 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2825` |
| 50 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5582` |
| 51 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5685` |
| 52 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2847` |
| 53 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5625` |
| 54 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 55 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 56 | 0.20% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 57 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5488` |
| 58 | 0.20% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 59 | 0.20% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 60 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5623` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.52% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 3 | 8.85% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 5 | 4.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4105` |
| 6 | 2.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3969` |
| 7 | 2.92% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2217` |
| 8 | 2.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 1.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 11 | 1.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4103` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 14 | 0.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 16 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.89% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.66% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 19 | 0.52% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 20 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.42% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.34% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 24 | 0.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 25 | 0.32% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.28% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 28 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isJoinNonEmpty(VoxelShape, VoxelShape, BooleanOp) line: 1320` |
| 29 | 0.28% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 30 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4073` |

GC pauses: 0, total 0 ms

