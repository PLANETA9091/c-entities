# JVM hotspots (JFR ExecutionSample)

Samples: 5783

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10508` |
| 2 | 8.54% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 7.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6120` |
| 4 | 5.27% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 4.12% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4568` |
| 6 | 2.11% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10499` |
| 7 | 2.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 1.90% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6191` |
| 11 | 1.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4436` |
| 12 | 1.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3199` |
| 14 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3237` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10503` |
| 16 | 0.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6064` |
| 18 | 0.81% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 19 | 0.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6055` |
| 20 | 0.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6883` |
| 21 | 0.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6893` |
| 22 | 0.61% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6177` |
| 24 | 0.45% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4516` |
| 25 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5866` |
| 26 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 27 | 0.33% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 28 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5865` |
| 29 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5958` |
| 30 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 31 | 0.24% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.24% | `java.util.Arrays.fill(long[], long) line: 3144` |
| 34 | 0.24% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3190` |
| 36 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10538` |
| 37 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 38 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6002` |
| 39 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 40 | 0.21% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 41 | 0.21% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4518` |
| 42 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 43 | 0.19% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 44 | 0.19% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 45 | 0.19% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 46 | 0.19% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 47 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6224` |
| 48 | 0.17% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6097` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5867` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6003` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2747` |
| 53 | 0.16% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 54 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 55 | 0.16% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3189` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6054` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3182` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6004` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.17% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 14.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10691` |
| 3 | 8.54% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6055` |
| 5 | 4.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4469` |
| 6 | 3.06% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2559` |
| 7 | 2.02% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 1.90% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4788` |
| 9 | 1.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.82% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4333` |
| 11 | 1.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6893` |
| 13 | 0.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4467` |
| 14 | 0.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 0.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6177` |
| 16 | 0.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4436` |
| 18 | 0.38% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.38% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 20 | 0.33% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.31% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 762` |
| 22 | 0.29% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.26% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 24 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 25 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5837` |
| 27 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.22% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 29 | 0.21% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 30 | 0.19% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |

GC pauses: 0, total 0 ms

