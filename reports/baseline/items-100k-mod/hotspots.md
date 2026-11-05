# JVM hotspots (JFR ExecutionSample)

Samples: 5952

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.71% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10303` |
| 2 | 8.67% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 7.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 4 | 5.38% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 3.86% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 6 | 2.44% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10294` |
| 8 | 1.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 1.56% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6092` |
| 11 | 1.18% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10298` |
| 12 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 13 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 15 | 0.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3138` |
| 17 | 0.79% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 18 | 0.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 19 | 0.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5965` |
| 20 | 0.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6794` |
| 21 | 0.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3100` |
| 22 | 0.54% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4417` |
| 23 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6078` |
| 24 | 0.37% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 25 | 0.32% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.32% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 27 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 28 | 0.29% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 29 | 0.29% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 30 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 31 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.27% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 33 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5903` |
| 34 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 35 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5859` |
| 36 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |
| 37 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 38 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 39 | 0.22% | `java.util.Arrays.fill(long[], long) line: 3144` |
| 40 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5766` |
| 41 | 0.22% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4419` |
| 42 | 0.20% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 43 | 0.20% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 44 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5998` |
| 45 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 46 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10333` |
| 47 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5860` |
| 48 | 0.18% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6125` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5988` |
| 51 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 52 | 0.17% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6074` |
| 54 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 55 | 0.15% | `java.util.HashMap.putVal(int, Object, Object, boolean, boolean) line: 647` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3083` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4688` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5768` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5941` |
| 60 | 0.15% | `java.lang.Double.isFinite(double) line: 1023` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.35% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 15.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10486` |
| 3 | 8.69% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 5 | 4.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4370` |
| 6 | 2.47% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 7 | 2.44% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 8 | 1.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 1.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4234` |
| 12 | 1.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6794` |
| 13 | 0.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 0.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4368` |
| 15 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6078` |
| 16 | 0.44% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 17 | 0.44% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 18 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 19 | 0.39% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.32% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 21 | 0.32% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 390` |
| 22 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 23 | 0.29% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 24 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.24% | `java.util.HashMap.put(Object, Object) line: 619` |
| 26 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5738` |
| 27 | 0.20% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.20% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 29 | 0.20% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 30 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

