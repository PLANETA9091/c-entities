# JVM hotspots (JFR ExecutionSample)

Samples: 6506

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.59% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 2 | 11.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 3 | 7.70% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4034` |
| 4 | 5.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 5 | 4.15% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2750` |
| 6 | 3.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2647` |
| 7 | 3.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.54% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2107` |
| 9 | 2.24% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 10 | 1.98% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3138` |
| 11 | 1.91% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 12 | 1.57% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 13 | 1.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2010` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2061` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3165` |
| 16 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.80% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 615` |
| 19 | 0.78% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 20 | 0.63% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 22 | 0.57% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 23 | 0.54% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 24 | 0.49% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 25 | 0.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 26 | 0.45% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 27 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2003` |
| 28 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1422` |
| 29 | 0.38% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 30 | 0.37% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 31 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2702` |
| 32 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2683` |
| 33 | 0.34% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 34 | 0.32% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 246` |
| 35 | 0.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3284` |
| 36 | 0.29% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2057` |
| 37 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1403` |
| 38 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 39 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 40 | 0.20% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 41 | 0.20% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1493` |
| 42 | 0.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 43 | 0.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 44 | 0.18% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3149` |
| 45 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 46 | 0.15% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3171` |
| 47 | 0.14% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 48 | 0.14% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 49 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2640` |
| 50 | 0.14% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 51 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2622` |
| 52 | 0.14% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 53 | 0.12% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 54 | 0.12% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 55 | 0.12% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 123` |
| 56 | 0.12% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 57 | 0.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2620` |
| 58 | 0.12% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 59 | 0.11% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 60 | 0.11% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 2 | 11.39% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2751` |
| 3 | 7.78% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 4 | 6.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2683` |
| 5 | 5.56% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 4.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2640` |
| 7 | 3.54% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2355` |
| 8 | 3.46% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 2.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2031` |
| 10 | 2.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3286` |
| 11 | 2.27% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 12 | 1.91% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3148` |
| 13 | 1.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2001` |
| 14 | 1.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 1.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2030` |
| 16 | 1.01% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1056` |
| 17 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.81% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 19 | 0.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2011` |
| 20 | 0.65% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 0.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 23 | 0.51% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 24 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2010` |
| 25 | 0.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 26 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 28 | 0.43% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 29 | 0.34% | `net.minecraft.util.Mth.packDegrees(float) line: 183` |
| 30 | 0.32% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |

GC pauses: 0, total 0 ms

