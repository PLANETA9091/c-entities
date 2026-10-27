# JVM hotspots (JFR ExecutionSample)

Samples: 4141

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 2 | 8.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 3 | 5.55% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.91% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 5 | 2.32% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 2.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2135` |
| 7 | 2.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 1.91% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3567` |
| 10 | 1.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 0.97% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7177` |
| 13 | 0.82% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2089` |
| 15 | 0.68% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 16 | 0.68% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 17 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1484` |
| 18 | 0.60% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7167` |
| 19 | 0.53% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2273` |
| 20 | 0.48% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 21 | 0.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2175` |
| 22 | 0.43% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.36% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 24 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 25 | 0.36% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 26 | 0.36% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 27 | 0.36% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 28 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 29 | 0.34% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 30 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1468` |
| 31 | 0.34% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 32 | 0.31% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 33 | 0.31% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 34 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2164` |
| 35 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 36 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 37 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1465` |
| 38 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 39 | 0.27% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 40 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 41 | 0.27% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 42 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7171` |
| 43 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 44 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 45 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3604` |
| 46 | 0.22% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 47 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2093` |
| 48 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 49 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 50 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 51 | 0.19% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 52 | 0.19% | `java.util.Collection.stream() line: 748` |
| 53 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2121` |
| 54 | 0.19% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2154` |
| 56 | 0.17% | `java.util.Comparator.lambda$comparingLong$6043328a$1(ToLongFunction, Object, Object) line: 515` |
| 57 | 0.17% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3264` |
| 58 | 0.17% | `java.lang.Math.multiplyHigh(long, long) line: 1406` |
| 59 | 0.17% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 60 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3612` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 8.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 5.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7212` |
| 4 | 5.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4278` |
| 5 | 4.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7204` |
| 6 | 3.31% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 8 | 2.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2078` |
| 9 | 2.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3604` |
| 10 | 2.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.50% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1081` |
| 12 | 1.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 13 | 0.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1469` |
| 14 | 0.68% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 15 | 0.68% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7170` |
| 16 | 0.58% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 17 | 0.56% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2196` |
| 19 | 0.51% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.43% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2089` |
| 23 | 0.36% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 24 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2158` |
| 25 | 0.34% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 26 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 28 | 0.31% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isJoinNonEmpty(VoxelShape, VoxelShape, BooleanOp) line: 1320` |
| 29 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 30 | 0.29% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |

GC pauses: 0, total 0 ms

