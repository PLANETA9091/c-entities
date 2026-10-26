# JVM hotspots (JFR ExecutionSample)

Samples: 3924

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.95% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 8.92% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4001` |
| 3 | 4.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 4 | 2.80% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 5 | 2.78% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 6 | 2.70% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.01% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 1.89% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 9 | 1.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2075` |
| 10 | 1.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2029` |
| 12 | 1.25% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3488` |
| 15 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.64% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.59% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 18 | 0.54% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 19 | 0.51% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 20 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 21 | 0.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 22 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 23 | 0.38% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4041` |
| 24 | 0.36% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2213` |
| 25 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2104` |
| 26 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 27 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3525` |
| 28 | 0.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 29 | 0.33% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 6238` |
| 30 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2013` |
| 31 | 0.31% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 32 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 33 | 0.28% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 34 | 0.28% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 35 | 0.28% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 36 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.28% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 38 | 0.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4199` |
| 39 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 40 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 42 | 0.25% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 43 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 44 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 45 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 762` |
| 46 | 0.23% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4032` |
| 47 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 48 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 49 | 0.20% | `java.math.BigInteger.montReduce(int[], int[], int, int) line: 3290` |
| 50 | 0.18% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 51 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 52 | 0.18% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3494` |
| 54 | 0.18% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2032` |
| 56 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 57 | 0.15% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 58 | 0.15% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 59 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 741` |
| 60 | 0.15% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState) line: 87` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.36% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 2 | 9.94% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 5.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4199` |
| 4 | 4.92% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.93% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 6 | 2.78% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 7 | 2.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2018` |
| 8 | 2.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 2.65% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 2.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3525` |
| 11 | 1.78% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6421` |
| 12 | 1.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 13 | 1.15% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 1.04% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2029` |
| 16 | 0.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.59% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 18 | 0.54% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 19 | 0.54% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 20 | 0.51% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 21 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2136` |
| 22 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 23 | 0.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2030` |
| 24 | 0.43% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.41% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.41% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4001` |
| 27 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.38% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |
| 29 | 0.33% | `net.minecraft.world.level.levelgen.synth.NoiseStack$SmearedPerlin.addToVolume(DensityBuffer, DensityVolume, double, double, float) line: 153` |
| 30 | 0.31% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |

GC pauses: 0, total 0 ms

