# JVM hotspots (JFR ExecutionSample)

Samples: 5042

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7547` |
| 2 | 5.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8299` |
| 3 | 4.90% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 4.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 4.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.09% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 7 | 2.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.48% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12819` |
| 9 | 2.08% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13105` |
| 11 | 1.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5234` |
| 12 | 0.99% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13081` |
| 13 | 0.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5236` |
| 14 | 0.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 16 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.73% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 18 | 0.69% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7392` |
| 19 | 0.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.60% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 21 | 0.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7436` |
| 22 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7368` |
| 23 | 0.36% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 24 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 25 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7396` |
| 26 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 27 | 0.30% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5508` |
| 28 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 29 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 30 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 31 | 0.24% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 32 | 0.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8309` |
| 33 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 34 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5197` |
| 35 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 36 | 0.18% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 37 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 38 | 0.18% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 39 | 0.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 40 | 0.18% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2464` |
| 41 | 0.18% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2455` |
| 42 | 0.18% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 43 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 44 | 0.16% | `com.viaversion.viaversion.util.MinecraftKey.isValidPath(String) line: 92` |
| 45 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7231` |
| 46 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6893` |
| 47 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7230` |
| 48 | 0.16% | `java.util.ArrayList$Itr.next() line: 1058` |
| 49 | 0.14% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 50 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 51 | 0.14% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 52 | 0.14% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 53 | 0.14% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 54 | 0.14% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2459` |
| 55 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7374` |
| 56 | 0.14% | `java.lang.Double.isFinite(double) line: 1023` |
| 57 | 0.12% | `com.mojang.serialization.DataResult.success(Object, Lifecycle) line: 41` |
| 58 | 0.12% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 59 | 0.12% | `it.unimi.dsi.fastutil.ints.Int2IntOpenHashMap.find(int) line: 238` |
| 60 | 0.12% | `ru.planeta9091.cebaseline.BaselinePlugin.onTickEnd(ServerTickEndEvent) line: 565` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 22.21% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 6.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8309` |
| 3 | 5.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 5.63% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 5 | 4.88% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5771` |
| 6 | 4.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.80% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 2.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13196` |
| 9 | 2.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5159` |
| 10 | 2.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7378` |
| 11 | 1.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 0.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 15 | 0.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7374` |
| 16 | 0.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7396` |
| 17 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 19 | 0.36% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 20 | 0.34% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.34% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 22 | 0.34% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 23 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5245` |
| 25 | 0.26% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 26 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7450` |
| 27 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 29 | 0.24% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 30 | 0.20% | `net.minecraft.world.level.chunk.status.ChunkStatusTasks.lambda$buildTerrain$0(ChunkAccess) line: 130` |

GC pauses: 0, total 0 ms

