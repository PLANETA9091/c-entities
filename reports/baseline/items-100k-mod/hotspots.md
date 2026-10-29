# JVM hotspots (JFR ExecutionSample)

Samples: 5614

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8130` |
| 2 | 10.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4448` |
| 3 | 6.45% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.10% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8077` |
| 7 | 2.92% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3143` |
| 8 | 2.51% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3035` |
| 11 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.94% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4454` |
| 14 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4485` |
| 15 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.75% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 17 | 0.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2256` |
| 18 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8091` |
| 19 | 0.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5168` |
| 20 | 0.57% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7773` |
| 21 | 0.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2275` |
| 22 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5166` |
| 23 | 0.46% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3096` |
| 24 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 25 | 0.43% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1907` |
| 27 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4493` |
| 28 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 29 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 30 | 0.29% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 31 | 0.29% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.29% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 33 | 0.29% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 34 | 0.29% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 276` |
| 35 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2933` |
| 36 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8081` |
| 37 | 0.25% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 38 | 0.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 39 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 40 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4397` |
| 41 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4517` |
| 42 | 0.21% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3098` |
| 43 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 44 | 0.20% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 45 | 0.20% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2251` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7771` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4309` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3028` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8085` |
| 51 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 52 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 53 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 54 | 0.16% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 55 | 0.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 56 | 0.16% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8088` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2936` |
| 59 | 0.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 60 | 0.14% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.73% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.48% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4485` |
| 5 | 4.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8126` |
| 6 | 4.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8118` |
| 7 | 3.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5168` |
| 8 | 3.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3056` |
| 9 | 2.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2938` |
| 10 | 2.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1748` |
| 11 | 1.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 0.91% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3055` |
| 16 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2260` |
| 18 | 0.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3035` |
| 19 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8084` |
| 20 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.53% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.52% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 563` |
| 23 | 0.43% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.32% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3036` |
| 27 | 0.29% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 539` |
| 28 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.27% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 30 | 0.25% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |

GC pauses: 0, total 0 ms

