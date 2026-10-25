# JVM hotspots (JFR ExecutionSample)

Samples: 5176

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 7.86% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3671` |
| 3 | 6.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3817` |
| 4 | 6.05% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3681` |
| 5 | 5.08% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5439` |
| 6 | 3.69% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 7 | 1.85% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 1.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3116` |
| 9 | 1.53% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 1.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 11 | 1.29% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 12 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.04% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 14 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.93% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 16 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 0.79% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2206` |
| 18 | 0.58% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.50% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 20 | 0.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.48% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 22 | 0.46% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 24 | 0.41% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3711` |
| 25 | 0.41% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 26 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 27 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 28 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2097` |
| 29 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 30 | 0.25% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 31 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 33 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 34 | 0.25% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 36 | 0.23% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 37 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 38 | 0.21% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 39 | 0.21% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 40 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 41 | 0.19% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 42 | 0.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3819` |
| 43 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 44 | 0.17% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 45 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2108` |
| 46 | 0.17% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 197` |
| 47 | 0.15% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3704` |
| 48 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2006` |
| 49 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3153` |
| 50 | 0.15% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 51 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3122` |
| 52 | 0.15% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 53 | 0.14% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 54 | 0.14% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 55 | 0.14% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 765` |
| 56 | 0.14% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 57 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3072` |
| 58 | 0.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3815` |
| 59 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2093` |
| 60 | 0.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.84% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 2 | 12.19% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5439` |
| 3 | 6.99% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 5.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 5 | 3.69% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 6 | 3.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3819` |
| 7 | 2.74% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3153` |
| 9 | 1.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2011` |
| 11 | 1.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 12 | 1.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5440` |
| 14 | 1.04% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 15 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 17 | 0.85% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 18 | 0.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2129` |
| 19 | 0.71% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 20 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.50% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 699` |
| 22 | 0.41% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 24 | 0.39% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 25 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.27% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 27 | 0.27% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.27% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 29 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.23% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |

GC pauses: 0, total 0 ms

