# JVM hotspots (JFR ExecutionSample)

Samples: 3781

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 2 | 5.55% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.78% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 4 | 3.46% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 5 | 3.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 6 | 2.01% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 1.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2417` |
| 8 | 1.90% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2463` |
| 9 | 1.80% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 10 | 1.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3895` |
| 11 | 1.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.01% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 15 | 0.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2492` |
| 16 | 0.71% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 18 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7495` |
| 19 | 0.56% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 20 | 0.56% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2601` |
| 21 | 0.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2503` |
| 22 | 0.53% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 23 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7505` |
| 24 | 0.53% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 25 | 0.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1458` |
| 26 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 27 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3932` |
| 28 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7499` |
| 29 | 0.40% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 30 | 0.37% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 31 | 0.34% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 32 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 33 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 34 | 0.29% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadWrite.set(VarHandle, Object, Object) line: 148` |
| 35 | 0.29% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 36 | 0.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4606` |
| 37 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 38 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 39 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 40 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3901` |
| 41 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 42 | 0.21% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 43 | 0.21% | `java.util.AbstractCollection.isEmpty() line: 91` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1781` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2449` |
| 46 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 47 | 0.19% | `net.minecraft.resources.DelegatingOps.empty() line: 31` |
| 48 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 49 | 0.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4603` |
| 51 | 0.19% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 52 | 0.19% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 53 | 0.19% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2420` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2408` |
| 56 | 0.19% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 57 | 0.16% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 58 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 59 | 0.16% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 60 | 0.16% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 7.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4606` |
| 3 | 5.66% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7540` |
| 4 | 4.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7532` |
| 5 | 3.97% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2406` |
| 7 | 3.57% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 2.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3932` |
| 10 | 1.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2417` |
| 11 | 1.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 12 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 13 | 1.11% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2418` |
| 15 | 0.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 0.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2524` |
| 17 | 0.66% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 322` |
| 18 | 0.63% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7498` |
| 19 | 0.53% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 20 | 0.48% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.42% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 23 | 0.40% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 24 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7490` |
| 25 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 26 | 0.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 27 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 28 | 0.37% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.34% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 30 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

