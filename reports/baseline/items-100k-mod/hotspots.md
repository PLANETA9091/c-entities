# JVM hotspots (JFR ExecutionSample)

Samples: 5708

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7819` |
| 2 | 10.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4161` |
| 3 | 5.94% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.94% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 4.77% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7766` |
| 7 | 3.22% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2856` |
| 8 | 2.33% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2758` |
| 11 | 1.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4879` |
| 12 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.89% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 0.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4167` |
| 15 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4198` |
| 16 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.74% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7780` |
| 18 | 0.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4881` |
| 19 | 0.61% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7462` |
| 20 | 0.61% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 21 | 0.60% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 22 | 0.56% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 23 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4206` |
| 24 | 0.42% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7460` |
| 25 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2018` |
| 26 | 0.40% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2809` |
| 27 | 0.35% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 28 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7770` |
| 30 | 0.32% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 31 | 0.30% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2002` |
| 33 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 34 | 0.28% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 35 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 37 | 0.26% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.getTailOpaque() line: 80` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4107` |
| 39 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 40 | 0.23% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 41 | 0.23% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2805` |
| 42 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 43 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4109` |
| 45 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 46 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 47 | 0.19% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1999` |
| 49 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 50 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 51 | 0.18% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 52 | 0.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 774` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2663` |
| 55 | 0.18% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2751` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2770` |
| 58 | 0.16% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2764` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4230` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.68% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 13.09% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.96% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4198` |
| 5 | 4.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7815` |
| 6 | 3.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7807` |
| 7 | 3.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2779` |
| 8 | 2.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4881` |
| 9 | 2.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2661` |
| 10 | 1.98% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1541` |
| 15 | 0.96% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 0.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2778` |
| 17 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7773` |
| 19 | 0.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2003` |
| 20 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2758` |
| 21 | 0.42% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 400` |
| 22 | 0.39% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.39% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.39% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.37% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 26 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2759` |
| 27 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.28% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 29 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.26% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.appendList(MultiThreadedQueue$LinkedNode, MultiThreadedQueue$LinkedNode) line: 983` |

GC pauses: 0, total 0 ms

