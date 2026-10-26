# JVM hotspots (JFR ExecutionSample)

Samples: 3900

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.49% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6650` |
| 2 | 7.67% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2110` |
| 4 | 2.82% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 2.64% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 6 | 2.56% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 7 | 2.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 8 | 1.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2035` |
| 10 | 1.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 11 | 1.44% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 12 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2081` |
| 14 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3494` |
| 15 | 1.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 1.15% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 17 | 0.97% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4047` |
| 18 | 0.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 19 | 0.82% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.72% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 21 | 0.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3531` |
| 22 | 0.44% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2219` |
| 23 | 0.38% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 24 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 25 | 0.38% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 26 | 0.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 27 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 28 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 29 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2067` |
| 30 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 31 | 0.36% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 32 | 0.36% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 33 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 34 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1431` |
| 35 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 36 | 0.31% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |
| 37 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1415` |
| 38 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2039` |
| 39 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2019` |
| 40 | 0.26% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 624` |
| 41 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3500` |
| 42 | 0.26% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 43 | 0.26% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 44 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 45 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2121` |
| 46 | 0.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 47 | 0.23% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 48 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 49 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 50 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2045` |
| 51 | 0.21% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4038` |
| 52 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 53 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 54 | 0.18% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 55 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 56 | 0.18% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 57 | 0.18% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 58 | 0.18% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 59 | 0.18% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 60 | 0.18% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 123` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 2 | 10.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 3 | 6.46% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6650` |
| 4 | 4.23% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 5 | 3.64% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.05% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 3.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3531` |
| 8 | 2.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2024` |
| 9 | 2.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.41% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4038` |
| 13 | 1.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1065` |
| 14 | 1.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 15 | 1.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2035` |
| 16 | 0.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.79% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 18 | 0.72% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 19 | 0.62% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2142` |
| 21 | 0.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2036` |
| 22 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 23 | 0.44% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.44% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.38% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.38% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 27 | 0.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 28 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2121` |
| 29 | 0.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 30 | 0.31% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6651` |

GC pauses: 0, total 0 ms

