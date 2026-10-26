# JVM hotspots (JFR ExecutionSample)

Samples: 4197

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.25% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4001` |
| 2 | 11.17% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 7.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 4 | 2.76% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 5 | 2.05% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 1.98% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 7 | 1.95% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 1.74% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 9 | 1.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2075` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3488` |
| 11 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.88% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 0.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2029` |
| 15 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.71% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2213` |
| 17 | 0.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 18 | 0.52% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 19 | 0.50% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 20 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 21 | 0.43% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 22 | 0.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4199` |
| 23 | 0.41% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 24 | 0.41% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 25 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2115` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3525` |
| 27 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2104` |
| 28 | 0.33% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.<init>(Object, MultiThreadedQueue$LinkedNode) line: 1250` |
| 29 | 0.33% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 30 | 0.31% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4041` |
| 31 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 33 | 0.29% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 34 | 0.24% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 35 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2032` |
| 36 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 37 | 0.21% | `java.util.HashMap.putVal(int, Object, Object, boolean, boolean) line: 651` |
| 38 | 0.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 39 | 0.21% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 40 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 41 | 0.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 42 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 43 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 44 | 0.19% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 45 | 0.19% | `java.util.Collection.stream() line: 748` |
| 46 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 47 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2018` |
| 49 | 0.19% | `net.minecraft.world.entity.EntityType.getCategory() line: 333` |
| 50 | 0.19% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 51 | 0.19% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 52 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 53 | 0.17% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 54 | 0.17% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 55 | 0.17% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 56 | 0.17% | `java.lang.Class.cast(Object) line: 3491` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4032` |
| 58 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2020` |
| 59 | 0.17% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 60 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3494` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.01% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 2 | 10.34% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 3 | 8.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 5.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4199` |
| 5 | 2.91% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 6 | 2.84% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 8 | 2.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3525` |
| 9 | 2.03% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2018` |
| 10 | 1.98% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 11 | 1.69% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6421` |
| 12 | 1.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 14 | 1.07% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2136` |
| 17 | 0.69% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4001` |
| 18 | 0.67% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 19 | 0.64% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.48% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 21 | 0.45% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 22 | 0.36% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.33% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.<init>() line: 90` |
| 24 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 25 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 26 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.31% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2029` |
| 29 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2098` |
| 30 | 0.26% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |

GC pauses: 0, total 0 ms

