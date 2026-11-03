# JVM hotspots (JFR ExecutionSample)

Samples: 5270

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.66% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 5.46% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 4.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9655` |
| 4 | 4.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5588` |
| 5 | 4.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4082` |
| 6 | 3.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9648` |
| 8 | 2.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 9 | 2.03% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9611` |
| 10 | 2.03% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9608` |
| 11 | 1.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5659` |
| 12 | 1.86% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 1.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5553` |
| 16 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5562` |
| 17 | 1.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6361` |
| 18 | 0.97% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 19 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.91% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |
| 21 | 0.83% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.83% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 23 | 0.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2777` |
| 24 | 0.72% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4030` |
| 25 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9616` |
| 26 | 0.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5645` |
| 27 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6351` |
| 28 | 0.44% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 29 | 0.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9831` |
| 30 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 31 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5366` |
| 32 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 33 | 0.28% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 34 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 35 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2730` |
| 36 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5643` |
| 37 | 0.27% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 38 | 0.27% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 39 | 0.27% | `java.lang.Double.isFinite(double) line: 1023` |
| 40 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 41 | 0.25% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 42 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 43 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2751` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5563` |
| 46 | 0.21% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 393` |
| 47 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3943` |
| 48 | 0.21% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4025` |
| 49 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5552` |
| 50 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5367` |
| 51 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 52 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 53 | 0.19% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 54 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2729` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 57 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 58 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2736` |
| 59 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 60 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3836` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.98% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9785` |
| 3 | 6.66% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3983` |
| 5 | 4.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5553` |
| 6 | 3.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3847` |
| 7 | 3.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2121` |
| 8 | 2.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6361` |
| 9 | 2.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5645` |
| 12 | 1.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3981` |
| 14 | 1.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4302` |
| 16 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.83% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3951` |
| 19 | 0.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 20 | 0.47% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 21 | 0.44% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 22 | 0.42% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.34% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.30% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 27 | 0.27% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 390` |
| 28 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.25% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, NoiseChunk, RandomState, BiomeManager, Set, MaterialRule, ServerLevel) line: 227` |
| 30 | 0.23% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |

GC pauses: 0, total 0 ms

