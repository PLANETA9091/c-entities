# JVM hotspots (JFR ExecutionSample)

Samples: 5262

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7583` |
| 2 | 4.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7454` |
| 3 | 4.20% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 4.18% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 8 | 2.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13141` |
| 9 | 1.82% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12855` |
| 10 | 1.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8335` |
| 11 | 1.20% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 12 | 1.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7406` |
| 16 | 0.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5234` |
| 17 | 0.97% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 18 | 0.84% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2464` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5236` |
| 20 | 0.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8345` |
| 21 | 0.48% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 22 | 0.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7410` |
| 23 | 0.38% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 24 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 25 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7380` |
| 26 | 0.32% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 28 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 29 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7382` |
| 30 | 0.27% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 31 | 0.27% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 32 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 33 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5197` |
| 34 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 35 | 0.23% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 36 | 0.21% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 37 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 38 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 39 | 0.19% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.19% | `java.lang.Class.cast(Object) line: 3491` |
| 41 | 0.19% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 42 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7296` |
| 43 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 44 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5147` |
| 45 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5226` |
| 46 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7693` |
| 47 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 48 | 0.17% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1863` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12854` |
| 50 | 0.17% | `java.util.ArrayList$Itr.next() line: 1058` |
| 51 | 0.15% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 52 | 0.15% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 53 | 0.15% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5238` |
| 55 | 0.13% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 56 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 57 | 0.13% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 58 | 0.13% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 59 | 0.13% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 60 | 0.13% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2459` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 21.21% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 6.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8345` |
| 3 | 4.50% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 4 | 4.20% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5771` |
| 5 | 3.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5159` |
| 6 | 3.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 3.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 2.47% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 2.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13232` |
| 10 | 1.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7388` |
| 12 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 0.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7392` |
| 15 | 0.74% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 16 | 0.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7468` |
| 17 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 18 | 0.44% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.40% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 21 | 0.32% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 23 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.27% | `java.util.ArrayList.grow(int) line: 238` |
| 25 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7410` |
| 26 | 0.25% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.25% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 28 | 0.23% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 29 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 30 | 0.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13245` |

GC pauses: 0, total 0 ms

