# JVM hotspots (JFR ExecutionSample)

Samples: 4992

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10309` |
| 2 | 6.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9082` |
| 3 | 5.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.48% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 6 | 2.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6600` |
| 7 | 2.10% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 8 | 1.96% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14878` |
| 9 | 1.80% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 10 | 1.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8843` |
| 11 | 1.24% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 12 | 1.12% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1022` |
| 13 | 1.04% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14652` |
| 14 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.02% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 16 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 18 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 19 | 0.60% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 20 | 0.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10269` |
| 21 | 0.52% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 22 | 0.38% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 23 | 0.34% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 24 | 0.34% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 25 | 0.34% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 26 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 27 | 0.32% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3159` |
| 28 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 29 | 0.30% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3163` |
| 30 | 0.28% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 31 | 0.26% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 32 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 33 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 34 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 35 | 0.26% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6609` |
| 36 | 0.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15799` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3158` |
| 38 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 39 | 0.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 40 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8837` |
| 41 | 0.22% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 42 | 0.22% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3162` |
| 43 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 44 | 0.20% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 45 | 0.20% | `java.lang.Double.isFinite(double) line: 1023` |
| 46 | 0.20% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 48 | 0.18% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 49 | 0.18% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 50 | 0.18% | `java.util.ArrayList$Itr.next() line: 1058` |
| 51 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9263` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8817` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10268` |
| 55 | 0.18% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 56 | 0.16% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 57 | 0.16% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.setHeadPlain(MultiThreadedQueue$LinkedNode) line: 41` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8567` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3168` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6599` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.40% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 9.84% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 5.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 4.73% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 5 | 2.72% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6335` |
| 6 | 2.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 7 | 2.00% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1038` |
| 8 | 1.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15853` |
| 9 | 1.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.40% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 11 | 1.30% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14644` |
| 12 | 1.12% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 13 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8817` |
| 14 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8821` |
| 15 | 1.02% | `dev.planeta9091.centities.CeHooks.syncVelRead(Entity) line: 15447` |
| 16 | 0.96% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1049` |
| 17 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 19 | 0.64% | `dev.planeta9091.centities.CeHooks.itemDispatchWalk(ItemEntity, int) line: 6154` |
| 20 | 0.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10310` |
| 21 | 0.58% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 22 | 0.50% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 23 | 0.44% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.40% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.34% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 28 | 0.30% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 30 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |

GC pauses: 0, total 0 ms

