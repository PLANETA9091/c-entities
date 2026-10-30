# JVM hotspots (JFR ExecutionSample)

Samples: 6291

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4737` |
| 2 | 11.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 3 | 7.41% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.40% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 6 | 3.24% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 7 | 2.70% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3438` |
| 8 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 1.29% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4801` |
| 11 | 1.03% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8453` |
| 12 | 1.02% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 13 | 0.97% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2450` |
| 14 | 0.92% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5528` |
| 16 | 0.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5530` |
| 17 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4838` |
| 20 | 0.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3330` |
| 21 | 0.51% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.41% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 23 | 0.40% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3391` |
| 24 | 0.35% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 25 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4620` |
| 26 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4619` |
| 27 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 28 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 30 | 0.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 31 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 32 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8443` |
| 33 | 0.24% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 34 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 35 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 36 | 0.22% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 37 | 0.22% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8133` |
| 38 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3323` |
| 39 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 40 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4707` |
| 41 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4846` |
| 42 | 0.19% | `java.lang.Integer.valueOf(int) line: 1006` |
| 43 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4807` |
| 44 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 45 | 0.17% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 46 | 0.17% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 47 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3342` |
| 48 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4750` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3228` |
| 50 | 0.17% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 51 | 0.16% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 52 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 53 | 0.16% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 54 | 0.16% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4748` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2462` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8422` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3435` |
| 59 | 0.16% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 60 | 0.14% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.82% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 7.41% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.99% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8488` |
| 5 | 3.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8480` |
| 6 | 3.02% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3351` |
| 7 | 2.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2446` |
| 8 | 2.08% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5530` |
| 9 | 1.91% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.78% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1921` |
| 11 | 1.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3233` |
| 14 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4838` |
| 15 | 1.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2092` |
| 16 | 0.83% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2463` |
| 17 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3350` |
| 19 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8446` |
| 21 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 22 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3330` |
| 23 | 0.37% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.35% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 613` |
| 25 | 0.33% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.24% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 29 | 0.22% | `java.math.BigInteger.multiplyToLen(int[], int, int[], int, int[]) line: 1844` |
| 30 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

