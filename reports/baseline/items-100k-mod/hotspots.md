# JVM hotspots (JFR ExecutionSample)

Samples: 5150

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.12% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 9.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5435` |
| 3 | 5.17% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 3.55% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4028` |
| 5 | 2.83% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 6 | 2.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9335` |
| 7 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3901` |
| 8 | 1.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9330` |
| 9 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.22% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.01% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9329` |
| 13 | 0.93% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 14 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6213` |
| 16 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2712` |
| 17 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6203` |
| 19 | 0.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5512` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5441` |
| 21 | 0.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2750` |
| 22 | 0.64% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.58% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3981` |
| 24 | 0.41% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 25 | 0.41% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 26 | 0.35% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 27 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2703` |
| 28 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 29 | 0.33% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 30 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2702` |
| 31 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 33 | 0.31% | `ca.spottedleaf.concurrentutil.executor.thread.BalancedPrioritisedThreadPool$OrderedStreamGroup$Queue.createTask(Runnable, Priority, long, long) line: 464` |
| 34 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5497` |
| 35 | 0.29% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4025` |
| 36 | 0.27% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 37 | 0.27% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 38 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5340` |
| 39 | 0.25% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3983` |
| 40 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2695` |
| 41 | 0.23% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 42 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 43 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 44 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 45 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 46 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 47 | 0.21% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 48 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5434` |
| 49 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5254` |
| 50 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3894` |
| 51 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 52 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 53 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 54 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 55 | 0.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3907` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3913` |
| 58 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3805` |
| 59 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5428` |
| 60 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5341` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.08% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.72% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9334` |
| 3 | 5.17% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9437` |
| 5 | 4.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3934` |
| 6 | 3.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9427` |
| 7 | 3.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6213` |
| 8 | 3.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2094` |
| 9 | 2.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3803` |
| 10 | 2.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 2.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3932` |
| 14 | 1.11% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3901` |
| 15 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5497` |
| 16 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2725` |
| 18 | 0.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.43% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.43% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 21 | 0.41% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 22 | 0.41% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 23 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3902` |
| 24 | 0.39% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.39% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.35% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.31% | `ca.spottedleaf.concurrentutil.executor.thread.BalancedPrioritisedThreadPool$OrderedStreamGroup$Queue.createTask(Runnable, Priority) line: 459` |
| 29 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.29% | `net.minecraft.world.level.levelgen.synth.NoiseStack$SmearedPerlin.addToVolume(DensityBuffer, DensityVolume, double, double, float) line: 153` |

GC pauses: 0, total 0 ms

