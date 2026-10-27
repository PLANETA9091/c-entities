# JVM hotspots (JFR ExecutionSample)

Samples: 4829

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7176` |
| 2 | 6.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4236` |
| 3 | 5.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 5.61% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7123` |
| 5 | 3.11% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2114` |
| 6 | 2.90% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3527` |
| 8 | 1.82% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7137` |
| 11 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 0.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 14 | 0.79% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 0.75% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2252` |
| 16 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1464` |
| 17 | 0.64% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 0.60% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 19 | 0.58% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 20 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7127` |
| 21 | 0.52% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 22 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 23 | 0.48% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 24 | 0.48% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 25 | 0.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4238` |
| 26 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2154` |
| 28 | 0.33% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 29 | 0.31% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 30 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3564` |
| 31 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 32 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3533` |
| 34 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 35 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 36 | 0.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 37 | 0.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 38 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1448` |
| 39 | 0.23% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.setBoolean(Object, boolean) line: 132` |
| 40 | 0.21% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 41 | 0.21% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7131` |
| 42 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2143` |
| 43 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 44 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 45 | 0.19% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 46 | 0.19% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 47 | 0.19% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.19% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2052` |
| 50 | 0.17% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 51 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 52 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 765` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3572` |
| 54 | 0.17% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2133` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2060` |
| 57 | 0.14% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 58 | 0.14% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 59 | 0.14% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(NoiseChunk, ChunkAccess) line: 465` |
| 60 | 0.14% | `java.util.AbstractCollection.isEmpty() line: 91` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.90% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 7.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7172` |
| 3 | 6.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 6.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4238` |
| 5 | 4.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7164` |
| 6 | 4.10% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 3.04% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2057` |
| 8 | 2.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 2.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3564` |
| 10 | 1.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1067` |
| 12 | 1.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7130` |
| 13 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 0.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 0.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2175` |
| 16 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7122` |
| 17 | 0.54% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 18 | 0.50% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2069` |
| 20 | 0.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2137` |
| 21 | 0.46% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1449` |
| 23 | 0.46% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 24 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 26 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 27 | 0.33% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7134` |
| 29 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |

GC pauses: 0, total 0 ms

