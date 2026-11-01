# JVM hotspots (JFR ExecutionSample)

Samples: 6075

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 2 | 9.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5145` |
| 3 | 5.53% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 3.59% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 7 | 3.13% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 8 | 1.66% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 11 | 1.15% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.09% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 13 | 0.99% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 14 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.89% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8797` |
| 16 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5151` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 18 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5182` |
| 20 | 0.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5874` |
| 21 | 0.58% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.53% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3723` |
| 23 | 0.43% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8479` |
| 24 | 0.43% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 25 | 0.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2267` |
| 26 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5190` |
| 27 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 28 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 29 | 0.31% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 30 | 0.31% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 31 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3547` |
| 33 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2687` |
| 35 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5093` |
| 36 | 0.26% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 37 | 0.25% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 38 | 0.25% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 39 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 40 | 0.21% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.<init>(Object, MultiThreadedQueue$LinkedNode) line: 1250` |
| 41 | 0.21% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 393` |
| 42 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3545` |
| 44 | 0.21% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 45 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 46 | 0.20% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 47 | 0.20% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2688` |
| 49 | 0.20% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8477` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4964` |
| 51 | 0.18% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2694` |
| 53 | 0.18% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5050` |
| 55 | 0.16% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 56 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 57 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 58 | 0.16% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8787` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4963` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.06% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.53% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5182` |
| 5 | 4.54% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8832` |
| 6 | 4.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 7 | 3.90% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3676` |
| 8 | 3.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 9 | 2.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5874` |
| 10 | 2.02% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3545` |
| 12 | 1.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3674` |
| 15 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 17 | 0.66% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 18 | 0.56% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 762` |
| 19 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 20 | 0.49% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 21 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8790` |
| 22 | 0.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 23 | 0.40% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.36% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.31% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.26% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 28 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3644` |
| 29 | 0.23% | `java.util.ArrayList.grow(int) line: 238` |
| 30 | 0.23% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |

GC pauses: 0, total 0 ms

