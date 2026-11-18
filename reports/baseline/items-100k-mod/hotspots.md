# JVM hotspots (JFR ExecutionSample)

Samples: 5461

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9870` |
| 2 | 6.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8727` |
| 3 | 5.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 5.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 4.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 6 | 3.50% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 984` |
| 7 | 2.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8513` |
| 8 | 1.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6044` |
| 9 | 1.61% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14400` |
| 10 | 1.39% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.19% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14207` |
| 12 | 1.17% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3009` |
| 13 | 1.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5998` |
| 14 | 0.99% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14975` |
| 16 | 0.88% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 954` |
| 17 | 0.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 0.79% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 19 | 0.73% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 983` |
| 20 | 0.71% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 985` |
| 21 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 22 | 0.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6007` |
| 23 | 0.55% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 982` |
| 24 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 25 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 26 | 0.48% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 994` |
| 27 | 0.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9871` |
| 28 | 0.46% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 968` |
| 29 | 0.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6040` |
| 30 | 0.44% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3005` |
| 31 | 0.42% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 32 | 0.42% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 987` |
| 33 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8509` |
| 34 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9862` |
| 35 | 0.37% | `java.util.ArrayList$Itr.next() line: 1058` |
| 36 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8491` |
| 37 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6046` |
| 38 | 0.37% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3004` |
| 39 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.31% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 41 | 0.31% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3014` |
| 42 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 43 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5934` |
| 44 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14989` |
| 45 | 0.24% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 46 | 0.24% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 47 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8244` |
| 48 | 0.22% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 49 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 50 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 51 | 0.22% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1030` |
| 52 | 0.20% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 990` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8485` |
| 54 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 55 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 56 | 0.16% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 57 | 0.16% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.compareExchangeNextVolatile(MultiThreadedQueue$LinkedNode, MultiThreadedQueue$LinkedNode) line: 1319` |
| 58 | 0.16% | `java.lang.Double.isFinite(double) line: 1023` |
| 59 | 0.16% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 60 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 11.35% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 6.90% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 968` |
| 4 | 5.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9871` |
| 6 | 4.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 7 | 2.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5946` |
| 8 | 1.78% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1028` |
| 9 | 1.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15323` |
| 10 | 1.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5975` |
| 11 | 1.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 12 | 1.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8491` |
| 13 | 1.25% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1000` |
| 14 | 1.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15285` |
| 15 | 1.19% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 994` |
| 16 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 1.03% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14199` |
| 18 | 0.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8495` |
| 19 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 21 | 0.49% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 989` |
| 22 | 0.40% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.40% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 24 | 0.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 25 | 0.37% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 163` |
| 26 | 0.37% | `dev.planeta9091.centities.CeHooks.mergeMaxStack(Object) line: 879` |
| 27 | 0.35% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8513` |
| 29 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 30 | 0.27% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |

GC pauses: 0, total 0 ms

