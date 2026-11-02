# JVM hotspots (JFR ExecutionSample)

Samples: 4927

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5546` |
| 2 | 5.72% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 3 | 5.24% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 3.98% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9625` |
| 6 | 3.25% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9632` |
| 7 | 3.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 1.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 9 | 1.70% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.66% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9585` |
| 11 | 1.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9588` |
| 12 | 1.52% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5555` |
| 14 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |
| 16 | 1.06% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 17 | 1.04% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 1.01% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2777` |
| 20 | 0.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6338` |
| 21 | 0.85% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4030` |
| 22 | 0.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5636` |
| 23 | 0.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6328` |
| 24 | 0.47% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 25 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 26 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9593` |
| 27 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5622` |
| 28 | 0.37% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 29 | 0.35% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 30 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 31 | 0.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9808` |
| 32 | 0.28% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 33 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 35 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 36 | 0.28% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5363` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 39 | 0.24% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 40 | 0.24% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 41 | 0.24% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5192` |
| 42 | 0.24% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 43 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5618` |
| 44 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 45 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 46 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 47 | 0.22% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 48 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2751` |
| 49 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5362` |
| 50 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 51 | 0.20% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 393` |
| 52 | 0.20% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 53 | 0.20% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4074` |
| 54 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 55 | 0.18% | `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock.unlock(ReentrantAreaLock$Node) line: 266` |
| 56 | 0.18% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5496` |
| 58 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 59 | 0.16% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2730` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.40% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 10.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9762` |
| 3 | 6.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3983` |
| 4 | 5.24% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 3.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3847` |
| 6 | 3.02% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2121` |
| 7 | 3.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6338` |
| 9 | 2.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.91% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 12 | 1.50% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 13 | 1.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3981` |
| 14 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 1.10% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5622` |
| 17 | 0.63% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 18 | 0.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 19 | 0.47% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.47% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 21 | 0.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3951` |
| 22 | 0.45% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.37% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.37% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 25 | 0.32% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 26 | 0.32% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 27 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.26% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 29 | 0.24% | `net.minecraft.world.level.levelgen.synth.NoiseStack$SmearedPerlin.addToVolume(DensityBuffer, DensityVolume, double, double, float) line: 153` |
| 30 | 0.24% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |

GC pauses: 0, total 0 ms

