# JVM hotspots (JFR ExecutionSample)

Samples: 5399

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 7.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 3 | 7.39% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 5.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 5.09% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 6 | 4.37% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 3.11% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 8 | 2.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1851` |
| 9 | 1.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1837` |
| 10 | 1.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2386` |
| 11 | 1.56% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1948` |
| 12 | 1.04% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 13 | 0.94% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 14 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 0.87% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 16 | 0.83% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 17 | 0.80% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.78% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1902` |
| 19 | 0.72% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 20 | 0.65% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 21 | 0.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 22 | 0.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1844` |
| 23 | 0.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 24 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 25 | 0.57% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 26 | 0.52% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 806` |
| 27 | 0.52% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 28 | 0.50% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 29 | 0.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2413` |
| 30 | 0.46% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 31 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 32 | 0.44% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 33 | 0.41% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 34 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2431` |
| 35 | 0.37% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 599` |
| 36 | 0.35% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 37 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1339` |
| 38 | 0.33% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 39 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 152` |
| 40 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 41 | 0.28% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1898` |
| 42 | 0.28% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1413` |
| 43 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2358` |
| 44 | 0.22% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 45 | 0.22% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 82` |
| 46 | 0.22% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 47 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2379` |
| 48 | 0.20% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 49 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1342` |
| 50 | 0.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 51 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 52 | 0.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 53 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1874` |
| 54 | 0.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 55 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 56 | 0.15% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 123` |
| 57 | 0.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 58 | 0.15% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 59 | 0.13% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 60 | 0.13% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.65% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 14.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 7.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 4 | 5.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2413` |
| 5 | 5.28% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 6 | 4.37% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 4.22% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2480` |
| 8 | 3.11% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 9 | 2.19% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 10 | 1.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1872` |
| 11 | 1.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1871` |
| 12 | 1.02% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 13 | 1.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 0.94% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 901` |
| 15 | 0.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 0.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.70% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1852` |
| 19 | 0.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1025` |
| 20 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 21 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1352` |
| 23 | 0.50% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 24 | 0.44% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 25 | 0.44% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 26 | 0.41% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 27 | 0.37% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 28 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 29 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 30 | 0.33% | `net.minecraft.util.Mth.packDegrees(float) line: 183` |

GC pauses: 0, total 0 ms

