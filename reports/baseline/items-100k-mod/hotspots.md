# JVM hotspots (JFR ExecutionSample)

Samples: 4887

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.59% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 7.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 3 | 6.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 4 | 4.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 4.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.64% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4729` |
| 7 | 3.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6309` |
| 8 | 3.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.07% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 2.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4597` |
| 11 | 1.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6380` |
| 12 | 1.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10785` |
| 14 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6236` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4677` |
| 16 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3366` |
| 18 | 0.78% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 19 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.70% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 22 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10797` |
| 23 | 0.55% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 24 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7082` |
| 25 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 26 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 27 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 28 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6366` |
| 29 | 0.31% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 393` |
| 30 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 31 | 0.29% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 32 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.27% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 409` |
| 34 | 0.27% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 35 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 36 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 37 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 38 | 0.23% | `java.util.ArrayList$Itr.next() line: 1058` |
| 39 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6259` |
| 40 | 0.20% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 41 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10836` |
| 42 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10801` |
| 43 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 44 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3318` |
| 45 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3340` |
| 46 | 0.18% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 47 | 0.18% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 294` |
| 48 | 0.18% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4728` |
| 50 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3310` |
| 51 | 0.16% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 52 | 0.16% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 410` |
| 53 | 0.16% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4672` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4679` |
| 55 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 56 | 0.14% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 57 | 0.14% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 58 | 0.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 59 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6227` |
| 60 | 0.12% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.66% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 7.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6227` |
| 4 | 6.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 5.38% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 4.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 3.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4630` |
| 8 | 2.99% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4494` |
| 9 | 2.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7082` |
| 10 | 2.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2619` |
| 11 | 2.07% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4949` |
| 12 | 1.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4628` |
| 14 | 1.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11017` |
| 15 | 1.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4597` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11030` |
| 17 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.76% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 19 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6366` |
| 21 | 0.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4598` |
| 22 | 0.47% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.47% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 442` |
| 24 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3345` |
| 25 | 0.41% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.39% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 27 | 0.37% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.31% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 29 | 0.31% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 407` |
| 30 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6384` |

GC pauses: 0, total 0 ms

