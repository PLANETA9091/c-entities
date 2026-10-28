# JVM hotspots (JFR ExecutionSample)

Samples: 5722

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 10.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 5.00% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.93% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 4.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.48% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 7 | 3.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 8 | 2.74% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 10 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 12 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.96% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 14 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 15 | 0.84% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 16 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.68% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 18 | 0.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 19 | 0.56% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4160` |
| 21 | 0.54% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 0.52% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7416` |
| 23 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 24 | 0.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1678` |
| 25 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 26 | 0.40% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 27 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7724` |
| 28 | 0.35% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 29 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 30 | 0.33% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 31 | 0.30% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 32 | 0.30% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.<init>(Object, MultiThreadedQueue$LinkedNode) line: 1250` |
| 33 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 34 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 35 | 0.26% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7414` |
| 36 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 37 | 0.24% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 38 | 0.23% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 761` |
| 39 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 40 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3976` |
| 41 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2706` |
| 42 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 43 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 44 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2725` |
| 46 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 47 | 0.19% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4063` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2618` |
| 51 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 52 | 0.17% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4064` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7728` |
| 55 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 56 | 0.16% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 57 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7731` |
| 59 | 0.16% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 60 | 0.16% | `java.lang.Double.isFinite(double) line: 1023` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.59% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 13.07% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 4 | 4.93% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 4.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 6 | 4.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 7 | 3.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 8 | 3.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 9 | 2.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 10 | 1.92% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.14% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 14 | 1.08% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 15 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 17 | 0.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2733` |
| 18 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7727` |
| 20 | 0.54% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 430` |
| 21 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.40% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.40% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 24 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2714` |
| 25 | 0.33% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 26 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 27 | 0.33% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.light(LightChunkGetter, ChunkAccess, Boolean[]) line: 914` |
| 30 | 0.30% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.<init>() line: 90` |

GC pauses: 0, total 0 ms

