# JVM hotspots (JFR ExecutionSample)

Samples: 4387

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 11.65% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4001` |
| 3 | 6.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 4 | 2.89% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6418` |
| 5 | 2.85% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 2.48% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 7 | 2.26% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2075` |
| 9 | 1.85% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3488` |
| 11 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.91% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2213` |
| 15 | 0.80% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 0.78% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 17 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2029` |
| 18 | 0.57% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 19 | 0.50% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 20 | 0.43% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 21 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2104` |
| 22 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 23 | 0.41% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 24 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 25 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4041` |
| 27 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 28 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3494` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2115` |
| 30 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 32 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 33 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 34 | 0.27% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 35 | 0.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 36 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3525` |
| 37 | 0.27% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 38 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 40 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 41 | 0.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4199` |
| 42 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 43 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 44 | 0.21% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 45 | 0.21% | `com.mojang.serialization.Encoder$1.encode(Object, DynamicOps, Object) line: 25` |
| 46 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 47 | 0.21% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 48 | 0.21% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 49 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 50 | 0.18% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 51 | 0.18% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2061` |
| 53 | 0.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 54 | 0.16% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 55 | 0.16% | `java.lang.Class.cast(Object) line: 3491` |
| 56 | 0.16% | `java.util.AbstractCollection.isEmpty() line: 91` |
| 57 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 58 | 0.14% | `jdk.internal.foreign.HeapMemorySegmentImpl$OfByte.unsafeGetBase() line: 114` |
| 59 | 0.14% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.35% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 2 | 11.12% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6418` |
| 3 | 6.47% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 4.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4199` |
| 5 | 3.46% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 3.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2018` |
| 7 | 3.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 8 | 2.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 2.33% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6419` |
| 10 | 2.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 11 | 2.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3525` |
| 12 | 1.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2136` |
| 16 | 0.89% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2029` |
| 18 | 0.57% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 19 | 0.57% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 20 | 0.48% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 21 | 0.43% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.43% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 23 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 24 | 0.41% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 25 | 0.36% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4002` |
| 26 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2030` |
| 27 | 0.32% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 29 | 0.27% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 30 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

