# JVM hotspots (JFR ExecutionSample)

Samples: 5826

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 10.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 5.60% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 5.20% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 5 | 4.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 4.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 7 | 4.17% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.90% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1955` |
| 9 | 1.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2393` |
| 10 | 1.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1858` |
| 11 | 1.08% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 12 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.79% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 0.70% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 16 | 0.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.60% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 18 | 0.58% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1909` |
| 19 | 0.57% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 20 | 0.50% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 21 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 22 | 0.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1844` |
| 23 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 24 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1851` |
| 25 | 0.36% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 26 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 27 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 28 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2420` |
| 29 | 0.33% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2486` |
| 30 | 0.27% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1420` |
| 31 | 0.27% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 32 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 33 | 0.26% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 599` |
| 34 | 0.26% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 813` |
| 35 | 0.26% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 36 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 37 | 0.24% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.24% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 39 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 40 | 0.22% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 41 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2364` |
| 42 | 0.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1905` |
| 44 | 0.21% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1870` |
| 46 | 0.21% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 47 | 0.21% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 240` |
| 48 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 49 | 0.17% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 50 | 0.17% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 51 | 0.15% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 52 | 0.15% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.15% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1349` |
| 55 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2386` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2368` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2392` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1911` |
| 59 | 0.14% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 60 | 0.14% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 82` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.26% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 10.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 3 | 10.04% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.29% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2487` |
| 5 | 5.20% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 6 | 4.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2420` |
| 7 | 4.29% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 3.09% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1879` |
| 9 | 2.99% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 10 | 1.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1878` |
| 12 | 1.08% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 13 | 0.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.84% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 0.81% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 16 | 0.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.60% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 19 | 0.50% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 901` |
| 20 | 0.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 21 | 0.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 22 | 0.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1032` |
| 23 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2386` |
| 24 | 0.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 25 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1859` |
| 26 | 0.34% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.31% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 680` |
| 28 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 29 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1359` |
| 30 | 0.27% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |

GC pauses: 0, total 0 ms

