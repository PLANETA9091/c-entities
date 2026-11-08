# JVM hotspots (JFR ExecutionSample)

Samples: 5424

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7107` |
| 2 | 6.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10970` |
| 3 | 5.48% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 5.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 5.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6344` |
| 6 | 3.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 2.60% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4764` |
| 8 | 2.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.40% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6415` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4632` |
| 12 | 1.42% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10820` |
| 13 | 1.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.85% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 17 | 0.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6271` |
| 18 | 0.68% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 19 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3401` |
| 20 | 0.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7117` |
| 21 | 0.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 22 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10832` |
| 23 | 0.41% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.39% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 25 | 0.35% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 427` |
| 26 | 0.35% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4712` |
| 27 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 29 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6401` |
| 30 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3352` |
| 31 | 0.28% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 312` |
| 32 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 33 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 34 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 35 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 36 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10836` |
| 37 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 38 | 0.22% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 39 | 0.22% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 40 | 0.22% | `java.util.ArrayList$Itr.next() line: 1058` |
| 41 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4625` |
| 42 | 0.22% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4714` |
| 43 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 44 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3358` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3353` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 411` |
| 48 | 0.18% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3359` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6448` |
| 51 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 52 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 53 | 0.17% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 54 | 0.17% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 508` |
| 55 | 0.15% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 70` |
| 56 | 0.15% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 57 | 0.15% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4707` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4708` |
| 60 | 0.15% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.61% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 6.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 5.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6262` |
| 6 | 4.00% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4665` |
| 8 | 2.40% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4984` |
| 9 | 2.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2638` |
| 10 | 2.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7117` |
| 11 | 2.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4529` |
| 12 | 1.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11052` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4663` |
| 15 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6401` |
| 17 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11065` |
| 19 | 0.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4632` |
| 20 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 21 | 0.48% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 460` |
| 22 | 0.46% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3380` |
| 24 | 0.39% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4633` |
| 26 | 0.37% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.31% | `java.util.ArrayList.grow(int) line: 238` |
| 29 | 0.31% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 30 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |

GC pauses: 0, total 0 ms

