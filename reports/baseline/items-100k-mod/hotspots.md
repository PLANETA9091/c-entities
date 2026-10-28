# JVM hotspots (JFR ExecutionSample)

Samples: 5892

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7819` |
| 2 | 10.69% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4161` |
| 3 | 6.31% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.82% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.43% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2856` |
| 7 | 3.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7766` |
| 8 | 1.73% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4879` |
| 11 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4167` |
| 12 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7780` |
| 13 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2758` |
| 15 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.85% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4198` |
| 18 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2018` |
| 19 | 0.66% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2809` |
| 20 | 0.63% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 21 | 0.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4881` |
| 22 | 0.46% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 23 | 0.44% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 24 | 0.41% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7462` |
| 25 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 26 | 0.32% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 27 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4206` |
| 28 | 0.32% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7460` |
| 29 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.31% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 31 | 0.29% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 32 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7770` |
| 33 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2002` |
| 34 | 0.27% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 35 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4022` |
| 36 | 0.25% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4160` |
| 38 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 39 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 40 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 41 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2751` |
| 42 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2663` |
| 43 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4108` |
| 44 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4109` |
| 45 | 0.20% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 46 | 0.20% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 47 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 48 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 49 | 0.20% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 50 | 0.19% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 51 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1999` |
| 52 | 0.19% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 53 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 54 | 0.17% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4166` |
| 56 | 0.17% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4021` |
| 58 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 59 | 0.15% | `java.util.Comparator.lambda$comparingLong$6043328a$1(ToLongFunction, Object, Object) line: 515` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 774` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.63% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.31% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7807` |
| 5 | 4.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4198` |
| 6 | 4.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7815` |
| 7 | 3.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2779` |
| 8 | 2.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4881` |
| 9 | 1.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2661` |
| 11 | 1.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1541` |
| 13 | 1.14% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.95% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2778` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 16 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.66% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 400` |
| 18 | 0.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2758` |
| 19 | 0.49% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7773` |
| 21 | 0.39% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.37% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 23 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2003` |
| 24 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 26 | 0.27% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 27 | 0.25% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.22% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

