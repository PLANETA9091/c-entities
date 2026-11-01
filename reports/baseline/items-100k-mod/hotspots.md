# JVM hotspots (JFR ExecutionSample)

Samples: 6289

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8939` |
| 2 | 10.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5226` |
| 3 | 5.61% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.25% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 5.10% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 3.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8886` |
| 7 | 3.56% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3819` |
| 8 | 2.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5962` |
| 9 | 1.97% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 10 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.02% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8900` |
| 12 | 0.99% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5232` |
| 16 | 0.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3692` |
| 17 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5263` |
| 18 | 0.76% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 19 | 0.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5972` |
| 20 | 0.57% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 21 | 0.52% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.51% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8582` |
| 23 | 0.51% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3772` |
| 24 | 0.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 26 | 0.38% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8580` |
| 27 | 0.37% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 28 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5271` |
| 29 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8890` |
| 30 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 31 | 0.29% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 32 | 0.29% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 33 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5045` |
| 34 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 35 | 0.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8894` |
| 36 | 0.24% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 37 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 38 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2688` |
| 39 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 40 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 41 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2687` |
| 42 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3589` |
| 44 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2709` |
| 45 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5131` |
| 46 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 47 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 48 | 0.16% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 49 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3685` |
| 50 | 0.16% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 51 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3596` |
| 52 | 0.16% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 53 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5046` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3704` |
| 55 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 56 | 0.14% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 57 | 0.14% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 58 | 0.14% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 59 | 0.14% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.getTailOpaque() line: 80` |
| 60 | 0.14% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2694` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.54% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.63% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5263` |
| 5 | 5.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8935` |
| 6 | 4.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8927` |
| 7 | 3.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3725` |
| 8 | 3.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 2.58% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 10 | 2.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5972` |
| 11 | 2.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3594` |
| 12 | 1.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 0.92% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 16 | 0.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 17 | 0.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3723` |
| 18 | 0.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8893` |
| 20 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3692` |
| 21 | 0.37% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.37% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 23 | 0.29% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.27% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.27% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 27 | 0.25% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 28 | 0.25% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 29 | 0.22% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 30 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |

GC pauses: 0, total 0 ms

