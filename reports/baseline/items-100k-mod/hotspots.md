# JVM hotspots (JFR ExecutionSample)

Samples: 4223

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 8.86% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3870` |
| 3 | 4.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4066` |
| 4 | 3.41% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6277` |
| 5 | 2.84% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 2.53% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 7 | 2.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 2.34% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 9 | 1.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3357` |
| 10 | 1.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 12 | 1.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.07% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.73% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 17 | 0.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 18 | 0.57% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2206` |
| 19 | 0.54% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 20 | 0.52% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 21 | 0.52% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.47% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 23 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4068` |
| 24 | 0.43% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 25 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2108` |
| 26 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2097` |
| 27 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 28 | 0.33% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 29 | 0.33% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 30 | 0.31% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 31 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 33 | 0.28% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 34 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2006` |
| 36 | 0.28% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 37 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3394` |
| 39 | 0.26% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3910` |
| 40 | 0.24% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 41 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 42 | 0.21% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3363` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2013` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2054` |
| 46 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 47 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 48 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 49 | 0.19% | `java.util.Collection.stream() line: 748` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 6095` |
| 51 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 52 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 53 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 54 | 0.17% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 55 | 0.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2007` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 246` |
| 58 | 0.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 59 | 0.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 60 | 0.17% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.28% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6277` |
| 2 | 9.42% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 5.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4068` |
| 4 | 5.04% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 6 | 3.13% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 3.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2011` |
| 8 | 2.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3394` |
| 9 | 2.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 10 | 2.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 2.18% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6278` |
| 12 | 1.04% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 13 | 0.99% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 0.85% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 16 | 0.76% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 17 | 0.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2129` |
| 18 | 0.57% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 699` |
| 19 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.50% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.45% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 22 | 0.43% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 23 | 0.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 24 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2023` |
| 25 | 0.40% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.28% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 30 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |

GC pauses: 0, total 0 ms

