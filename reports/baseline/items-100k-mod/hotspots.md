# JVM hotspots (JFR ExecutionSample)

Samples: 4871

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 2 | 8.01% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 3 | 6.36% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 5.13% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 5 | 2.30% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 2.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2463` |
| 7 | 2.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3895` |
| 8 | 2.07% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.90% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2417` |
| 13 | 0.88% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 15 | 0.82% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 16 | 0.82% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2601` |
| 17 | 0.82% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7505` |
| 18 | 0.72% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 19 | 0.72% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7495` |
| 21 | 0.51% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 22 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2492` |
| 23 | 0.43% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 24 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3932` |
| 25 | 0.37% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 26 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1458` |
| 28 | 0.33% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 29 | 0.33% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 30 | 0.31% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 31 | 0.31% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 32 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 34 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 35 | 0.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4606` |
| 36 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 37 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 38 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 40 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 41 | 0.21% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 42 | 0.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 43 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 44 | 0.18% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 45 | 0.18% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 46 | 0.18% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 99` |
| 47 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 48 | 0.18% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.setBoolean(Object, boolean) line: 132` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 50 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 51 | 0.16% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 52 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.lambda$setGenerationTask$0(ChunkProgressionTask, ChunkStatus, List, ChunkAccess, Throwable) line: 1616` |
| 53 | 0.16% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2401` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4602` |
| 56 | 0.16% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 57 | 0.14% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 58 | 0.14% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 59 | 0.14% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 60 | 0.14% | `java.math.MutableBigInteger.mulsub(int[], int[], int, int, int) line: 675` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.75% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 8.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 6.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7540` |
| 4 | 5.54% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7532` |
| 5 | 5.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4606` |
| 6 | 3.59% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2406` |
| 8 | 2.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3932` |
| 9 | 2.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.89% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.58% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 12 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 13 | 0.90% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2524` |
| 14 | 0.80% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 0.72% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 16 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1782` |
| 17 | 0.55% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 18 | 0.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2417` |
| 19 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7498` |
| 20 | 0.51% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 21 | 0.51% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 322` |
| 22 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2486` |
| 23 | 0.43% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.43% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 25 | 0.37% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 26 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.35% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 29 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 30 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

