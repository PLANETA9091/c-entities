# JVM hotspots (JFR ExecutionSample)

Samples: 6035

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8964` |
| 2 | 10.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5251` |
| 3 | 6.16% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 3.79% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3844` |
| 7 | 3.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8911` |
| 8 | 1.96% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 0.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3717` |
| 11 | 0.93% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 12 | 0.91% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8925` |
| 13 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.88% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5257` |
| 17 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 18 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5288` |
| 19 | 0.63% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 20 | 0.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5997` |
| 21 | 0.50% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8607` |
| 22 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5296` |
| 23 | 0.43% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3797` |
| 24 | 0.36% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 25 | 0.33% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8605` |
| 26 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2688` |
| 27 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8915` |
| 28 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 29 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 30 | 0.27% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 32 | 0.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5987` |
| 33 | 0.27% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 34 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5156` |
| 35 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 36 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 37 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 38 | 0.23% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 39 | 0.23% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 40 | 0.23% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 41 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 42 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 43 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 44 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5070` |
| 46 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 47 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2694` |
| 48 | 0.18% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3729` |
| 50 | 0.18% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5329` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3710` |
| 53 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 54 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 55 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5069` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3841` |
| 58 | 0.17% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3799` |
| 59 | 0.15% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 60 | 0.15% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.21% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.18% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8960` |
| 5 | 4.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5288` |
| 6 | 4.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8952` |
| 7 | 4.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3750` |
| 8 | 2.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 9 | 2.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5997` |
| 10 | 2.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3619` |
| 11 | 1.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.21% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.13% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3748` |
| 15 | 0.85% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 16 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 18 | 0.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3717` |
| 19 | 0.43% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 20 | 0.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.40% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.36% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8918` |
| 24 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.27% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 28 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.20% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |
| 30 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8911` |

GC pauses: 0, total 0 ms

