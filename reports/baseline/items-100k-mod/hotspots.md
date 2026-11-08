# JVM hotspots (JFR ExecutionSample)

Samples: 5717

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10970` |
| 2 | 5.72% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 5.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7107` |
| 4 | 4.83% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 4.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6344` |
| 6 | 3.18% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4764` |
| 7 | 3.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.78% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6415` |
| 11 | 1.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4632` |
| 12 | 1.45% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10820` |
| 13 | 1.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10832` |
| 16 | 0.91% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 17 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6271` |
| 18 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.70% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 20 | 0.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3401` |
| 22 | 0.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7117` |
| 23 | 0.59% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4712` |
| 24 | 0.51% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 25 | 0.47% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 427` |
| 26 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 27 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6401` |
| 28 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10836` |
| 29 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 30 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 31 | 0.28% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 33 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3353` |
| 34 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6262` |
| 35 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6448` |
| 36 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 113` |
| 37 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 38 | 0.23% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 39 | 0.21% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 40 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3352` |
| 41 | 0.21% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 411` |
| 42 | 0.21% | `java.util.ArrayList$Itr.next() line: 1058` |
| 43 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 44 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 45 | 0.19% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 46 | 0.19% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4763` |
| 47 | 0.19% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 312` |
| 49 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 50 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 51 | 0.17% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 52 | 0.17% | `java.lang.String.length() line: 1583` |
| 53 | 0.17% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3359` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3345` |
| 56 | 0.17% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4709` |
| 58 | 0.17% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 59 | 0.16% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10841` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.45% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 6.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 4.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6262` |
| 5 | 4.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.97% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 3.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4665` |
| 8 | 2.78% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4984` |
| 9 | 2.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4529` |
| 10 | 2.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2638` |
| 11 | 2.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7117` |
| 12 | 1.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11052` |
| 14 | 1.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11065` |
| 15 | 1.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4663` |
| 16 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6401` |
| 18 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 20 | 0.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4632` |
| 21 | 0.61% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 460` |
| 22 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4633` |
| 24 | 0.42% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 25 | 0.38% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 26 | 0.33% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.31% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 29 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3380` |
| 30 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

