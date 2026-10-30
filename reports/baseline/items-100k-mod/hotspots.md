# JVM hotspots (JFR ExecutionSample)

Samples: 6010

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4838` |
| 2 | 11.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8593` |
| 3 | 5.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.81% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 3.31% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8540` |
| 6 | 3.11% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 379` |
| 7 | 2.76% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3539` |
| 8 | 2.66% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2525` |
| 10 | 1.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3431` |
| 11 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.01% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5629` |
| 13 | 0.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 14 | 0.78% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 15 | 0.78% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8554` |
| 16 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 0.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4902` |
| 18 | 0.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5631` |
| 19 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4939` |
| 20 | 0.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.57% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 0.53% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3492` |
| 23 | 0.42% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.37% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 25 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 26 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8544` |
| 27 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 28 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4721` |
| 29 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 30 | 0.28% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 31 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 32 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 34 | 0.27% | `java.util.HashMap.getNode(Object) line: 577` |
| 35 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 36 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 37 | 0.23% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 38 | 0.23% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 39 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3424` |
| 40 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4808` |
| 41 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 42 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4980` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4947` |
| 44 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4720` |
| 45 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 46 | 0.20% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 47 | 0.20% | `java.lang.Integer.valueOf(int) line: 1006` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4943` |
| 49 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 50 | 0.18% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.<init>(Object, MultiThreadedQueue$LinkedNode) line: 1249` |
| 51 | 0.18% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2516` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4851` |
| 54 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8234` |
| 56 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 57 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 58 | 0.17% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3487` |
| 59 | 0.15% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 60 | 0.15% | `java.lang.Class.cast(Object) line: 3491` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.30% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 4.81% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8589` |
| 5 | 4.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8581` |
| 6 | 3.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3452` |
| 7 | 2.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3334` |
| 8 | 2.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5631` |
| 9 | 2.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2521` |
| 10 | 2.01% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 1975` |
| 11 | 1.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4939` |
| 14 | 1.03% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3451` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2160` |
| 17 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3431` |
| 18 | 0.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8547` |
| 20 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2538` |
| 21 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 23 | 0.40% | `java.util.HashMap.get(Object) line: 565` |
| 24 | 0.37% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3432` |
| 26 | 0.35% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.32% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8539` |

GC pauses: 0, total 0 ms

