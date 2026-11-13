# JVM hotspots (JFR ExecutionSample)

Samples: 4564

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.14% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 5.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6651` |
| 3 | 4.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11978` |
| 4 | 3.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 3.53% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4887` |
| 6 | 2.76% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7357` |
| 8 | 2.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.73% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 10 | 1.71% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.69% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6608` |
| 12 | 1.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 13 | 1.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7347` |
| 15 | 1.16% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11828` |
| 16 | 1.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4647` |
| 17 | 1.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6612` |
| 18 | 1.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 0.96% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 20 | 0.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6286` |
| 21 | 0.74% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11840` |
| 22 | 0.66% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 23 | 0.55% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11844` |
| 25 | 0.50% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 26 | 0.48% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 27 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6594` |
| 28 | 0.39% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 30 | 0.35% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 31 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3461` |
| 32 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 34 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 35 | 0.28% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 36 | 0.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6592` |
| 38 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 39 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 40 | 0.24% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 41 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6455` |
| 42 | 0.24% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 43 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 44 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4586` |
| 45 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 46 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 47 | 0.20% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 48 | 0.20% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 49 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 740` |
| 50 | 0.20% | `java.lang.Double.isFinite(double) line: 1023` |
| 51 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6537` |
| 52 | 0.20% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4886` |
| 53 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6330` |
| 54 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4655` |
| 55 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6487` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11879` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 11827` |
| 58 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6493` |
| 59 | 0.15% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 60 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.31% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 5.48% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 3 | 4.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 4.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6455` |
| 5 | 4.03% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4655` |
| 7 | 2.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4627` |
| 8 | 2.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7357` |
| 11 | 1.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6537` |
| 12 | 1.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6594` |
| 13 | 1.71% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5123` |
| 14 | 1.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12073` |
| 15 | 1.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12060` |
| 16 | 1.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 1.14% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2684` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4650` |
| 19 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6612` |
| 21 | 0.59% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.50% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 24 | 0.48% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 25 | 0.46% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 26 | 0.44% | `java.util.ArrayList.grow(int) line: 238` |
| 27 | 0.39% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 28 | 0.39% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 29 | 0.35% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 30 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |

GC pauses: 0, total 0 ms

