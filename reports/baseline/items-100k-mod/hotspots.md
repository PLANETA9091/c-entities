# JVM hotspots (JFR ExecutionSample)

Samples: 5184

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.94% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 4.19% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 4.15% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10059` |
| 4 | 3.53% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.01% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6746` |
| 6 | 2.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 2.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5973` |
| 8 | 2.82% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10052` |
| 9 | 2.80% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4467` |
| 10 | 1.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5947` |
| 11 | 1.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.56% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 14 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6044` |
| 15 | 1.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10012` |
| 16 | 1.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 17 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5938` |
| 18 | 1.12% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10015` |
| 19 | 1.10% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 1.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6736` |
| 21 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6077` |
| 22 | 0.96% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 23 | 0.95% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3136` |
| 24 | 0.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 25 | 0.54% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 26 | 0.54% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4415` |
| 27 | 0.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6030` |
| 28 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4328` |
| 29 | 0.48% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 30 | 0.46% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 31 | 0.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5751` |
| 32 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 33 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 34 | 0.33% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 35 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.31% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 37 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10020` |
| 38 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4221` |
| 39 | 0.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10235` |
| 40 | 0.27% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 41 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.27% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.27% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 44 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6026` |
| 45 | 0.25% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 46 | 0.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3089` |
| 47 | 0.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3088` |
| 48 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4347` |
| 49 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 50 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 51 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3110` |
| 52 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3101` |
| 53 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 54 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 55 | 0.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 56 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6043` |
| 57 | 0.21% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4410` |
| 58 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5885` |
| 59 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 60 | 0.19% | `java.util.Comparator.lambda$comparingLong$6043328a$1(ToLongFunction, Object, Object) line: 515` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.39% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 10.07% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10189` |
| 3 | 4.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5938` |
| 4 | 4.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 4.21% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 6 | 4.21% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 7 | 3.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4232` |
| 8 | 3.11% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4368` |
| 9 | 2.89% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 2.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6746` |
| 11 | 1.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 13 | 1.54% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 14 | 1.14% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4366` |
| 16 | 1.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4335` |
| 17 | 0.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6030` |
| 18 | 0.56% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4336` |
| 20 | 0.48% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 21 | 0.46% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 22 | 0.44% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.44% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 24 | 0.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6048` |
| 25 | 0.37% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.37% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 27 | 0.33% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 29 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.31% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |

GC pauses: 0, total 0 ms

