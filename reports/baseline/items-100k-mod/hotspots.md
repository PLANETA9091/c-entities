# JVM hotspots (JFR ExecutionSample)

Samples: 3550

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.10% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 2 | 5.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9118` |
| 3 | 3.58% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9204` |
| 5 | 3.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6173` |
| 6 | 2.99% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 2.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5424` |
| 8 | 2.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3890` |
| 9 | 2.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2701` |
| 10 | 1.75% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3970` |
| 11 | 1.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.63% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4017` |
| 13 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5462` |
| 15 | 1.10% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 16 | 0.99% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |
| 17 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 18 | 0.93% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8783` |
| 19 | 0.82% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 20 | 0.76% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.70% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 22 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9125` |
| 23 | 0.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5430` |
| 24 | 0.48% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 25 | 0.45% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 26 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3883` |
| 27 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6163` |
| 28 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5505` |
| 29 | 0.42% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3972` |
| 30 | 0.39% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 31 | 0.39% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 32 | 0.39% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 33 | 0.37% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 34 | 0.34% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 35 | 0.34% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 36 | 0.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 37 | 0.34% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 38 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5242` |
| 39 | 0.34% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 40 | 0.31% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 41 | 0.31% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8781` |
| 42 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 43 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3902` |
| 44 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 45 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2692` |
| 46 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5330` |
| 47 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 48 | 0.25% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 49 | 0.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2704` |
| 50 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5243` |
| 51 | 0.23% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 52 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 53 | 0.23% | `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock.unlock(ReentrantAreaLock$Node) line: 266` |
| 54 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2780` |
| 55 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3792` |
| 56 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5372` |
| 57 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3787` |
| 58 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 59 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 60 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.70% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5462` |
| 3 | 7.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9200` |
| 4 | 5.21% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2083` |
| 5 | 4.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6173` |
| 6 | 4.03% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 3.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 2.99% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 9 | 2.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3921` |
| 10 | 2.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3923` |
| 11 | 2.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.69% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9129` |
| 13 | 1.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 1.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3792` |
| 15 | 1.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 16 | 1.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9192` |
| 17 | 0.96% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 0.76% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9117` |
| 19 | 0.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3891` |
| 20 | 0.62% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2714` |
| 22 | 0.56% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 23 | 0.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3890` |
| 24 | 0.54% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9118` |
| 26 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 27 | 0.45% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.42% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.42% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.39% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |

GC pauses: 0, total 0 ms

