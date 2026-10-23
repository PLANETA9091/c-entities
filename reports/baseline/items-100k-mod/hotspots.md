# JVM hotspots (JFR ExecutionSample)

Samples: 8812

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 2 | 7.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 7.25% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 6.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 5 | 5.42% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2186` |
| 6 | 4.93% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 4.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2093` |
| 8 | 4.09% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 9 | 2.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1558` |
| 10 | 2.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1544` |
| 11 | 2.10% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1609` |
| 12 | 2.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1583` |
| 13 | 1.90% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 14 | 1.61% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 15 | 1.04% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 16 | 1.03% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 17 | 0.82% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1551` |
| 19 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 20 | 0.70% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 21 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 22 | 0.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 23 | 0.61% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 24 | 0.59% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 576` |
| 25 | 0.53% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1655` |
| 26 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 27 | 0.49% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 28 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2138` |
| 30 | 0.40% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 31 | 0.40% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1605` |
| 32 | 0.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 33 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 82` |
| 34 | 0.37% | `net.minecraft.world.entity.Entity.position() line: 5043` |
| 35 | 0.34% | `net.minecraft.world.entity.Entity.getPassengers() line: 4806` |
| 36 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 152` |
| 37 | 0.32% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 38 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1581` |
| 39 | 0.27% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 40 | 0.26% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 41 | 0.26% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 43 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 120` |
| 44 | 0.24% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 45 | 0.24% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 240` |
| 46 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2064` |
| 47 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 48 | 0.17% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 720` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1325` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1986` |
| 51 | 0.17% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 282` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1611` |
| 53 | 0.16% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1570` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2080` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2120` |
| 57 | 0.15% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 58 | 0.14% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 59 | 0.14% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |
| 60 | 0.14% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 2 | 8.75% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 3 | 7.57% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 7.19% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2187` |
| 5 | 6.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2120` |
| 6 | 6.73% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 5.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2086` |
| 8 | 4.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 4.05% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 10 | 2.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1578` |
| 11 | 1.90% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 12 | 1.79% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 13 | 1.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1559` |
| 14 | 1.02% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 15 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 0.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1579` |
| 18 | 0.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 0.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 20 | 0.61% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 21 | 0.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 22 | 0.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 23 | 0.53% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 24 | 0.42% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 434` |
| 25 | 0.40% | `net.minecraft.util.Mth.packDegrees(float) line: 183` |
| 26 | 0.37% | `net.minecraft.world.entity.Entity.trackingPosition() line: 5047` |
| 27 | 0.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 28 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 119` |
| 29 | 0.32% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 901` |
| 30 | 0.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |

GC pauses: 0, total 0 ms

