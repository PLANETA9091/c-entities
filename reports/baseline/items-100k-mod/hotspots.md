# JVM hotspots (JFR ExecutionSample)

Samples: 5371

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.15% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 6.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 3 | 5.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9777` |
| 4 | 4.51% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 3.95% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 6 | 2.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9770` |
| 7 | 2.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 1.79% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9733` |
| 10 | 1.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9730` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 12 | 1.38% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5781` |
| 14 | 1.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 15 | 1.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 1.01% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 17 | 1.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 18 | 0.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 19 | 0.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5684` |
| 20 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2835` |
| 22 | 0.67% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2873` |
| 24 | 0.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 25 | 0.54% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4152` |
| 26 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9738` |
| 27 | 0.45% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 28 | 0.39% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.37% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 30 | 0.32% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 31 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5489` |
| 32 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 33 | 0.30% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 34 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2826` |
| 36 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3976` |
| 37 | 0.28% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 38 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5622` |
| 39 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 40 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 41 | 0.26% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 42 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2832` |
| 43 | 0.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9953` |
| 44 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 45 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 46 | 0.24% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 47 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5581` |
| 48 | 0.22% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 49 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5624` |
| 50 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2847` |
| 51 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5488` |
| 52 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 53 | 0.20% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 54 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5674` |
| 55 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5490` |
| 56 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5625` |
| 57 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 58 | 0.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 59 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2825` |
| 60 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5693` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.08% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 3 | 8.15% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4105` |
| 5 | 3.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 6 | 3.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2217` |
| 7 | 2.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3969` |
| 9 | 2.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 12 | 1.38% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 13 | 1.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 1.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4103` |
| 16 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 18 | 0.56% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.45% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 20 | 0.43% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 22 | 0.41% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 23 | 0.37% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 24 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5785` |
| 25 | 0.35% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 26 | 0.32% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 27 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4073` |

GC pauses: 0, total 0 ms

