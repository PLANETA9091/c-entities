# JVM hotspots (JFR ExecutionSample)

Samples: 8638

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.86% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.get(VarHandle, Object) line: 92` |
| 2 | 10.38% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2972` |
| 3 | 8.86% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 4 | 8.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3099` |
| 5 | 5.83% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 3633` |
| 6 | 4.93% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2962` |
| 7 | 4.43% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 8 | 4.01% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 9 | 3.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 10 | 2.18% | `jdk.internal.reflect.MethodHandleObjectFieldAccessorImpl.get(Object) line: 57` |
| 11 | 1.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1965` |
| 12 | 1.93% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2062` |
| 14 | 1.02% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2995` |
| 15 | 0.98% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 16 | 0.96% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3002` |
| 17 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2507` |
| 18 | 0.76% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 19 | 0.72% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 20 | 0.66% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 21 | 0.52% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 23 | 0.46% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2016` |
| 24 | 0.44% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 25 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 26 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 27 | 0.37% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 433` |
| 28 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2218` |
| 30 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 31 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 32 | 0.25% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 614` |
| 33 | 0.23% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.get(Object) line: 55` |
| 34 | 0.23% | `java.lang.reflect.Field.get(Object) line: 440` |
| 35 | 0.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 36 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1403` |
| 37 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2552` |
| 38 | 0.20% | `jdk.internal.reflect.MethodHandleIntegerFieldAccessorImpl.getInt(Object) line: 79` |
| 39 | 0.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 40 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1384` |
| 41 | 0.16% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.getBoolean(Object) line: 63` |
| 42 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1958` |
| 43 | 0.15% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1467` |
| 44 | 0.15% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 45 | 0.15% | `java.lang.invoke.Invokers.checkCustomized(MethodHandle) line: 625` |
| 46 | 0.13% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 47 | 0.13% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 48 | 0.13% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2012` |
| 49 | 0.13% | `jdk.internal.reflect.MethodHandleIntegerFieldAccessorImpl.setInt(Object, int) line: 166` |
| 50 | 0.12% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 51 | 0.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 52 | 0.12% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 53 | 0.10% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 54 | 0.10% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2600` |
| 55 | 0.10% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 56 | 0.09% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 70` |
| 57 | 0.09% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1102` |
| 58 | 0.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2534` |
| 59 | 0.09% | `dev.planeta9091.centities.CeHooks$RefField.getInt(Object) line: 2197` |
| 60 | 0.09% | `jdk.internal.reflect.MethodHandleLongFieldAccessorImpl.getLong(Object) line: 83` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.47% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 2 | 13.19% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2218` |
| 3 | 8.86% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 4 | 8.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 5.90% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 6 | 4.42% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 7 | 4.02% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2972` |
| 8 | 3.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2534` |
| 9 | 2.50% | `java.lang.reflect.Field.get(Object) line: 440` |
| 10 | 2.49% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 11 | 2.14% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.88% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2601` |
| 13 | 1.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1986` |
| 14 | 0.96% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 3634` |
| 15 | 0.81% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 16 | 0.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1966` |
| 17 | 0.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1985` |
| 18 | 0.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 19 | 0.66% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 901` |
| 20 | 0.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 21 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 22 | 0.51% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 23 | 0.46% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1047` |
| 24 | 0.44% | `net.minecraft.util.Mth.packDegrees(float) line: 183` |
| 25 | 0.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 26 | 0.34% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 27 | 0.32% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 28 | 0.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 29 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 30 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |

GC pauses: 0, total 0 ms

