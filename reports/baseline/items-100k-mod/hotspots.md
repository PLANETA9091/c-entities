# JVM hotspots (JFR ExecutionSample)

Samples: 8780

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 2 | 11.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3067` |
| 3 | 10.50% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2967` |
| 4 | 5.67% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2938` |
| 5 | 3.10% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 6 | 2.98% | `jdk.internal.reflect.MethodHandleObjectFieldAccessorImpl.get(Object) line: 57` |
| 7 | 2.85% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 2.70% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 9 | 1.94% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 10 | 1.85% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2050` |
| 11 | 1.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1953` |
| 12 | 1.07% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 13 | 1.06% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2963` |
| 14 | 0.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2488` |
| 15 | 0.85% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 16 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 17 | 0.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 0.62% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 19 | 0.58% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.getBoolean(Object) line: 63` |
| 20 | 0.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.57% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 22 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 23 | 0.43% | `jdk.internal.reflect.MethodHandleIntegerFieldAccessorImpl.getInt(Object) line: 79` |
| 24 | 0.40% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 25 | 0.38% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1462` |
| 26 | 0.34% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 27 | 0.33% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.get(Object) line: 55` |
| 28 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 29 | 0.31% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2934` |
| 30 | 0.31% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 31 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1939` |
| 32 | 0.30% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2004` |
| 33 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1398` |
| 34 | 0.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.26% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2972` |
| 36 | 0.24% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 82` |
| 37 | 0.23% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2581` |
| 38 | 0.23% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.22% | `jdk.internal.reflect.MethodHandleIntegerFieldAccessorImpl.setInt(Object, int) line: 166` |
| 40 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 41 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 42 | 0.21% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 43 | 0.21% | `java.lang.invoke.Invokers.checkCustomized(MethodHandle) line: 625` |
| 44 | 0.21% | `java.lang.invoke.MethodHandle.type() line: 470` |
| 45 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 46 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2515` |
| 47 | 0.18% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 48 | 0.18% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2943` |
| 49 | 0.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2199` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 242` |
| 52 | 0.16% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 53 | 0.16% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 54 | 0.16% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 55 | 0.16% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 56 | 0.16% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3066` |
| 58 | 0.15% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1382` |
| 60 | 0.14% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.12% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 2 | 15.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 3 | 11.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2515` |
| 5 | 3.41% | `java.lang.reflect.Field.get(Object) line: 440` |
| 6 | 3.10% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.71% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2582` |
| 8 | 2.69% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 9 | 2.18% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 10 | 2.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1974` |
| 11 | 1.94% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 12 | 1.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 14 | 1.07% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 15 | 1.04% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 16 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 0.64% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1042` |
| 18 | 0.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 0.60% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.get(Object) line: 55` |
| 20 | 0.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1973` |
| 21 | 0.57% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 22 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 23 | 0.48% | `java.lang.reflect.Field.getInt(Object) line: 626` |
| 24 | 0.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 901` |
| 25 | 0.35% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 690` |
| 26 | 0.34% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.33% | `jdk.internal.reflect.MethodHandleObjectFieldAccessorImpl.get(Object) line: 57` |
| 28 | 0.33% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2967` |
| 29 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 30 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1954` |

GC pauses: 0, total 0 ms

