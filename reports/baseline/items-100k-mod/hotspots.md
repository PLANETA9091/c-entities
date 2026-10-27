# JVM hotspots (JFR ExecutionSample)

Samples: 4541

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7554` |
| 2 | 7.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3905` |
| 3 | 4.69% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.58% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.70% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7501` |
| 7 | 3.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2503` |
| 8 | 3.15% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2601` |
| 9 | 2.55% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.65% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.37% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2554` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3911` |
| 15 | 0.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4614` |
| 16 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 0.81% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 18 | 0.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3950` |
| 19 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.68% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 21 | 0.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4616` |
| 22 | 0.66% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7505` |
| 23 | 0.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3942` |
| 24 | 0.46% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7195` |
| 25 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 26 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7515` |
| 27 | 0.40% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7197` |
| 28 | 0.40% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 29 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3852` |
| 30 | 0.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1458` |
| 31 | 0.31% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2550` |
| 32 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 33 | 0.29% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.setTailPlain(MultiThreadedQueue$LinkedNode) line: 66` |
| 34 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3855` |
| 35 | 0.29% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 36 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 37 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2408` |
| 39 | 0.24% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 40 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 41 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1781` |
| 42 | 0.24% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 270` |
| 43 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 44 | 0.22% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 45 | 0.22% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2596` |
| 46 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7512` |
| 47 | 0.22% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 48 | 0.22% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 49 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 50 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 51 | 0.20% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 52 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3967` |
| 53 | 0.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7547` |
| 54 | 0.20% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 653` |
| 55 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3766` |
| 56 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 57 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 58 | 0.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2551` |
| 59 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3916` |
| 60 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.64% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.42% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7550` |
| 4 | 4.69% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 4.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3942` |
| 6 | 4.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4616` |
| 7 | 3.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2524` |
| 8 | 2.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2406` |
| 9 | 2.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7542` |
| 10 | 2.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2523` |
| 12 | 1.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2503` |
| 15 | 1.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 1.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 17 | 1.08% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 18 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7508` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7500` |
| 20 | 0.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2504` |
| 21 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.51% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 322` |
| 23 | 0.42% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1782` |
| 25 | 0.40% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.37% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 27 | 0.37% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 28 | 0.29% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.<init>() line: 92` |
| 29 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.26% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

