# JVM hotspots (JFR ExecutionSample)

Samples: 5678

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5472` |
| 2 | 10.97% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9395` |
| 3 | 8.21% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.70% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4052` |
| 5 | 4.31% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 1.81% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.22% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9390` |
| 9 | 1.14% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9387` |
| 10 | 1.04% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 11 | 1.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 0.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3925` |
| 14 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2758` |
| 15 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2720` |
| 16 | 0.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5549` |
| 17 | 0.62% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 0.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6241` |
| 19 | 0.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5478` |
| 20 | 0.55% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4005` |
| 21 | 0.42% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6251` |
| 23 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5534` |
| 24 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9394` |
| 25 | 0.35% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 26 | 0.35% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 27 | 0.32% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 28 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5288` |
| 30 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2711` |
| 31 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5419` |
| 32 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3918` |
| 33 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 34 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5377` |
| 35 | 0.26% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 36 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 37 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 38 | 0.23% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 39 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 40 | 0.21% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 41 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5465` |
| 42 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 43 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 44 | 0.19% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 45 | 0.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2290` |
| 46 | 0.18% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 47 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5287` |
| 48 | 0.18% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2732` |
| 50 | 0.18% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 51 | 0.16% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 52 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2705` |
| 53 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2717` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5582` |
| 55 | 0.16% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4007` |
| 57 | 0.14% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 58 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 59 | 0.14% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 60 | 0.14% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.70% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 13.93% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9504` |
| 3 | 8.24% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3958` |
| 5 | 3.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2102` |
| 6 | 2.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9494` |
| 7 | 2.22% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 1.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 1.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3822` |
| 10 | 1.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6251` |
| 11 | 1.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2733` |
| 13 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5534` |
| 14 | 0.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3956` |
| 15 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3925` |
| 17 | 0.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.39% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.39% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 20 | 0.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9503` |
| 21 | 0.37% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 23 | 0.35% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 24 | 0.32% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.30% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.28% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 27 | 0.28% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 28 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.26% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 30 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |

GC pauses: 0, total 0 ms

