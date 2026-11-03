# JVM hotspots (JFR ExecutionSample)

Samples: 4854

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.67% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 4.99% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 4.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5962` |
| 4 | 4.26% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4456` |
| 5 | 4.16% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10048` |
| 7 | 2.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 2.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4324` |
| 9 | 2.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10041` |
| 10 | 1.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6033` |
| 11 | 1.87% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10001` |
| 12 | 1.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10004` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5936` |
| 14 | 1.24% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 1.11% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 17 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 1.03% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3125` |
| 19 | 1.03% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4404` |
| 20 | 0.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3087` |
| 21 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5927` |
| 22 | 0.74% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6735` |
| 24 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10009` |
| 25 | 0.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5741` |
| 26 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3078` |
| 27 | 0.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10224` |
| 28 | 0.45% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 29 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6019` |
| 30 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 31 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3099` |
| 32 | 0.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6725` |
| 33 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 34 | 0.35% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 35 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 38 | 0.33% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 39 | 0.25% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 40 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 41 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 42 | 0.25% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 43 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6017` |
| 44 | 0.25% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 45 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 46 | 0.25% | `java.lang.Double.isFinite(double) line: 1023` |
| 47 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4317` |
| 48 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 49 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 50 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 51 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 52 | 0.23% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 53 | 0.23% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 54 | 0.23% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 55 | 0.23% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4455` |
| 56 | 0.23% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 57 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 58 | 0.21% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4399` |
| 59 | 0.21% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 60 | 0.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.51% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10178` |
| 3 | 5.67% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4357` |
| 5 | 4.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4221` |
| 6 | 3.71% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2469` |
| 7 | 3.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6735` |
| 8 | 3.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5927` |
| 9 | 2.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 2.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4355` |
| 12 | 1.40% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4324` |
| 14 | 1.24% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4676` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6019` |
| 16 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.80% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 19 | 0.47% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4325` |
| 21 | 0.41% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.41% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 23 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.37% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 25 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.33% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6037` |
| 28 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.util.ParallelSearchRadiusIteration.generateBFSOrder(int) line: 344` |
| 30 | 0.25% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |

GC pauses: 0, total 0 ms

