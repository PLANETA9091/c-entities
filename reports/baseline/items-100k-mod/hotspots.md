# JVM hotspots (JFR ExecutionSample)

Samples: 2912

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 5.22% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 3 | 3.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2032` |
| 4 | 3.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 3.43% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 3.23% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 7 | 2.51% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4004` |
| 8 | 2.34% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 9 | 2.09% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 10 | 2.06% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 11 | 1.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 12 | 1.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2107` |
| 13 | 1.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2078` |
| 14 | 1.48% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4044` |
| 15 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 1.03% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 17 | 1.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3491` |
| 18 | 0.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2118` |
| 19 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.89% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4035` |
| 21 | 0.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1428` |
| 22 | 0.65% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2216` |
| 23 | 0.65% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2064` |
| 25 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 26 | 0.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 27 | 0.55% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 28 | 0.41% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 29 | 0.41% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 433` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 31 | 0.34% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 32 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3528` |
| 33 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 34 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 35 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 36 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 37 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3497` |
| 38 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 39 | 0.24% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 40 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 41 | 0.21% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 42 | 0.21% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 43 | 0.21% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 44 | 0.21% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 45 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 46 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 47 | 0.17% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 48 | 0.17% | `java.lang.Class.cast(Object) line: 3491` |
| 49 | 0.17% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 50 | 0.17% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.getTailOpaque() line: 80` |
| 51 | 0.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 52 | 0.17% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 53 | 0.17% | `it.unimi.dsi.fastutil.ints.Int2IntOpenHashMap.find(int) line: 238` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3536` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2097` |
| 56 | 0.17% | `net.minecraft.nbt.ListTag.getIdToWrite(List, int) line: 37` |
| 57 | 0.14% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isMergeEmpty(CachedShapeData, CachedShapeData, CollisionUtil$MergedVoxelCoordinateList, CollisionUtil$MergedVoxelCoordinateList, CollisionUtil$MergedVoxelCoordinateList, int) line: 1136` |
| 58 | 0.14% | `java.util.HashMap.getNode(Object) line: 588` |
| 59 | 0.14% | `net.minecraft.world.level.biome.Climate$RTree.buildParameterSpace(List) line: 455` |
| 60 | 0.14% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 2 | 9.92% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 3 | 5.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 4 | 5.08% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 5 | 4.53% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4005` |
| 6 | 3.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3528` |
| 7 | 3.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2021` |
| 8 | 2.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2032` |
| 9 | 2.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 10 | 2.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 2.23% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 2.09% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6457` |
| 13 | 1.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1062` |
| 15 | 1.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2033` |
| 16 | 1.00% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4004` |
| 17 | 0.96% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 18 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2139` |
| 20 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 0.55% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.55% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6458` |
| 24 | 0.48% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2119` |
| 26 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2118` |
| 27 | 0.41% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6459` |
| 28 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2101` |
| 29 | 0.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 30 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

