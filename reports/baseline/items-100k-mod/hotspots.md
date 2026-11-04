# JVM hotspots (JFR ExecutionSample)

Samples: 4848

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.57% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 4.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10092` |
| 3 | 4.10% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6751` |
| 5 | 3.63% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 6 | 2.99% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10085` |
| 7 | 2.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5978` |
| 8 | 2.62% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 9 | 2.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 2.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 11 | 1.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 12 | 1.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10048` |
| 13 | 1.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |
| 14 | 1.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6082` |
| 16 | 1.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 1.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6049` |
| 18 | 1.18% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 19 | 1.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4415` |
| 20 | 1.16% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 21 | 1.09% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3136` |
| 22 | 1.07% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10045` |
| 23 | 1.01% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 24 | 0.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5952` |
| 25 | 0.83% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 26 | 0.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6035` |
| 27 | 0.66% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 28 | 0.45% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 29 | 0.45% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4410` |
| 30 | 0.39% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 31 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4328` |
| 32 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4347` |
| 33 | 0.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 34 | 0.35% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 35 | 0.35% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 36 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 37 | 0.31% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 38 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 39 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 40 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5757` |
| 41 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5942` |
| 42 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3110` |
| 43 | 0.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10268` |
| 44 | 0.25% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 45 | 0.25% | `java.lang.Double.isFinite(double) line: 1023` |
| 46 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5756` |
| 47 | 0.23% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 48 | 0.23% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 49 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 50 | 0.23% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4466` |
| 51 | 0.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10053` |
| 52 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 53 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 54 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 55 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3089` |
| 56 | 0.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 57 | 0.21% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 58 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 59 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 60 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.52% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 10.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10222` |
| 3 | 4.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 4 | 4.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 4.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4368` |
| 6 | 4.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4232` |
| 7 | 3.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |
| 8 | 3.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6751` |
| 9 | 2.66% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 10 | 2.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 2.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4366` |
| 12 | 2.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 1.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 16 | 1.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4336` |
| 17 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6035` |
| 18 | 0.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6053` |
| 20 | 0.50% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 22 | 0.41% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 23 | 0.39% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.37% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.35% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.35% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 27 | 0.35% | `dev.planeta9091.centities.CeHooks.itemImmunities(ItemEntity) line: 4443` |
| 28 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 29 | 0.31% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 30 | 0.31% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |

GC pauses: 0, total 0 ms

