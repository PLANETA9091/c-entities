# JVM hotspots (JFR ExecutionSample)

Samples: 5461

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.43% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 5.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5650` |
| 3 | 4.83% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 4.78% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9717` |
| 5 | 4.41% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4144` |
| 6 | 2.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9710` |
| 8 | 2.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9673` |
| 9 | 1.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.85% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9670` |
| 11 | 1.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5721` |
| 12 | 1.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4012` |
| 13 | 1.52% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 14 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.97% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5624` |
| 17 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5615` |
| 19 | 0.92% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2824` |
| 20 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2786` |
| 21 | 0.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6413` |
| 22 | 0.70% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4092` |
| 23 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6423` |
| 24 | 0.51% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 25 | 0.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5707` |
| 26 | 0.40% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 27 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5428` |
| 28 | 0.38% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 29 | 0.37% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 31 | 0.37% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 32 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5429` |
| 33 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9678` |
| 34 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2777` |
| 35 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5562` |
| 36 | 0.31% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 37 | 0.29% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 38 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 39 | 0.27% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 40 | 0.26% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 41 | 0.26% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.<init>(Object, MultiThreadedQueue$LinkedNode) line: 1250` |
| 42 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 43 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 44 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2798` |
| 45 | 0.24% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 46 | 0.22% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 47 | 0.22% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 48 | 0.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 49 | 0.22% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 50 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5563` |
| 51 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 52 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5521` |
| 53 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 54 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5565` |
| 55 | 0.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9893` |
| 56 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5633` |
| 57 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 58 | 0.18% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 59 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5703` |
| 60 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5608` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.70% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9847` |
| 3 | 7.45% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4045` |
| 5 | 3.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5615` |
| 6 | 2.97% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2168` |
| 7 | 2.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3909` |
| 8 | 1.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6423` |
| 9 | 1.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.90% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.52% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4364` |
| 12 | 1.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4043` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4012` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5707` |
| 16 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.49% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 19 | 0.46% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.42% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.40% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 390` |
| 22 | 0.38% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.38% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 24 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 25 | 0.37% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 26 | 0.35% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 27 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4013` |
| 28 | 0.29% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 762` |
| 29 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

