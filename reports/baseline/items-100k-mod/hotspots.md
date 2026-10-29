# JVM hotspots (JFR ExecutionSample)

Samples: 5862

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7984` |
| 2 | 10.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4302` |
| 3 | 7.22% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.69% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 4.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7931` |
| 7 | 3.45% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2997` |
| 8 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 1.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5020` |
| 10 | 1.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2889` |
| 11 | 0.99% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 12 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.96% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7945` |
| 14 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.78% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 16 | 0.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4308` |
| 17 | 0.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4339` |
| 18 | 0.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2142` |
| 19 | 0.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5022` |
| 20 | 0.51% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 2950` |
| 21 | 0.48% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 22 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 23 | 0.38% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 24 | 0.38% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 25 | 0.36% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7627` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1815` |
| 27 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 28 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2882` |
| 30 | 0.29% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 31 | 0.29% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 32 | 0.24% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 33 | 0.24% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4347` |
| 35 | 0.24% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 36 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7935` |
| 37 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 38 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2895` |
| 39 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2126` |
| 40 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 41 | 0.20% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 42 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2901` |
| 43 | 0.20% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7625` |
| 44 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4252` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4249` |
| 46 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 47 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 48 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2123` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4371` |
| 51 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4250` |
| 52 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4163` |
| 53 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 54 | 0.15% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 55 | 0.15% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 56 | 0.15% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 57 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2794` |
| 59 | 0.14% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 60 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.57% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.19% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 7.23% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 6.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7980` |
| 5 | 5.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4339` |
| 6 | 4.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7972` |
| 7 | 3.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2910` |
| 8 | 2.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5022` |
| 9 | 2.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2792` |
| 13 | 1.14% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1665` |
| 14 | 0.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2909` |
| 15 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.70% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 17 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7938` |
| 18 | 0.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2127` |
| 19 | 0.49% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 518` |
| 20 | 0.48% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 505` |
| 21 | 0.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 22 | 0.41% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.41% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 24 | 0.36% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2889` |
| 27 | 0.29% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.29% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

