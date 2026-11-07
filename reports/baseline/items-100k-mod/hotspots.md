# JVM hotspots (JFR ExecutionSample)

Samples: 5569

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 2 | 7.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 3 | 5.44% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 4.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6309` |
| 5 | 4.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 2.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.68% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4729` |
| 9 | 2.62% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6380` |
| 11 | 1.76% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.20% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10785` |
| 13 | 1.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4597` |
| 14 | 1.06% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 15 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3366` |
| 17 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 0.74% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10797` |
| 19 | 0.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7082` |
| 20 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6236` |
| 21 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 22 | 0.56% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 23 | 0.47% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 24 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 25 | 0.40% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 409` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4677` |
| 27 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6366` |
| 28 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10801` |
| 29 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.task.ChunkLoadTask.<init>(ChunkTaskScheduler, ServerLevel, int, int, NewChunkHolder, Priority) line: 49` |
| 30 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 31 | 0.27% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 393` |
| 32 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 33 | 0.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3310` |
| 34 | 0.25% | `java.util.ArrayList$Itr.next() line: 1058` |
| 35 | 0.25% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 36 | 0.23% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 37 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 38 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 39 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 40 | 0.23% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4673` |
| 41 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 42 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 43 | 0.22% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 44 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3318` |
| 45 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3340` |
| 46 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 47 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 48 | 0.20% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10806` |
| 50 | 0.18% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4679` |
| 52 | 0.16% | `java.util.ArrayList.grow(int) line: 238` |
| 53 | 0.16% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 54 | 0.16% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |
| 55 | 0.16% | `java.lang.Class.cast(Object) line: 3491` |
| 56 | 0.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6227` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4948` |
| 59 | 0.16% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 60 | 0.14% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.83% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 7.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 4.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6227` |
| 5 | 4.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 4.06% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4630` |
| 8 | 2.62% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4949` |
| 9 | 2.42% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2619` |
| 10 | 2.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4494` |
| 11 | 1.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7082` |
| 13 | 1.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11030` |
| 14 | 1.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11017` |
| 15 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 16 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6366` |
| 17 | 0.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4628` |
| 18 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4597` |
| 20 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3345` |
| 21 | 0.54% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 442` |
| 22 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 23 | 0.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4598` |
| 24 | 0.41% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.38% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.34% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.createTask(int, int, ChunkAccess, NewChunkHolder, StaticCache2D, ChunkStatus, Priority) line: 659` |
| 28 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.27% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 407` |
| 30 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

