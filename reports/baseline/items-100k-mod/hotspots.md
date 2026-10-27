# JVM hotspots (JFR ExecutionSample)

Samples: 2945

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 2 | 6.66% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.74% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 4 | 4.14% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 5 | 3.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 6 | 3.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2463` |
| 7 | 3.19% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 8 | 2.61% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 9 | 2.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2417` |
| 10 | 2.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2492` |
| 11 | 1.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3895` |
| 13 | 1.39% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 14 | 1.32% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.85% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7495` |
| 18 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 19 | 0.81% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 20 | 0.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2503` |
| 21 | 0.75% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2601` |
| 22 | 0.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 23 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7505` |
| 24 | 0.54% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 25 | 0.54% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 26 | 0.51% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 27 | 0.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4606` |
| 28 | 0.48% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 29 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7502` |
| 30 | 0.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1458` |
| 31 | 0.37% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 32 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2409` |
| 33 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3901` |
| 34 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 35 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 36 | 0.27% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 37 | 0.27% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 38 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3932` |
| 39 | 0.24% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 40 | 0.24% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 41 | 0.24% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 42 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2450` |
| 43 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2449` |
| 44 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 45 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 46 | 0.20% | `net.minecraft.server.MinecraftServer.runServer() line: 1351` |
| 47 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 48 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 49 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 50 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 51 | 0.17% | `java.util.Objects.hashCode(Object) line: 97` |
| 52 | 0.17% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 53 | 0.17% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 54 | 0.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 55 | 0.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7187` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7185` |
| 58 | 0.17% | `net.minecraft.world.level.chunk.LevelChunk.getListenerRegistry(int) line: 272` |
| 59 | 0.17% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 60 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2420` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.02% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 10.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4606` |
| 3 | 7.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7540` |
| 4 | 4.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2406` |
| 5 | 4.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3932` |
| 7 | 2.78% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.72% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7498` |
| 9 | 2.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7532` |
| 10 | 2.11% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2417` |
| 11 | 1.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.80% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 13 | 1.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 1.26% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7490` |
| 16 | 1.09% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2418` |
| 17 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2524` |
| 19 | 0.78% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 20 | 0.68% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 322` |
| 21 | 0.54% | `net.minecraft.server.MinecraftServer.lambda$spin$0(AtomicReference) line: 299` |
| 22 | 0.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2486` |
| 23 | 0.51% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 24 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 25 | 0.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2503` |
| 26 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 27 | 0.41% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 28 | 0.34% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.31% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |

GC pauses: 0, total 0 ms

