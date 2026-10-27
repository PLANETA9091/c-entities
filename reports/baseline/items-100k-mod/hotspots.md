# JVM hotspots (JFR ExecutionSample)

Samples: 3782

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.21% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 2 | 5.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2410` |
| 3 | 4.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 4 | 3.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 5 | 3.49% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 6 | 3.04% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 2.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2381` |
| 9 | 1.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2335` |
| 10 | 1.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 11 | 1.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.27% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3813` |
| 15 | 0.79% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 16 | 0.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4524` |
| 17 | 0.53% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 18 | 0.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 19 | 0.48% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2519` |
| 20 | 0.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2345` |
| 21 | 0.45% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 22 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 23 | 0.45% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 24 | 0.40% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 25 | 0.37% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 26 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7423` |
| 27 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2327` |
| 28 | 0.34% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7413` |
| 30 | 0.32% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 31 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 32 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2326` |
| 33 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2319` |
| 34 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 35 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3850` |
| 36 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3819` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2421` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 258` |
| 39 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 40 | 0.24% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |
| 41 | 0.24% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 42 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 43 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 44 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 45 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 954` |
| 46 | 0.21% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 47 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1699` |
| 48 | 0.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 49 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2339` |
| 50 | 0.19% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 51 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.lambda$setGenerationTask$0(ChunkProgressionTask, ChunkStatus, List, ChunkAccess, Throwable) line: 1616` |
| 52 | 0.19% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 53 | 0.19% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.setTailPlain(MultiThreadedQueue$LinkedNode) line: 66` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7392` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 640` |
| 56 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 57 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2367` |
| 58 | 0.16% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 59 | 0.16% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 60 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 765` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4524` |
| 2 | 11.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7458` |
| 3 | 4.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 4.52% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2324` |
| 6 | 3.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3850` |
| 7 | 2.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 8 | 2.41% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 10 | 1.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 12 | 1.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 13 | 1.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7450` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7416` |
| 15 | 0.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2442` |
| 16 | 0.61% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 17 | 0.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2335` |
| 18 | 0.53% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 19 | 0.50% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 20 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 21 | 0.45% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 22 | 0.45% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 23 | 0.45% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 24 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7408` |
| 25 | 0.37% | `net.minecraft.world.phys.shapes.Shapes.joinIsNotEmpty(VoxelShape, VoxelShape, BooleanOp) line: 227` |
| 26 | 0.37% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7420` |
| 28 | 0.26% | `net.minecraft.server.MinecraftServer.lambda$spin$0(AtomicReference) line: 299` |
| 29 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1700` |
| 30 | 0.26% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |

GC pauses: 0, total 0 ms

