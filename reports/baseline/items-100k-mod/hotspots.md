# JVM hotspots (JFR ExecutionSample)

Samples: 5021

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 8.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 7.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 5.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 5 | 5.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 3.94% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 3.64% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 8 | 2.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1885` |
| 9 | 2.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2420` |
| 10 | 1.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1871` |
| 11 | 1.69% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1982` |
| 12 | 1.49% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 13 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.14% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 15 | 0.96% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 16 | 0.80% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.72% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 18 | 0.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 19 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.68% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 21 | 0.62% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 22 | 0.60% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 23 | 0.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 24 | 0.58% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 25 | 0.52% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 26 | 0.52% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 27 | 0.48% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 28 | 0.46% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 29 | 0.46% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1936` |
| 30 | 0.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1878` |
| 31 | 0.40% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 813` |
| 32 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2465` |
| 33 | 0.36% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 82` |
| 34 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 152` |
| 35 | 0.34% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 36 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 37 | 0.34% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 599` |
| 38 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 39 | 0.30% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 40 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2447` |
| 41 | 0.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 42 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1908` |
| 43 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 44 | 0.24% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1432` |
| 45 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2413` |
| 46 | 0.22% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 47 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1358` |
| 49 | 0.20% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2513` |
| 50 | 0.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 51 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 52 | 0.16% | `java.util.LinkedHashMap$LinkedHashIterator.nextNode() line: 1023` |
| 53 | 0.14% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 54 | 0.14% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 55 | 0.14% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1932` |
| 56 | 0.14% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 57 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2393` |
| 58 | 0.12% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 59 | 0.12% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.12% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.38% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 14.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 8.23% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 4 | 6.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2447` |
| 5 | 5.28% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 6 | 4.20% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 3.62% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2514` |
| 8 | 3.60% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 9 | 3.09% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 10 | 1.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1906` |
| 11 | 1.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 0.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 14 | 0.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1905` |
| 15 | 0.84% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 18 | 0.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1886` |
| 19 | 0.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 20 | 0.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 21 | 0.56% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 901` |
| 22 | 0.52% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 23 | 0.48% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 24 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1371` |
| 25 | 0.42% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 434` |
| 26 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 27 | 0.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 28 | 0.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1032` |
| 29 | 0.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 30 | 0.34% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |

GC pauses: 0, total 0 ms

