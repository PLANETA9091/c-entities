# JVM hotspots (JFR ExecutionSample)

Samples: 6112

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 2 | 9.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 5.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 4.79% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 4.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 6 | 4.27% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 7 | 2.54% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1661` |
| 8 | 2.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 9 | 2.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1564` |
| 10 | 2.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2099` |
| 11 | 1.57% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 12 | 1.18% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 13 | 1.05% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.05% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 15 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 17 | 0.65% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1615` |
| 18 | 0.64% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 19 | 0.64% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 20 | 0.61% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 21 | 0.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 22 | 0.56% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2192` |
| 23 | 0.54% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 24 | 0.52% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 25 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 26 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 27 | 0.49% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 720` |
| 28 | 0.38% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 29 | 0.36% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 82` |
| 30 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1550` |
| 31 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 32 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2126` |
| 33 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.28% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1325` |
| 35 | 0.28% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 36 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2071` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1611` |
| 38 | 0.26% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 39 | 0.25% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 40 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 41 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 42 | 0.21% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2129` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1557` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2144` |
| 46 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 576` |
| 48 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 49 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1254` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2072` |
| 52 | 0.16% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 53 | 0.16% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1251` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1991` |
| 56 | 0.15% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 57 | 0.15% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 240` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2075` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1992` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.09% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 2 | 10.01% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 9.33% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 5.35% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 5 | 5.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2126` |
| 6 | 4.42% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2193` |
| 7 | 4.25% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 8 | 3.80% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 9 | 2.95% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 10 | 2.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1585` |
| 11 | 1.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 13 | 1.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1584` |
| 14 | 1.10% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 0.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 16 | 0.64% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 901` |
| 17 | 0.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1565` |
| 19 | 0.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 20 | 0.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2092` |
| 21 | 0.54% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 22 | 0.51% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |
| 23 | 0.51% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 24 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 25 | 0.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1264` |
| 26 | 0.41% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.41% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 434` |
| 28 | 0.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 29 | 0.31% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 30 | 0.29% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |

GC pauses: 0, total 0 ms

