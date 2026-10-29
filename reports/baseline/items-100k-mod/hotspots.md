# JVM hotspots (JFR ExecutionSample)

Samples: 5769

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8242` |
| 2 | 9.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4560` |
| 3 | 6.01% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.34% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8189` |
| 7 | 3.00% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3255` |
| 8 | 1.96% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5278` |
| 9 | 1.61% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.20% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.94% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8203` |
| 15 | 0.80% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2336` |
| 16 | 0.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4566` |
| 17 | 0.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3147` |
| 18 | 0.75% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2317` |
| 19 | 0.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4597` |
| 20 | 0.66% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 21 | 0.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5280` |
| 22 | 0.57% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3208` |
| 23 | 0.47% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7885` |
| 24 | 0.42% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1968` |
| 25 | 0.38% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 276` |
| 27 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4605` |
| 28 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 29 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 30 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8193` |
| 31 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4421` |
| 32 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 33 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2312` |
| 35 | 0.24% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 36 | 0.24% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 37 | 0.24% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4420` |
| 39 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 40 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 41 | 0.19% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 42 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 43 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 44 | 0.19% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7883` |
| 45 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3153` |
| 46 | 0.17% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 47 | 0.17% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 48 | 0.17% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 49 | 0.17% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3045` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3159` |
| 52 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 53 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2320` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3252` |
| 56 | 0.14% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 57 | 0.14% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2311` |
| 58 | 0.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 59 | 0.14% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 60 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3140` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.25% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.48% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.03% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8238` |
| 5 | 4.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4597` |
| 6 | 4.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8230` |
| 7 | 3.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3168` |
| 8 | 2.72% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 2.69% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1809` |
| 10 | 2.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5280` |
| 11 | 1.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3050` |
| 13 | 1.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3167` |
| 15 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2321` |
| 17 | 0.69% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 18 | 0.62% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 563` |
| 19 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3147` |
| 20 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8196` |
| 21 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 22 | 0.43% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.38% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 24 | 0.36% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 539` |
| 25 | 0.31% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8188` |
| 27 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 28 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.24% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 30 | 0.24% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |

GC pauses: 0, total 0 ms

