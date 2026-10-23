# JVM hotspots (JFR ExecutionSample)

Samples: 5838

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 9.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 5.77% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 4.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 4.21% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 6 | 4.18% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 3.05% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1661` |
| 8 | 2.89% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 9 | 1.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2099` |
| 10 | 1.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1564` |
| 11 | 1.15% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 12 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.74% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 15 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.70% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 17 | 0.60% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 19 | 0.57% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 20 | 0.51% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 21 | 0.51% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 22 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1550` |
| 23 | 0.43% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1615` |
| 24 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2126` |
| 25 | 0.33% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 26 | 0.33% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 240` |
| 27 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 28 | 0.31% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 29 | 0.29% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 30 | 0.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 31 | 0.29% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 576` |
| 32 | 0.29% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 33 | 0.27% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 720` |
| 34 | 0.27% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2192` |
| 35 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 36 | 0.24% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 37 | 0.24% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1325` |
| 38 | 0.24% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.24% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 40 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1557` |
| 41 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2098` |
| 42 | 0.21% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 43 | 0.21% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2071` |
| 45 | 0.21% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 82` |
| 46 | 0.21% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 47 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1254` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2129` |
| 49 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 50 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 51 | 0.17% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2070` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2072` |
| 54 | 0.17% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 55 | 0.17% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1576` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1611` |
| 58 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2074` |
| 59 | 0.17% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1617` |
| 60 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.22% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 9.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 9.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 4 | 5.34% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2193` |
| 5 | 4.80% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 4.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2126` |
| 7 | 3.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1585` |
| 8 | 2.89% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 9 | 2.67% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 10 | 1.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.63% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 12 | 1.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 13 | 1.13% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1584` |
| 15 | 0.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 19 | 0.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 0.55% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |
| 21 | 0.51% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 22 | 0.34% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 23 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2092` |
| 24 | 0.33% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 279` |
| 25 | 0.31% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.31% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.31% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 901` |
| 28 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1565` |
| 29 | 0.29% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 434` |
| 30 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1264` |

GC pauses: 0, total 0 ms

