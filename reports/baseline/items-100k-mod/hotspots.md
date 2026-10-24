# JVM hotspots (JFR ExecutionSample)

Samples: 7988

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 2 | 10.08% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3027` |
| 3 | 9.60% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2927` |
| 4 | 6.26% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2898` |
| 5 | 4.78% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 3.33% | `jdk.internal.reflect.MethodHandleObjectFieldAccessorImpl.get(Object) line: 57` |
| 7 | 2.95% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 2.58% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 9 | 2.13% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2010` |
| 10 | 1.85% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 11 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2448` |
| 12 | 0.93% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2923` |
| 13 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1913` |
| 15 | 0.69% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 16 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 0.63% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 18 | 0.59% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 20 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.50% | `jdk.internal.reflect.MethodHandleIntegerFieldAccessorImpl.getInt(Object) line: 79` |
| 22 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 23 | 0.40% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 24 | 0.38% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 25 | 0.38% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1964` |
| 26 | 0.36% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 27 | 0.35% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2894` |
| 28 | 0.31% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.getBoolean(Object) line: 63` |
| 29 | 0.30% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2541` |
| 30 | 0.30% | `java.lang.invoke.Invokers.checkCustomized(MethodHandle) line: 625` |
| 31 | 0.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 32 | 0.29% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.get(Object) line: 55` |
| 33 | 0.28% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2159` |
| 34 | 0.26% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1443` |
| 35 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1899` |
| 36 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 82` |
| 37 | 0.24% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 38 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 39 | 0.23% | `jdk.internal.reflect.MethodHandleIntegerFieldAccessorImpl.setInt(Object, int) line: 166` |
| 40 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1906` |
| 41 | 0.21% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 42 | 0.20% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 43 | 0.20% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 44 | 0.20% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2903` |
| 45 | 0.20% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 46 | 0.19% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 47 | 0.19% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 817` |
| 48 | 0.18% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 49 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 50 | 0.18% | `java.lang.invoke.MethodHandle.type() line: 470` |
| 51 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2475` |
| 52 | 0.16% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 53 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2341` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2430` |
| 55 | 0.15% | `java.lang.reflect.Field.getOverrideFieldAccessor() line: 1164` |
| 56 | 0.14% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 57 | 0.14% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 58 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2441` |
| 59 | 0.14% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 241` |
| 60 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2478` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.76% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 2 | 14.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 3 | 10.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 4.49% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2542` |
| 5 | 3.76% | `java.lang.reflect.Field.get(Object) line: 440` |
| 6 | 3.44% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 3.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2475` |
| 8 | 2.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 9 | 2.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1934` |
| 10 | 1.85% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 11 | 1.73% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 12 | 1.56% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 13 | 1.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 0.89% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 15 | 0.80% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 0.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1933` |
| 18 | 0.56% | `java.lang.reflect.Field.getInt(Object) line: 626` |
| 19 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 20 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3027` |
| 21 | 0.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 22 | 0.39% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 23 | 0.38% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 24 | 0.36% | `jdk.internal.reflect.MethodHandleObjectFieldAccessorImpl.get(Object) line: 57` |
| 25 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2441` |
| 26 | 0.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1036` |
| 27 | 0.31% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.get(Object) line: 55` |
| 28 | 0.30% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 434` |
| 29 | 0.26% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 684` |
| 30 | 0.24% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |

GC pauses: 0, total 0 ms

