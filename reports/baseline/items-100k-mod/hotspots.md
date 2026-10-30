# JVM hotspots (JFR ExecutionSample)

Samples: 6508

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4719` |
| 2 | 8.74% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8421` |
| 3 | 5.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8474` |
| 4 | 3.87% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 3.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5512` |
| 6 | 2.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.87% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 8 | 2.75% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3420` |
| 9 | 2.40% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 1.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5510` |
| 13 | 1.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2448` |
| 14 | 1.20% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 15 | 1.12% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 16 | 0.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4783` |
| 17 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 0.80% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3312` |
| 20 | 0.69% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4820` |
| 21 | 0.48% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3373` |
| 22 | 0.46% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8115` |
| 23 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3305` |
| 24 | 0.43% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 25 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3324` |
| 26 | 0.41% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1087` |
| 27 | 0.31% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 28 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8425` |
| 29 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 31 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2460` |
| 32 | 0.28% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 33 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 34 | 0.26% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 35 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8435` |
| 36 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 37 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 38 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4861` |
| 39 | 0.22% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 40 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3217` |
| 41 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3210` |
| 42 | 0.20% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 43 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 44 | 0.18% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 45 | 0.18% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 46 | 0.18% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 47 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3333` |
| 48 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4789` |
| 49 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 50 | 0.17% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 51 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3417` |
| 53 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 54 | 0.15% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 55 | 0.15% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 56 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 57 | 0.14% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 58 | 0.14% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2438` |
| 59 | 0.14% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8117` |
| 60 | 0.14% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2439` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.75% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8470` |
| 3 | 5.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 5.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.89% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 6 | 3.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3333` |
| 7 | 2.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5512` |
| 8 | 2.40% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1921` |
| 9 | 2.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3215` |
| 10 | 2.09% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2444` |
| 11 | 1.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 13 | 1.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8462` |
| 14 | 1.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 1.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4820` |
| 16 | 1.01% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2461` |
| 17 | 0.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.88% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2092` |
| 19 | 0.77% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8428` |
| 20 | 0.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3332` |
| 21 | 0.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 22 | 0.45% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.43% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity, Vec3) line: 1041` |
| 24 | 0.34% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 26 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8420` |
| 27 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3313` |
| 28 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3312` |
| 29 | 0.28% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 613` |
| 30 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

