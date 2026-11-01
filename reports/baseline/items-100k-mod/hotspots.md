# JVM hotspots (JFR ExecutionSample)

Samples: 6174

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8988` |
| 2 | 9.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5275` |
| 3 | 5.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 5.04% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 4.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 4.00% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3868` |
| 7 | 3.55% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8935` |
| 8 | 1.72% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6011` |
| 10 | 1.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3741` |
| 11 | 1.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8949` |
| 14 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5281` |
| 16 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 17 | 0.70% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 0.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5312` |
| 19 | 0.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6021` |
| 20 | 0.53% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 22 | 0.52% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3821` |
| 23 | 0.50% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 24 | 0.49% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 25 | 0.49% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8631` |
| 26 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5320` |
| 27 | 0.40% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8629` |
| 28 | 0.32% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 30 | 0.31% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1864` |
| 31 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 32 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 33 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5094` |
| 34 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 35 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3645` |
| 36 | 0.24% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 37 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8939` |
| 38 | 0.24% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 39 | 0.23% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 40 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 41 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5180` |
| 42 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5224` |
| 43 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3753` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2709` |
| 45 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 46 | 0.19% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 47 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 48 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2694` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8943` |
| 51 | 0.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6012` |
| 52 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 53 | 0.18% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 54 | 0.18% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2687` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 393` |
| 58 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3734` |
| 59 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2688` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.95% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.46% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8984` |
| 4 | 5.04% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 4.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5312` |
| 6 | 4.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8976` |
| 7 | 4.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3774` |
| 8 | 2.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6021` |
| 9 | 2.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 2.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 11 | 1.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 12 | 1.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.96% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 15 | 0.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3772` |
| 16 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8942` |
| 18 | 0.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 19 | 0.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.44% | `java.math.BigInteger.multiplyToLen(int[], int, int[], int, int[]) line: 1844` |
| 21 | 0.40% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.39% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 23 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3741` |
| 24 | 0.32% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3742` |
| 26 | 0.32% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 27 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.31% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 29 | 0.31% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 30 | 0.24% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 762` |

GC pauses: 0, total 0 ms

