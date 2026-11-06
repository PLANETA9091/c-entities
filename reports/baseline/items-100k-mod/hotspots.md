# JVM hotspots (JFR ExecutionSample)

Samples: 5582

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10392` |
| 2 | 7.00% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 6.13% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 4.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 5 | 3.67% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 6 | 2.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.06% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 2.04% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10383` |
| 9 | 2.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6092` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 12 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.88% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 16 | 0.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 17 | 0.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3138` |
| 18 | 0.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5965` |
| 19 | 0.77% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10387` |
| 20 | 0.68% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 21 | 0.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3100` |
| 22 | 0.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6794` |
| 23 | 0.57% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.57% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4417` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6078` |
| 26 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 27 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 28 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5766` |
| 29 | 0.30% | `java.util.Arrays.fill(long[], long) line: 3144` |
| 30 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 31 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 32 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5903` |
| 33 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 34 | 0.23% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 35 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3090` |
| 36 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |
| 37 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6074` |
| 38 | 0.23% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 39 | 0.21% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 40 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 41 | 0.20% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 42 | 0.20% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 43 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10422` |
| 44 | 0.20% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4330` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5768` |
| 47 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 48 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 49 | 0.18% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 50 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5905` |
| 53 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 54 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 55 | 0.16% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 56 | 0.16% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3091` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5966` |
| 59 | 0.16% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5906` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10575` |
| 2 | 14.03% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.00% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 5 | 3.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4370` |
| 6 | 2.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4234` |
| 7 | 2.83% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 8 | 2.06% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 9 | 2.04% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6794` |
| 12 | 1.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.02% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 14 | 0.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4368` |
| 15 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6078` |
| 16 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.68% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 18 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.41% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.41% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.39% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 22 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5738` |
| 24 | 0.29% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 25 | 0.27% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.27% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 762` |
| 27 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 28 | 0.25% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 29 | 0.23% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 30 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4338` |

GC pauses: 0, total 0 ms

