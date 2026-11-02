# JVM hotspots (JFR ExecutionSample)

Samples: 4948

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5537` |
| 2 | 6.97% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 4.75% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9598` |
| 4 | 4.69% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4071` |
| 5 | 2.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 2.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9591` |
| 8 | 1.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9551` |
| 9 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5542` |
| 10 | 1.27% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.17% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5613` |
| 13 | 1.11% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 1.11% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9554` |
| 15 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 1.03% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3944` |
| 18 | 0.89% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6305` |
| 19 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2777` |
| 20 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |
| 21 | 0.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6315` |
| 22 | 0.77% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4024` |
| 23 | 0.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5598` |
| 24 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9559` |
| 25 | 0.40% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 26 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5442` |
| 27 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 28 | 0.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9774` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 30 | 0.32% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 31 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 32 | 0.30% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 33 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5484` |
| 36 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2729` |
| 39 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5354` |
| 40 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 41 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 42 | 0.24% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 43 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 44 | 0.22% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 45 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2751` |
| 46 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5594` |
| 47 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 48 | 0.20% | `net.minecraft.server.level.WorldGenRegion.getChunk(int, int) line: 175` |
| 49 | 0.20% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 50 | 0.20% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 51 | 0.20% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4026` |
| 52 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 53 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 54 | 0.18% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 393` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5646` |
| 57 | 0.18% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 58 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 59 | 0.16% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 60 | 0.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.51% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 10.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9728` |
| 3 | 6.97% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3977` |
| 5 | 3.09% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2121` |
| 6 | 2.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3841` |
| 7 | 2.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 1.92% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6315` |
| 11 | 1.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3975` |
| 12 | 1.25% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4292` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5598` |
| 14 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3944` |
| 16 | 0.59% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 17 | 0.55% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 18 | 0.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 19 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 22 | 0.36% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 23 | 0.32% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 24 | 0.30% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 390` |
| 25 | 0.28% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.26% | `net.minecraft.world.level.chunk.status.ChunkStatusTasks.lambda$buildTerrain$0(ChunkAccess) line: 130` |
| 28 | 0.24% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 213` |
| 29 | 0.24% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 30 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

