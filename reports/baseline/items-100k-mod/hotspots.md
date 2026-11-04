# JVM hotspots (JFR ExecutionSample)

Samples: 4619

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.79% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 5.22% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 4.03% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10093` |
| 4 | 3.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5978` |
| 5 | 3.79% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 6 | 3.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 1.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 9 | 1.73% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10086` |
| 10 | 1.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6049` |
| 11 | 1.71% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10049` |
| 12 | 1.60% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10046` |
| 13 | 1.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |
| 15 | 1.08% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 16 | 0.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6751` |
| 17 | 0.95% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 18 | 0.95% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 19 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5952` |
| 21 | 0.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3136` |
| 22 | 0.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6035` |
| 23 | 0.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 24 | 0.54% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4415` |
| 25 | 0.52% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 26 | 0.50% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 27 | 0.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5849` |
| 28 | 0.45% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 29 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10054` |
| 30 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5756` |
| 31 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5757` |
| 32 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 33 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 34 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 35 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3089` |
| 36 | 0.30% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 37 | 0.30% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 38 | 0.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10269` |
| 39 | 0.28% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 40 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 41 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5928` |
| 42 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 43 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5961` |
| 44 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 45 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 46 | 0.22% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 47 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5953` |
| 48 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 49 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 50 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 51 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 52 | 0.19% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 53 | 0.19% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5891` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6031` |
| 57 | 0.19% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4411` |
| 58 | 0.19% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 59 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5936` |
| 60 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.73% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10223` |
| 3 | 7.82% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4368` |
| 5 | 4.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |
| 6 | 3.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4232` |
| 7 | 3.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 8 | 2.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6751` |
| 9 | 2.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 2.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 2.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4366` |
| 13 | 1.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 14 | 1.08% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 15 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6035` |
| 16 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.65% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.45% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.45% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 21 | 0.43% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 23 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 24 | 0.35% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.35% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 26 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.30% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 29 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6053` |
| 30 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4336` |

GC pauses: 0, total 0 ms

