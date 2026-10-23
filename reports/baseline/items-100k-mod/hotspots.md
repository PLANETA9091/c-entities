# JVM hotspots (JFR ExecutionSample)

Samples: 3509

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 9.60% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 4.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 4 | 3.08% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 2.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1550` |
| 6 | 2.02% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2192` |
| 7 | 1.74% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 8 | 1.65% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 9 | 1.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2099` |
| 11 | 1.40% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 12 | 1.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.11% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 14 | 1.05% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 15 | 1.03% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1589` |
| 16 | 1.00% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 17 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 19 | 0.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1557` |
| 20 | 0.68% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 21 | 0.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2126` |
| 22 | 0.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1251` |
| 23 | 0.60% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 25 | 0.57% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 26 | 0.51% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 576` |
| 27 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 28 | 0.46% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 29 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1587` |
| 30 | 0.40% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 31 | 0.40% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 720` |
| 32 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1992` |
| 33 | 0.40% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 34 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2144` |
| 35 | 0.40% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 36 | 0.37% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 37 | 0.34% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 38 | 0.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 39 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1991` |
| 40 | 0.31% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 282` |
| 41 | 0.31% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1661` |
| 42 | 0.28% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 123` |
| 43 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1576` |
| 44 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 45 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1564` |
| 46 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2074` |
| 47 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 48 | 0.23% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 49 | 0.23% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1325` |
| 50 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2092` |
| 51 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 52 | 0.23% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 53 | 0.23% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 54 | 0.23% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 240` |
| 55 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 56 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 57 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 58 | 0.20% | `com.google.common.collect.ImmutableMap.keySet() line: 71` |
| 59 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2081` |
| 60 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2071` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.36% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 9.35% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2193` |
| 3 | 5.59% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 5.53% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 5 | 5.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2126` |
| 6 | 4.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.74% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 8 | 2.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2092` |
| 9 | 2.11% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 11 | 1.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 12 | 1.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 13 | 1.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 14 | 1.11% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 15 | 0.94% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |
| 16 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 17 | 0.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 19 | 0.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.54% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 23 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1585` |
| 24 | 0.46% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 26 | 0.46% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 27 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.43% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 29 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1264` |
| 30 | 0.40% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |

GC pauses: 0, total 0 ms

