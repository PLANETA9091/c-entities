# JVM hotspots (JFR ExecutionSample)

Samples: 6299

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4719` |
| 2 | 12.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8474` |
| 3 | 5.97% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.98% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.18% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 6 | 2.98% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8421` |
| 7 | 2.62% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3420` |
| 8 | 1.64% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5510` |
| 10 | 1.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3312` |
| 11 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.00% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 13 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4783` |
| 14 | 0.90% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2448` |
| 16 | 0.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8435` |
| 19 | 0.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4820` |
| 20 | 0.62% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 21 | 0.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5512` |
| 22 | 0.41% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3373` |
| 23 | 0.37% | `java.lang.Integer.valueOf(int) line: 1006` |
| 24 | 0.35% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 25 | 0.35% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 26 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 27 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4601` |
| 28 | 0.29% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 29 | 0.25% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 30 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4602` |
| 31 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3305` |
| 32 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 33 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 34 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 35 | 0.22% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 36 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4828` |
| 37 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8425` |
| 38 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 39 | 0.21% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 40 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3318` |
| 41 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4732` |
| 42 | 0.19% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 43 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 44 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 45 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 46 | 0.17% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.17% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 48 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4861` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8115` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8429` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3375` |
| 52 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 53 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2460` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4734` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8467` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8404` |
| 58 | 0.14% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.getTailOpaque() line: 80` |
| 59 | 0.14% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3416` |
| 60 | 0.14% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3417` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.62% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.51% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.97% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8470` |
| 5 | 4.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8462` |
| 6 | 3.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3333` |
| 7 | 2.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5512` |
| 8 | 2.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2444` |
| 9 | 1.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3215` |
| 11 | 1.70% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1921` |
| 12 | 1.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4820` |
| 14 | 1.08% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.92% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2092` |
| 16 | 0.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3332` |
| 17 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.83% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2461` |
| 19 | 0.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3312` |
| 20 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8428` |
| 21 | 0.41% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.35% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 613` |
| 23 | 0.33% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.33% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 25 | 0.32% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.30% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 27 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 29 | 0.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 30 | 0.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8420` |

GC pauses: 0, total 0 ms

