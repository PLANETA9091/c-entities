# JVM hotspots (JFR ExecutionSample)

Samples: 4430

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.91% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 9.26% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3846` |
| 3 | 5.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4024` |
| 4 | 3.88% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5646` |
| 5 | 3.86% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 6 | 2.42% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 7 | 2.10% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 2.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3352` |
| 9 | 1.76% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 10 | 1.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 11 | 1.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 14 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 16 | 0.86% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.65% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2206` |
| 18 | 0.61% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 19 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 20 | 0.47% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 21 | 0.45% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3877` |
| 22 | 0.41% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 23 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3389` |
| 24 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 25 | 0.36% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 26 | 0.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 27 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2097` |
| 28 | 0.32% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2054` |
| 30 | 0.29% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 31 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 33 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3358` |
| 34 | 0.25% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 35 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 36 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 37 | 0.23% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 38 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2108` |
| 39 | 0.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 40 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 41 | 0.20% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 42 | 0.20% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 123` |
| 43 | 0.20% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 44 | 0.20% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3842` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4026` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 618` |
| 49 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 50 | 0.18% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 51 | 0.18% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 52 | 0.16% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 53 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2006` |
| 55 | 0.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 5500` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2011` |
| 58 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 59 | 0.14% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 60 | 0.14% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.30% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5646` |
| 2 | 10.14% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 6.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 5.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4026` |
| 5 | 4.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 6 | 3.86% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 7 | 3.48% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2011` |
| 9 | 2.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 2.17% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5647` |
| 11 | 1.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3389` |
| 12 | 1.13% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 13 | 1.08% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 0.90% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 0.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2129` |
| 17 | 0.72% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 18 | 0.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 19 | 0.61% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 20 | 0.54% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 3033` |
| 21 | 0.52% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 24 | 0.38% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 699` |
| 25 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 26 | 0.36% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 27 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.27% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, NoiseChunk, RandomState, BiomeManager, Set, MaterialRule, ServerLevel) line: 227` |
| 30 | 0.23% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |

GC pauses: 0, total 0 ms

