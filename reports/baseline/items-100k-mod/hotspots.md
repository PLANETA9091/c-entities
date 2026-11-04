# JVM hotspots (JFR ExecutionSample)

Samples: 5057

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.86% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10083` |
| 2 | 7.85% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 4.71% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 4 | 3.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5979` |
| 5 | 3.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4468` |
| 6 | 2.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4336` |
| 8 | 2.10% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10074` |
| 9 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5953` |
| 11 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6050` |
| 12 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5944` |
| 13 | 1.01% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 1.01% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10078` |
| 17 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3099` |
| 19 | 0.73% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10120` |
| 20 | 0.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6752` |
| 21 | 0.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3137` |
| 22 | 0.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6036` |
| 23 | 0.57% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.47% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 25 | 0.47% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4416` |
| 26 | 0.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6742` |
| 27 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3090` |
| 28 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5758` |
| 29 | 0.36% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 30 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 31 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 32 | 0.34% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 33 | 0.30% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 36 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5894` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 39 | 0.24% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 40 | 0.24% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 393` |
| 41 | 0.22% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 42 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 43 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 44 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 45 | 0.22% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 46 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3111` |
| 47 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5759` |
| 48 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6032` |
| 49 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6083` |
| 50 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 51 | 0.20% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 424` |
| 52 | 0.20% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 53 | 0.20% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4412` |
| 54 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4329` |
| 55 | 0.20% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 56 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5976` |
| 58 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3089` |
| 59 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5893` |
| 60 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5954` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10253` |
| 2 | 12.95% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.85% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 3.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5944` |
| 5 | 3.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4369` |
| 6 | 3.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2481` |
| 7 | 2.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6752` |
| 8 | 2.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4233` |
| 9 | 2.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.40% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4367` |
| 13 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6036` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4688` |
| 15 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4336` |
| 17 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.51% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 19 | 0.45% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.45% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.36% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 22 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 23 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6054` |
| 25 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isJoinNonEmpty(VoxelShape, VoxelShape, BooleanOp) line: 1320` |
| 27 | 0.26% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.26% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 29 | 0.22% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 30 | 0.22% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isJoinNonEmpty(VoxelShape, VoxelShape, BooleanOp) line: 1336` |

GC pauses: 0, total 0 ms

