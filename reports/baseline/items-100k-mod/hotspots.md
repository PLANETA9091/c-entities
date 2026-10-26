# JVM hotspots (JFR ExecutionSample)

Samples: 4046

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.68% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 7.02% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 3 | 5.29% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6863` |
| 4 | 4.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 5 | 2.62% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 6 | 2.57% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.13% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 8 | 2.05% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2035` |
| 10 | 1.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2081` |
| 11 | 1.51% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3494` |
| 14 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.91% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.84% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 18 | 0.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2121` |
| 19 | 0.54% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 20 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1431` |
| 21 | 0.47% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 22 | 0.44% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.42% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4047` |
| 24 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 25 | 0.40% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 26 | 0.40% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2219` |
| 27 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2110` |
| 28 | 0.35% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 29 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 30 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3531` |
| 31 | 0.32% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 32 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3500` |
| 34 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 35 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 36 | 0.27% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.<init>(Object, MultiThreadedQueue$LinkedNode) line: 1249` |
| 37 | 0.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 38 | 0.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 39 | 0.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 40 | 0.25% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 41 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 42 | 0.22% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4038` |
| 44 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1415` |
| 45 | 0.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 46 | 0.20% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 47 | 0.20% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 49 | 0.20% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 433` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2067` |
| 51 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 52 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 53 | 0.17% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 54 | 0.17% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 55 | 0.17% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 56 | 0.17% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 57 | 0.17% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 58 | 0.17% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 59 | 0.17% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 624` |
| 60 | 0.17% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.67% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6863` |
| 2 | 8.18% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 6.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 4 | 5.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 5 | 4.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.14% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2024` |
| 8 | 2.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 2.37% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6864` |
| 10 | 2.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3531` |
| 11 | 2.13% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2035` |
| 13 | 1.21% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1065` |
| 14 | 1.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.99% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.77% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 18 | 0.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2036` |
| 19 | 0.67% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.57% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 21 | 0.57% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 22 | 0.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.47% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 24 | 0.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2142` |
| 25 | 0.40% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 26 | 0.37% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.37% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 29 | 0.32% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 126` |
| 30 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.saveChunk(ChunkAccess, boolean, Completable[]) line: 1719` |

GC pauses: 0, total 0 ms

