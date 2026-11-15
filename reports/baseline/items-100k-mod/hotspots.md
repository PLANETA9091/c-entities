# JVM hotspots (JFR ExecutionSample)

Samples: 5559

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7583` |
| 2 | 7.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8335` |
| 3 | 5.05% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.46% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13141` |
| 8 | 2.86% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 9 | 2.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7454` |
| 10 | 2.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5234` |
| 12 | 1.48% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12855` |
| 13 | 1.01% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7406` |
| 16 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.59% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2464` |
| 18 | 0.54% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 19 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5236` |
| 20 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7382` |
| 22 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.34% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 24 | 0.32% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 25 | 0.29% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5508` |
| 26 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 27 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 28 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 29 | 0.23% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 30 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 31 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 32 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 33 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.20% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 35 | 0.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8345` |
| 36 | 0.18% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 37 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 38 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 39 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5197` |
| 40 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7033` |
| 41 | 0.16% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 42 | 0.16% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 43 | 0.14% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 44 | 0.14% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.compareExchangeNextVolatile(MultiThreadedQueue$LinkedNode, MultiThreadedQueue$LinkedNode) line: 1319` |
| 45 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7468` |
| 46 | 0.14% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 47 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7231` |
| 48 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5235` |
| 49 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7693` |
| 50 | 0.14% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 51 | 0.14% | `ru.planeta9091.cebaseline.BaselinePlugin.onTickEnd(ServerTickEndEvent) line: 565` |
| 52 | 0.14% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1430` |
| 53 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 54 | 0.13% | `java.math.BigInteger.montReduce(int[], int[], int, int) line: 3290` |
| 55 | 0.13% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 56 | 0.13% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 57 | 0.13% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 58 | 0.13% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.13% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 60 | 0.13% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.04% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 5.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8345` |
| 4 | 5.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.46% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5771` |
| 6 | 4.39% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 3.08% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 2.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7392` |
| 9 | 2.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5159` |
| 10 | 1.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13232` |
| 12 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 0.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7388` |
| 15 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 16 | 0.36% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 17 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 18 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7468` |
| 20 | 0.31% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 21 | 0.29% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5245` |
| 23 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 24 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.25% | `java.util.ArrayList.grow(int) line: 238` |
| 26 | 0.25% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13194` |
| 28 | 0.23% | `java.util.zip.ZipFile.getEntry(String) line: 294` |
| 29 | 0.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13245` |
| 30 | 0.23% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |

GC pauses: 0, total 0 ms

