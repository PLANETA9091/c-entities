# JVM hotspots (JFR ExecutionSample)

Samples: 5284

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.55% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 2 | 6.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5584` |
| 3 | 5.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9651` |
| 4 | 4.81% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 5 | 4.43% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 6 | 2.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 2.10% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9644` |
| 9 | 1.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9604` |
| 10 | 1.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5655` |
| 11 | 1.59% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9607` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5549` |
| 13 | 1.04% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 14 | 1.02% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 15 | 1.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 1.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.98% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2777` |
| 18 | 0.95% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 19 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5558` |
| 20 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6357` |
| 21 | 0.76% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |
| 22 | 0.72% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.70% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4030` |
| 24 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6347` |
| 25 | 0.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5641` |
| 26 | 0.47% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 27 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9612` |
| 28 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5362` |
| 29 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5363` |
| 30 | 0.34% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 31 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 32 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2751` |
| 33 | 0.34% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 34 | 0.32% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 35 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.32% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 37 | 0.30% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 38 | 0.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9827` |
| 39 | 0.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 40 | 0.26% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.25% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 42 | 0.25% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 43 | 0.25% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 44 | 0.23% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 45 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 46 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2729` |
| 47 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2730` |
| 48 | 0.23% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 49 | 0.23% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4074` |
| 50 | 0.23% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 51 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 52 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5688` |
| 53 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 54 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 55 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 56 | 0.19% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 57 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5559` |
| 58 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5455` |
| 59 | 0.19% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4032` |
| 60 | 0.17% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.59% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.53% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9781` |
| 3 | 7.57% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3983` |
| 5 | 3.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5549` |
| 6 | 3.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2121` |
| 7 | 2.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3847` |
| 8 | 2.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 1.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6357` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3981` |
| 13 | 1.02% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 14 | 1.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5641` |
| 15 | 1.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.51% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.51% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 19 | 0.49% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 20 | 0.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 21 | 0.44% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 22 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3850` |
| 23 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.32% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 25 | 0.30% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.30% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 27 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.25% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isJoinNonEmpty(VoxelShape, VoxelShape, BooleanOp) line: 1320` |
| 30 | 0.25% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |

GC pauses: 0, total 0 ms

