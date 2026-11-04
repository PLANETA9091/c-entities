# JVM hotspots (JFR ExecutionSample)

Samples: 4172

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.34% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 7.02% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 4.39% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10093` |
| 5 | 4.07% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 6 | 3.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 3.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5978` |
| 8 | 2.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 9 | 2.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10086` |
| 10 | 1.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5952` |
| 11 | 1.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6049` |
| 12 | 1.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10049` |
| 13 | 1.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10046` |
| 14 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |
| 15 | 1.22% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 1.10% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 17 | 1.01% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 18 | 0.96% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4415` |
| 19 | 0.93% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3136` |
| 20 | 0.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 22 | 0.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 23 | 0.67% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 24 | 0.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6751` |
| 25 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6035` |
| 26 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10269` |
| 28 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5756` |
| 29 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10054` |
| 30 | 0.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6741` |
| 31 | 0.34% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 33 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 34 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 35 | 0.29% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 36 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 37 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 38 | 0.26% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 39 | 0.26% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 40 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5849` |
| 41 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 42 | 0.24% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 43 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3089` |
| 44 | 0.24% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 45 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5757` |
| 46 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 47 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 48 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 49 | 0.22% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 50 | 0.22% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 51 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3110` |
| 52 | 0.22% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5192` |
| 53 | 0.22% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 508` |
| 54 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3088` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 57 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5892` |
| 58 | 0.19% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 59 | 0.17% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 60 | 0.17% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 393` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.27% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 10.07% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10223` |
| 3 | 8.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5943` |
| 5 | 4.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4232` |
| 6 | 4.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4368` |
| 7 | 3.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 3.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6751` |
| 9 | 3.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 10 | 1.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4366` |
| 12 | 1.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 13 | 1.22% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 14 | 1.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 15 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6035` |
| 16 | 1.01% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 17 | 0.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4336` |
| 19 | 0.53% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 20 | 0.50% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.48% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 22 | 0.43% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.43% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 24 | 0.41% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.31% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 26 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 27 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6053` |
| 29 | 0.26% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 30 | 0.24% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |

GC pauses: 0, total 0 ms

