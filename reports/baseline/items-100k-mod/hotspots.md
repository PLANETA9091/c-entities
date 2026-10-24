# JVM hotspots (JFR ExecutionSample)

Samples: 4789

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 6.72% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 3 | 6.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 5.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 5 | 5.41% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 2.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1888` |
| 7 | 2.07% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1953` |
| 8 | 2.00% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2530` |
| 9 | 1.65% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 10 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2437` |
| 11 | 1.06% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 12 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.00% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 14 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.96% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 16 | 0.92% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 17 | 0.90% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1902` |
| 18 | 0.84% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1927` |
| 20 | 0.77% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 21 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 22 | 0.63% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 23 | 0.52% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 24 | 0.50% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 25 | 0.46% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 26 | 0.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1895` |
| 27 | 0.46% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 28 | 0.46% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 29 | 0.44% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 599` |
| 30 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 31 | 0.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1925` |
| 32 | 0.42% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 33 | 0.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1999` |
| 34 | 0.42% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 35 | 0.40% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1432` |
| 36 | 0.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 120` |
| 37 | 0.40% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 38 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1358` |
| 39 | 0.33% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 40 | 0.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 41 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2464` |
| 42 | 0.29% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 43 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 44 | 0.29% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 240` |
| 45 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 46 | 0.27% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 282` |
| 47 | 0.27% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 813` |
| 48 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 49 | 0.25% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 50 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 130` |
| 51 | 0.25% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 52 | 0.23% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 82` |
| 53 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2482` |
| 54 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1914` |
| 55 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 56 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 57 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 58 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1893` |
| 59 | 0.19% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1949` |
| 60 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1889` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.75% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 2 | 9.56% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 6.68% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 4 | 6.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2464` |
| 5 | 6.12% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 6 | 5.16% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2531` |
| 7 | 3.80% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1922` |
| 9 | 2.28% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 10 | 2.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2430` |
| 11 | 1.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 13 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 1.00% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 0.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 16 | 0.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 17 | 0.69% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 18 | 0.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1923` |
| 19 | 0.67% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1032` |
| 20 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 21 | 0.52% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 22 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1903` |
| 23 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 24 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 25 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 26 | 0.46% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 27 | 0.44% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 28 | 0.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 29 | 0.38% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 30 | 0.38% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |

GC pauses: 0, total 0 ms

